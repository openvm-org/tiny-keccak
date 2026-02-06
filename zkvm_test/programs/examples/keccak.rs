#![no_main]
#![no_std]

extern crate alloc;

use alloc::vec::Vec;
use hex::FromHex;
use tiny_keccak::{Hasher, Keccak};

openvm::entry!(main);

fn main() {
    let input = Vec::from_hex("E926AE8B0AF6E53176DBFFCC2A6B88C6BD765F939D3D178A9BDE9EF3AA131C61E31C1E42CDFAF4B4DCDE579A37E150EFBEF5555B4C1CB40439D835A724E2FAE7").unwrap();
    let expected =
        Vec::from_hex("574271CD13959E8DDEAE5BFBDB02A3FDF54F2BABFD0CBEB893082A974957D0C1").unwrap();

    let mut hasher = Keccak::v256();
    let mut output = [0u8; 32];

    hasher.update(&input);
    hasher.finalize(&mut output);

    if output[..] != expected[..] {
        panic!("Hash mismatch");
    }
}
