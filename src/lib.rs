#![no_std]
#![cfg_attr(doc_cfg, feature(doc_cfg))]

extern crate alloc;

mod error;
#[cfg(feature = "hash")]
mod hash;
#[cfg(feature = "proof")]
mod proof;
pub mod types;

pub use ethereum_types::{Bloom, H160, H256, U256};

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
pub use proof::verify_proof;

#[cfg(feature = "hash")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hash")))]
pub use hash::keccak_256;
