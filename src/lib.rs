const SBOX: [u8; 256] = [
    0x63,0x7c,0x77,0x7b,0xf2,0x6b,0x6f,0xc5,0x30,0x01,0x67,0x2b,0xfe,0xd7,0xab,0x76,
    0xca,0x82,0xc9,0x7d,0xfa,0x59,0x47,0xf0,0xad,0xd4,0xa2,0xaf,0x9c,0xa4,0x72,0xc0,
    0xb7,0xfd,0x93,0x26,0x36,0x3f,0xf7,0xcc,0x34,0xa5,0xe5,0xf1,0x71,0xd8,0x31,0x15,
    0x04,0xc7,0x23,0xc3,0x18,0x96,0x05,0x9a,0x07,0x12,0x80,0xe2,0xeb,0x27,0xb2,0x75,
    0x09,0x83,0x2c,0x1a,0x1b,0x6e,0x5a,0xa0,0x52,0x3b,0xd6,0xb3,0x29,0xe3,0x2f,0x84,
    0x53,0xd1,0x00,0xed,0x20,0xfc,0xb1,0x5b,0x6a,0xcb,0xbe,0x39,0x4a,0x4c,0x58,0xcf,
    0xd0,0xef,0xaa,0xfb,0x43,0x4d,0x33,0x85,0x45,0xf9,0x02,0x7f,0x50,0x3c,0x9f,0xa8,
    0x51,0xa3,0x40,0x8f,0x92,0x9d,0x38,0xf5,0xbc,0xb6,0xda,0x21,0x10,0xff,0xf3,0xd2,
    0xcd,0x0c,0x13,0xec,0x5f,0x97,0x44,0x17,0xc4,0xa7,0x7e,0x3d,0x64,0x5d,0x19,0x73,
    0x60,0x81,0x4f,0xdc,0x22,0x2a,0x90,0x88,0x46,0xee,0xb8,0x14,0xde,0x5e,0x0b,0xdb,
    0xe0,0x32,0x3a,0x0a,0x49,0x06,0x24,0x5c,0xc2,0xd3,0xac,0x62,0x91,0x95,0xe4,0x79,
    0xe7,0xc8,0x37,0x6d,0x8d,0xd5,0x4e,0xa9,0x6c,0x56,0xf4,0xea,0x65,0x7a,0xae,0x08,
    0xba,0x78,0x25,0x2e,0x1c,0xa6,0xb4,0xc6,0xe8,0xdd,0x74,0x1f,0x4b,0xbd,0x8b,0x8a,
    0x70,0x3e,0xb5,0x66,0x48,0x03,0xf6,0x0e,0x61,0x35,0x57,0xb9,0x86,0xc1,0x1d,0x9e,
    0xe1,0xf8,0x98,0x11,0x69,0xd9,0x8e,0x94,0x9b,0x1e,0x87,0xe9,0xce,0x55,0x28,0xdf,
    0x8c,0xa1,0x89,0x0d,0xbf,0xe6,0x42,0x68,0x41,0x99,0x2d,0x0f,0xb0,0x54,0xbb,0x16
];

