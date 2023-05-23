#![cfg_attr(doc_cfg, feature(doc_cfg))]

extern crate alloc;

#[cfg(feature = "ckb-light-client")]
pub mod ckb_light_client;
mod error;
#[cfg(feature = "hash")]
mod hash;
#[cfg(feature = "metadata")]
mod metadata;
#[cfg(feature = "proof")]
mod proof;
pub mod types;

pub use error::Error;

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
pub use proof::verify_proof;

#[cfg(feature = "hash")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "hash")))]
pub use hash::keccak_256;

#[cfg(feature = "metadata")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "metadata")))]
pub use metadata::{CkbRelatedInfoBuilder, MetadataBuilder};

pub mod consts;
