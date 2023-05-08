use alloc::vec::Vec;

use bytes::Bytes;
use ethereum_types::{Bloom, H160, H256, H64, U256};

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-rlp", derive(rlp_derive::RlpEncodable))]
pub struct AxonHeader {
    pub prev_hash:                H256,
    pub proposer:                 H160,
    pub state_root:               H256,
    pub transactions_root:        H256,
    pub signed_txs_hash:          H256,
    pub receipts_root:            H256,
    pub log_bloom:                Bloom,
    pub difficulty:               U256,
    pub timestamp:                u64,
    pub number:                   u64,
    pub gas_used:                 U256,
    pub gas_limit:                U256,
    pub extra_data:               Bytes,
    pub mixed_hash:               Option<H256>,
    pub nonce:                    H64,
    pub base_fee_per_gas:         U256,
    pub proof:                    Proof,
    pub call_system_script_count: u32,
    pub chain_id:                 u64,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-rlp", derive(rlp_derive::RlpEncodable))]
pub struct AxonBlock {
    pub header:    AxonHeader,
    pub tx_hashes: Vec<H256>,
}

#[cfg(feature = "proof")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
#[cfg_attr(feature = "impl-rlp", derive(rlp_derive::RlpEncodable))]
pub struct Proposal {
    pub prev_hash:                H256,
    pub proposer:                 H160,
    pub prev_state_root:          H256,
    pub transactions_root:        H256,
    pub signed_txs_hash:          H256,
    pub timestamp:                u64,
    pub number:                   u64,
    pub gas_limit:                U256,
    pub extra_data:               Bytes,
    pub mixed_hash:               Option<H256>,
    pub base_fee_per_gas:         U256,
    pub proof:                    Proof,
    pub chain_id:                 u64,
    pub call_system_script_count: u32,
    pub tx_hashes:                Vec<H256>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "impl-rlp", derive(rlp_derive::RlpEncodable))]
pub struct Proof {
    pub number:     u64,
    pub round:      u64,
    pub block_hash: H256,
    pub signature:  Bytes,
    pub bitmap:     Bytes,
}

#[cfg(feature = "proof")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
pub struct Validator {
    pub bls_pub_key:    Bytes,
    pub address:        H160,
    pub propose_weight: u32,
    pub vote_weight:    u32,
}

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
impl core::cmp::PartialOrd for Validator {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(feature = "proof")]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
impl core::cmp::Ord for Validator {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.address.cmp(&other.address)
    }
}

#[cfg(feature = "proof")]
#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(doc_cfg, doc(cfg(feature = "proof")))]
#[cfg_attr(feature = "impl-rlp", derive(rlp_derive::RlpEncodable))]
pub struct Vote {
    pub height:     u64,
    pub round:      u64,
    pub vote_type:  u8,
    pub block_hash: Bytes,
}

#[cfg(test)]
impl Vote {
    fn random() -> Self {
        Self {
            height:     rand::random(),
            round:      rand::random(),
            vote_type:  2,
            block_hash: tests::random_bytes(32),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    pub fn random_bytes(len: usize) -> Bytes {
        (0..len).map(|_| rand::random()).collect::<Vec<u8>>().into()
    }

    #[test]
    fn test_vote_codec() {
        let vote = Vote::random();
        let raw = rlp::encode(&vote);
        let decoded: overlord::types::Vote = rlp::decode(&raw).unwrap();
        assert_eq!(vote.height, decoded.height);
        assert_eq!(vote.round, decoded.round);
        assert_eq!(vote.block_hash, decoded.block_hash);
    }
}
