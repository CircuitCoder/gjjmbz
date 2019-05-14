use gjjmbz::*;
use std::time::*;
use rand::Rng;

const KEY: [u8; 32] = [
    0x00,
    0x01,
    0x02,
    0x03,
    0x04,
    0x05,
    0x06,
    0x07,
    0x08,
    0x09,
    0x0a,
    0x0b,
    0x0c,
    0x0d,
    0x0e,
    0x0f,
    0x10,
    0x11,
    0x12,
    0x13,
    0x14,
    0x15,
    0x16,
    0x17,
    0x18,
    0x19,
    0x1a,
    0x1b,
    0x1c,
    0x1d,
    0x1e,
    0x1f,
];

#[derive(paw_structopt::StructOpt, structopt::StructOpt)]
struct Args {
    #[structopt(short="v", long="variant", raw(possible_values="&[\"128\", \"192\", \"256\"]"), default_value="256")]
    /// Which variant to use
    variant: usize,

    #[structopt(short="z", long="zero")]
    /// Use zeros instead of random values to test
    zero: bool,

    #[structopt(short="s", long="size", default_value="16384")]
    /// Total size in KiB
    size: usize,
}

#[paw::main]
fn main(args: Args) {
    let mut buf: Vec<u8> = vec![0; args.size * 1024];
    if !args.zero {
        rand::thread_rng().fill(buf.as_mut_slice());
    }

    let key = match args.variant {
        128 => &KEY[0..16],
        192=> &KEY[0..24],
        256=> &KEY[0..32],
        _ => unreachable!(),
    };

    let enc_cbc = cbc_from_key(key, [0; 16]).unwrap();
    let dec_cbc = cbc_from_key(key, [0; 16]).unwrap();

    let begin = Instant::now();
    let cipher = enc_cbc.encrypt(buf.iter().cloned());
    let enc = Instant::now();
    let plain = dec_cbc.decrypt(cipher.iter().cloned());
    let dec = Instant::now();

    let passed = plain == buf;
    if !passed {
        println!("Decrypted plaintext mismatch");
    }

    // Stat
    let enc_duration = enc - begin;
    let enc_nanos = enc_duration.subsec_nanos();
    let enc_secs = enc_duration.as_secs();
    let enc_secs_f = enc_secs as f64 + enc_nanos as f64 / 1000000000f64;

    let dec_duration = dec - enc;
    let dec_nanos = dec_duration.subsec_nanos();
    let dec_secs = dec_duration.as_secs();
    let dec_secs_f = dec_secs as f64 + dec_nanos as f64 / 1000000000f64;

    let tot_size_m = args.size as f64 / 1024f64;
    let enc_m_per_sec = tot_size_m / enc_secs_f;
    let dec_m_per_sec = tot_size_m / dec_secs_f;

    println!("Enc Time:");
    println!("{}.{:0>9}s", enc_secs, enc_nanos);

    println!("Enc Speed:");
    println!("{} MiB/s", enc_m_per_sec);

    println!("Dec Time:");
    println!("{}.{:0>9}s", dec_secs, dec_nanos);

    println!("Dec Speed:");
    println!("{} MiB/s", dec_m_per_sec);
}
