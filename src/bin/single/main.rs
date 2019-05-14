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

    #[structopt(short="d", long="decrypt")]
    /// Decrypt mode
    decrypt: bool,

    #[structopt(name="text")]
    /// Ciphertext/Plaintext
    text: String,
}

#[paw::main]
fn main(args: Args) {
    let mut buf = hex::decode(args.text).unwrap();
    let mut array = [0; 16];
    array.copy_from_slice(&buf[..16]);

    let key = match args.variant {
        128 => &KEY[0..16],
        192=> &KEY[0..24],
        256=> &KEY[0..32],
        _ => unreachable!(),
    };

    let block = block_from_key(key).unwrap();

    if args.decrypt {
        block.decrypt(&mut array);
    } else {
        block.encrypt(&mut array);
    }

    println!("{}", hex::encode(array));
}
