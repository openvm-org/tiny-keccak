#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use hex::FromHex;
use tiny_keccak::{Hasher, Keccak};

openvm::entry!(main);

fn main() {
    let input = Vec::from_hex("").unwrap();
    let expected =
        Vec::from_hex("C5D2460186F7233C927E7DB2DCC703C0E500B653CA82273B7BFAD8045D85A470").unwrap();

    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];

    hasher.update(&input);
    hasher.finalize(&mut output);

    if output[..] != expected[..] {
        panic!("Hash mismatch");
    }
}
