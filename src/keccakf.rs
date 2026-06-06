use crate::{Buffer, Permutation};

const ROUNDS: usize = 24;

const RC: [u64; ROUNDS] = [
    1u64,
    0x8082u64,
    0x800000000000808au64,
    0x8000000080008000u64,
    0x808bu64,
    0x80000001u64,
    0x8000000080008081u64,
    0x8000000000008009u64,
    0x8au64,
    0x88u64,
    0x80008009u64,
    0x8000000au64,
    0x8000808bu64,
    0x800000000000008bu64,
    0x8000000000008089u64,
    0x8000000000008003u64,
    0x8000000000008002u64,
    0x8000000000000080u64,
    0x800au64,
    0x800000008000000au64,
    0x8000000080008081u64,
    0x8000000000008080u64,
    0x80000001u64,
    0x8000000080008008u64,
];

keccak_function!("`keccak-f[1600, 24]`", keccakf, ROUNDS, RC);

#[cfg(all(target_os = "zkvm", target_endian = "big"))]
compile_error!("the native keccakf path assumes the zkvm target is little-endian");

pub struct KeccakF;

impl Permutation for KeccakF {
    #[cfg(not(target_os = "zkvm"))]
    fn execute(buffer: &mut Buffer) {
        keccakf(buffer.words());
    }

    #[cfg(target_os = "zkvm")]
    fn execute(buffer: &mut Buffer) {
        // The native instruction reads the state as 200 raw bytes. This
        // matches the layout of `[u64; 25]` only on a little-endian target,
        // which the compile guard above enforces.
        // SAFETY: `buffer.words()` is `[u64; 25]` = 200 bytes, as required.
        unsafe {
            openvm_keccak256_guest::native_keccakf(buffer.words().as_mut_ptr() as *mut u8);
        }
    }
}