const INVSBOX: [u8; 256] = [
    0x52,0x09,0x6a,0xd5,0x30,0x36,0xa5,0x38,0xbf,0x40,0xa3,0x9e,0x81,0xf3,0xd7,0xfb,
    0x7c,0xe3,0x39,0x82,0x9b,0x2f,0xff,0x87,0x34,0x8e,0x43,0x44,0xc4,0xde,0xe9,0xcb,
    0x54,0x7b,0x94,0x32,0xa6,0xc2,0x23,0x3d,0xee,0x4c,0x95,0x0b,0x42,0xfa,0xc3,0x4e,
    0x08,0x2e,0xa1,0x66,0x28,0xd9,0x24,0xb2,0x76,0x5b,0xa2,0x49,0x6d,0x8b,0xd1,0x25,
    0x72,0xf8,0xf6,0x64,0x86,0x68,0x98,0x16,0xd4,0xa4,0x5c,0xcc,0x5d,0x65,0xb6,0x92,
    0x6c,0x70,0x48,0x50,0xfd,0xed,0xb9,0xda,0x5e,0x15,0x46,0x57,0xa7,0x8d,0x9d,0x84,
    0x90,0xd8,0xab,0x00,0x8c,0xbc,0xd3,0x0a,0xf7,0xe4,0x58,0x05,0xb8,0xb3,0x45,0x06,
    0xd0,0x2c,0x1e,0x8f,0xca,0x3f,0x0f,0x02,0xc1,0xaf,0xbd,0x03,0x01,0x13,0x8a,0x6b,
    0x3a,0x91,0x11,0x41,0x4f,0x67,0xdc,0xea,0x97,0xf2,0xcf,0xce,0xf0,0xb4,0xe6,0x73,
    0x96,0xac,0x74,0x22,0xe7,0xad,0x35,0x85,0xe2,0xf9,0x37,0xe8,0x1c,0x75,0xdf,0x6e,
    0x47,0xf1,0x1a,0x71,0x1d,0x29,0xc5,0x89,0x6f,0xb7,0x62,0x0e,0xaa,0x18,0xbe,0x1b,
    0xfc,0x56,0x3e,0x4b,0xc6,0xd2,0x79,0x20,0x9a,0xdb,0xc0,0xfe,0x78,0xcd,0x5a,0xf4,
    0x1f,0xdd,0xa8,0x33,0x88,0x07,0xc7,0x31,0xb1,0x12,0x10,0x59,0x27,0x80,0xec,0x5f,
    0x60,0x51,0x7f,0xa9,0x19,0xb5,0x4a,0x0d,0x2d,0xe5,0x7a,0x9f,0x93,0xc9,0x9c,0xef,
    0xa0,0xe0,0x3b,0x4d,0xae,0x2a,0xf5,0xb0,0xc8,0xeb,0xbb,0x3c,0x83,0x53,0x99,0x61,
    0x17,0x2b,0x04,0x7e,0xba,0x77,0xd6,0x26,0xe1,0x69,0x14,0x63,0x55,0x21,0x0c,0x7d
];

const RNDC: [u32; 11] = [
    0x00, // Ignore first slot
    0x01, 0x02, 0x04, 0x08, 0x10,
    0x20, 0x40, 0x80, 0x1B, 0x36,
];

const COLMAP: [usize; 16] = [
    0, 4, 8, 12,
    1, 5, 9, 13,
    2, 6, 10, 14,
    3, 7, 11, 15,
];

fn gdouble(input: u8) -> u8 {
    let hi_bit = input & 0x80;
    if hi_bit != 0 {
        (input << 1) ^ 0x1B
    } else {
        input << 1
    }
}

pub trait GJJMBlock {
    fn encrypt(&self, state: &mut [u8; 16]);
    fn decrypt(&self, state: &mut [u8; 16]);
}

impl<T: GJJMBlock + ?Sized> GJJMBlock for Box<T> {
    fn encrypt(&self, state: &mut [u8; 16]) {
        (**self).encrypt(state);
    }
    fn decrypt(&self, state: &mut [u8; 16]) {
        (**self).decrypt(state);
    }
}

