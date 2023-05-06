#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

extern crate alloc;

mod error;
mod mpt;
#[cfg(feature = "proof")]
mod proof;
pub mod types;

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
pub use proof::verify_proof;

#[cfg(feature = "hash")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hash")))]
pub fn keccak_256(data: &[u8]) -> [u8; 32] {
    use tiny_keccak::Hasher;

    let mut hasher = tiny_keccak::Keccak::v256();
    hasher.update(data);
    let mut output = [0u8; 32];
    hasher.finalize(&mut output);
    output
}
