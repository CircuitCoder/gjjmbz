Gao Ji Jia Mi Biao Zhun
> a.k.a. AES

实现了 AES-{128,192,256}-CBC，可以正确加密，解密。

## 正确性

使用以下方式运行可以输出单个分组加密、解密的结果：
```bash
# Encrypt
$ cargo run --release --bin single -- e54b04099c6c16ba14a0e25f4fb68dd4 --variant 128
# 90ad55ba79384012f1ce8663498190f4

# Decrypt
$ cargo run --release --bin single -- 90ad55ba79384012f1ce8663498190f4 --decrypt --variant 128
# e54b04099c6c16ba14a0e25f4fb68dd4
```

密钥为 `0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f` 的前缀

最终在 AES-128, AES-192, AES-256 得到的密文分别是：
- **AES-128**: 90ad55ba79384012f1ce8663498190f4
- **AES-192**: 0eed72e41ddae736fe1eb442cab439a9
- **AES-256**: 382e059150e21d9baaaebb8c57187678

## 速度

使用以下方式运行可以测量加密、解密的速度

```bash
# Specify size (in KiB, defaults to 16384)
$ cargo run --release --bin test -- --variant 128 --size 10240

# Use zero instead of random numbers
$ cargo run --release --bin test -- --variant 128 --zero
```

消息长度为 16384 KiB，密钥为 `0x000102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f` 的前缀时，使用三个不同的算法，输出如下：

```bash
# cargo run --release --bin test --quiet -- --variant 128
Enc Time:
0.497568400s
Enc Speed:
32.156382921423464 MiB/s
Dec Time:
0.598562300s
Dec Speed:
26.73071792192726 MiB/s

# cargo run --release --bin test --quiet -- --variant 192
Enc Time:
0.579310600s
Enc Speed:
27.619035453520098 MiB/s
Dec Time:
0.723190800s
Dec Speed:
22.124175252229424 MiB/s

# cargo run --release --bin test --quiet -- --variant 256
Time:
0.697646900s
Enc Speed:
22.934237936124994 MiB/s
Dec Time:
0.845464700s
Dec Speed:
18.92450388525979 MiB/s
```

额外的，短消息的输出如下：
```bash
# cargo run --release --bin test --quiet -- --variant 256 --size 16
Enc Time:
0.000575500s
Enc Speed:
27.150304083405736 MiB/s
Dec Time:
0.000812500s
Dec Speed:
19.23076923076923 MiB/s
```

使用 `openssl` 测试结果如下:

```bash
$ openssl test aes-256-cbc
Doing aes-256 cbc for 3s on 16 size blocks: 14920187 aes-256 cbc's in 2.86s
Doing aes-256 cbc for 3s on 64 size blocks: 4074134 aes-256 cbc's in 2.95s
Doing aes-256 cbc for 3s on 256 size blocks: 998602 aes-256 cbc's in 2.78s
Doing aes-256 cbc for 3s on 1024 size blocks: 257799 aes-256 cbc's in 2.91s
Doing aes-256 cbc for 3s on 8192 size blocks: 32679 aes-256 cbc's in 2.92s
Doing aes-256 cbc for 3s on 16384 size blocks: 15630 aes-256 cbc's in 2.82s
OpenSSL 1.1.1b  26 Feb 2019
built on: Tue Feb 26 16:11:02 2019 UTC
options:bn(64,64) rc4(16x,int) des(int) aes(partial) idea(int) blowfish(ptr)
compiler: gcc -fPIC -pthread -m64 -Wa,--noexecstack -Wall -O3 -Wa,--noexecstack -D_FORTIFY_SOURCE=2 -march=x86-64 -mtune=generic -O2 -pipe -fstack-protector-strong -fno-plt -Wl,-O1,--sort-common,--as-needed,-z,relro,-z,now -DOPENSSL_USE_NODELETE -DL_ENDIAN -DOPENSSL_PIC -DOPENSSL_CPUID_OBJ -DOPENSSL_IA32_SSE2 -DOPENSSL_BN_ASM_MONT -DOPENSSL_BN_ASM_MONT5 -DOPENSSL_BN_ASM_GF2m -DSHA1_ASM -DSHA256_ASM -DSHA512_ASM -DKECCAK1600_ASM -DRC4_ASM -DMD5_ASM -DAES_ASM -DVPAES_ASM -DBSAES_ASM -DGHASH_ASM -DECP_NISTZ256_ASM -DX25519_ASM -DPADLOCK_ASM -DPOLY1305_ASM -DNDEBUG
The 'numbers' are in 1000s of bytes per second processed.
type             16 bytes     64 bytes    256 bytes   1024 bytes   8192 bytes  16384 bytes
aes-256 cbc      83469.58k    88387.99k    91957.59k    90716.90k    91680.26k    90809.19k
```