macro_rules! gjjmbz_impl {
    ($name:ident, $KEY_SIZE:expr, $ROUNDS:expr) => {
        pub struct $name {
            key_segs: [u32; 4 * $ROUNDS + 4],
        }

        impl $name {
            const _KEY_BYTES: usize = $KEY_SIZE / 8;
            const N: usize = $KEY_SIZE / 32;
            pub fn new(key: &[u8]) -> Self {
                let mut result = Self {
                    key_segs: unsafe { std::mem::uninitialized() },
                };

                unsafe { std::ptr::copy_nonoverlapping(std::mem::transmute(&key[0]), &mut result.key_segs[0] as *mut u32, Self::N); }

                let mut last = result.key_segs[Self::N-1];

                for i in Self::N..(4 * $ROUNDS + 4) {
                    last = if i % Self::N == 0 {
                        result.key_segs[i-Self::N] ^ Self::sub_word(last.rotate_right(8)) ^ RNDC[i / Self::N]
                    } else if Self::N > 6 && i % Self::N == 4 {
                        result.key_segs[i-Self::N] ^ Self::sub_word(last)
                    } else {
                        result.key_segs[i-Self::N] ^ last
                    };

                    result.key_segs[i] = last;
                }

                result
            }

            fn sub_word(w: u32) -> u32 {
                (SBOX[(w>>24) as usize] as u32) << 24
                    | (SBOX[((w>>16)&0xff) as usize] as u32) << 16
                    | (SBOX[((w>>8)&0xff) as usize] as u32) << 8
                    | SBOX[(w&0xff) as usize] as u32
            }

            pub fn encrypt(&self, _state: &mut [u8; 16]) {
                let mut state: [u8; 16] = unsafe { std::mem::uninitialized() };
                for i in 0..16 {
                    state[i] = _state[COLMAP[i]];
                }

                self.add_round_key(&mut state, 0);

                for i in 1..$ROUNDS {
                    // TODO: merge sub_bytes, shift_rows and mix_columns
                    Self::sub_bytes(&mut state);
                    Self::shift_rows(&mut state);
                    Self::mix_columns(&mut state);
                    self.add_round_key(&mut state, i);
                }

                Self::sub_bytes(&mut state);
                Self::shift_rows(&mut state);
                self.add_round_key(&mut state, $ROUNDS);

                for i in 0..16 {
                    _state[i] = state[COLMAP[i]];
                }
            }

            pub fn decrypt(&self, _state: &mut [u8; 16]) {
                let mut state: [u8; 16] = unsafe { std::mem::uninitialized() };
                for i in 0..16 {
                    state[i] = _state[COLMAP[i]];
                }

                self.add_round_key(&mut state, $ROUNDS);
                for i in 1..$ROUNDS {
                    // TODO: merge sub_bytes, shift_rows and mix_columns
                    Self::inv_shift_rows(&mut state);
                    Self::inv_sub_bytes(&mut state);
                    self.add_round_key(&mut state, $ROUNDS - i);
                    Self::inv_mix_columns(&mut state);
                }

                Self::inv_shift_rows(&mut state);
                Self::inv_sub_bytes(&mut state);
                self.add_round_key(&mut state, 0);

                for i in 0..16 {
                    _state[i] = state[COLMAP[i]];
                }
            }

            pub fn print(state: &[u8; 16]) {
                for i in 0..4 {
                    for j in 0..4 {
                        print!("{:0>2x} ", state[i*4+j]);
                    }
                    println!("");
                }
                println!("");
            }

            pub fn sub_bytes(state: &mut [u8; 16]) {
                for i in 0..16 {
                    state[i] = SBOX[state[i] as usize];
                }
            }

            fn inv_sub_bytes(state: &mut [u8; 16]) {
                for i in 0..16 {
                    state[i] = INVSBOX[state[i] as usize];
                }
            }

            pub fn shift_rows(state: &mut [u8; 16]) {
                state[4..8].rotate_left(1);
                state[8..12].rotate_left(2);
                state[12..16].rotate_left(3);
            }

            fn inv_shift_rows(state: &mut [u8; 16]) {
                state[4..8].rotate_right(1);
                state[8..12].rotate_right(2);
                state[12..16].rotate_right(3);
            }

            pub fn mix_columns(state: &mut [u8; 16]) {
                let mut doubles: [u8; 4] = unsafe { std::mem::uninitialized() };
                let mut copy: [u8; 4] = unsafe { std::mem::uninitialized() };
                for col in 0..4 {
                    for row in 0..4 {
                        copy[row] = state[row * 4 + col] ;
                        doubles[row] = gdouble(copy[row]);
                    }

                    for row in 0..4 {
                        state[row * 4 + col] = doubles[row]
                            ^ doubles[(row + 1) % 4]
                            ^ copy[(row + 1) % 4]
                            ^ copy[(row + 2) % 4]
                            ^ copy[(row + 3) % 4]
                    }
                }
            }

            fn inv_mix_columns(state: &mut [u8; 16]) {
                let mut octas: [u8; 4] = unsafe { std::mem::uninitialized() };
                let mut quads: [u8; 4] = unsafe { std::mem::uninitialized() };
                let mut doubles: [u8; 4] = unsafe { std::mem::uninitialized() };
                let mut copy: [u8; 4] = unsafe { std::mem::uninitialized() };
                for col in 0..4 {
                    for row in 0..4 {
                        copy[row] = state[row * 4 + col] ;
                        doubles[row] = gdouble(copy[row]);
                        quads[row] = gdouble(doubles[row]);
                        octas[row] = gdouble(quads[row]);
                    }

                    for row in 0..4 {
                        state[row * 4 + col] = octas[row] ^ quads[row] ^ doubles[row]
                            ^ octas[(row + 1) % 4] ^ doubles[(row + 1) % 4] ^ copy[(row + 1) % 4]
                            ^ octas[(row + 2) % 4] ^ quads[(row + 2) % 4] ^ copy[(row + 2) % 4]
                            ^ octas[(row + 3) % 4] ^ copy[(row + 3) % 4];
                    }
                }
            }

            pub fn add_round_key(&self, state: &mut [u8; 16], rnd: usize) {
                let key = self.derive_key(rnd);

                for i in 0..16 {
                    state[i] ^= key[i];
                }
            }

            fn derive_key(&self, rnd: usize) -> [u8; 16] {
                let mut result: [u8; 16] = unsafe { std::mem::uninitialized() };

                // Assumes little endian
                for i in 0..16 {
                    result[i] = (self.key_segs[rnd * 4 + i % 4] >> (i / 4 * 8)) as u8;
                }

                result
            }
        }

        impl GJJMBlock for $name {
            fn encrypt(&self, state: &mut [u8; 16]) {
                self.encrypt(state);
            }
            fn decrypt(&self, state: &mut [u8; 16]) {
                self.decrypt(state);
            }
        }
    }
}

