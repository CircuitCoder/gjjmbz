use gjjmbz::*;

const INPUT: [u8; 16] = [
    0xe5,
    0x4b,
    0x04,
    0x09,
    0x9c,
    0x6c,
    0x16,
    0xba,
    0x14,
    0xa0,
    0xe2,
    0x5f,
    0x4f,
    0xb6,
    0x8d,
    0xd4,
];

const KEY: [u8; 16] = [
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
];

fn main() {
    let b = AESBlock::new(KEY);

    let mut block = INPUT;
    b.encrypt(&mut block);

    for c in block.iter() {
        print!("{:0>2x}", c);
    }
    println!("");

    b.decrypt(&mut block);

    for c in block.iter() {
        print!("{:0>2x}", c);
    }
    println!("");
}