可以看到，OpenSSL 在 16KiB 大小的消息上可以达到 90MB/s 的速度，是我的实现的三倍速度。这件事情很奇怪，可能原因有两个：
- OpenSSL 使用了部分处理器上包含的硬件 AES
- 我的代码写的菜，或者 Rustc 并没有成功把循环优化成 SSE

对 Debug 编译生成的文件进行 Profile, Profiler 输出如下：

```
$ cargo profiler callgrind --release --bin ./target/debug/test

Profiling test with callgrind...

Total Instructions...68,477,432

7,476,484 (10.9%) range.rs:core::iter::range
-----------------------------------------------------------------------
5,928,535 (8.7%) ???:memcpy@GLIBC_2.2.5
-----------------------------------------------------------------------
4,969,378 (7.3%) ptr.rs:core::ptr
-----------------------------------------------------------------------
4,934,754 (7.2%) intrinsics.rs:core::intrinsics
-----------------------------------------------------------------------
4,622,592 (6.8%) lib.rs:gjjmbz::GJJMBlock256
-----------------------------------------------------------------------
4,168,528 (6.1%) mod.rs:core::num
-----------------------------------------------------------------------
3,527,216 (5.2%) mod.rs:core::num
```

可以看到有 10% 左右的时间用于构造 Rust 的 for in 语法糖结构，至少说明在 Debug 模式下循环并没有被转换成类 C 循环，更没有被展开或者编译成 SSE。

Memcpy 所消耗的长时间是因为 Rust 为了保证生命周期完整，对象经常在不同的变量绑定之间移动。Debug 模式下每次移动会对应一次内存拷贝，可能在 Release 模式内 LLVM 会根据生命周期，将部分函数内联，合并掉一些拷贝。

由于 Release 模式导出的符号很少，所以 Profile 看不到有意义的结果。猜测可能是编译器的优化力度不足。

## 测试环境
Windows 10 1903(190426), WSL

```bash
$ lscpu
Architecture:        x86_64
CPU op-mode(s):      32-bit, 64-bit
Byte Order:          Little Endian
Address sizes:       36 bits physical, 48 bits virtual
CPU(s):              4
On-line CPU(s) list: 0-3
Thread(s) per core:  1
Core(s) per socket:  4
Socket(s):           1
Vendor ID:           GenuineIntel
CPU family:          6
Model:               58
Model name:          Intel(R) Core(TM) i5-3470 CPU @ 3.20GHz
Stepping:            9
CPU MHz:             3201.000
CPU max MHz:         3201.0000
BogoMIPS:            6402.00
Hypervisor vendor:   Windows Subsystem for Linux
Virtualization type: container
Flags:               fpu vme de pse tsc msr pae mce cx8 apic sep mtrr pge mca cmov pat pse36 clflush acpi mmx fxsr sse sse2 ss ht tm pbe syscall nx rdtscp lm pni pclmulqdq est tm2 ssse3 cx16 xtpr pdcm pcid sse4_1 sse4_2 popcnt aes xsave osxsave avx f16c rdrand hypervisor lahf_lm fsgsbase smep erms ibrs ibpb stibp ssbd
```