gjjmbz_impl!(GJJMBlock128, 128, 10);
gjjmbz_impl!(GJJMBlock192, 192, 12);
gjjmbz_impl!(GJJMBlock256, 256, 14);

pub struct GJJMCBC<T: GJJMBlock> {
    block: T,
    state: [u8; 16],
}

pub fn cbc_from_key(key: &[u8], iv: [u8; 16]) -> Option<GJJMCBC<impl GJJMBlock>> {
    let block: Box<GJJMBlock> = match key.len() {
        16 => Box::new(GJJMBlock128::new(key)),
        24 => Box::new(GJJMBlock192::new(key)),
        32 => Box::new(GJJMBlock256::new(key)),
        _ => return None,
    };

    Some(GJJMCBC::new(block, iv))
}

impl<T: GJJMBlock> GJJMCBC<T> {
    pub fn new(block: T, iv: [u8; 16]) -> Self {
        Self {
            block,
            state: iv,
        }
    }

    pub fn encrypt<I: Iterator<Item=u8>>(mut self, mut data: I) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        loop {
            for i in 0..16 {
                match data.next() {
                    Some(d) => { self.state[i] ^= d; },
                    None => {
                        // Full block
                        if i == 0 {
                            return result;
                        }

                        // Pad
                        for j in i..16 {
                            self.state[j] = 0;
                        }

                        self.block.encrypt(&mut self.state);
                        result.extend_from_slice(&self.state);
                        return result;
                    }
                }
            }

            // Block filled
            self.block.encrypt(&mut self.state);
            result.extend_from_slice(&self.state);
        }
    }

    pub fn decrypt<I: Iterator<Item=u8>>(mut self, mut data: I) -> Vec<u8> {
        let mut result: Vec<u8> = Vec::new();
        let mut current: [u8; 16] = [0; 16];
        let mut next_state: [u8; 16];

        loop {
            for i in 0..16 {
                match data.next() {
                    Some(d) => { current[i] = d; },
                    None => {
                        // Can only happen on block start
                        return result;
                    }
                }
            }

            // Block filled
            next_state = current;
            self.block.decrypt(&mut current);
            for i in 0..16 {
                current[i] ^= self.state[i];
            }
            result.extend_from_slice(&current);
            self.state = next_state;
        }
    }
}
