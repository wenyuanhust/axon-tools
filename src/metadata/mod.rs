#[cfg(feature = "abi")]
pub(crate) mod abi;

use std::{prelude::v1::*, vec};

use ethereum_types::H256;

use crate::{
    keccak_256,
    types::{CkbRelatedInfo, Metadata, MetadataVersion, NodePubKey, ProposeCount, ValidatorExtend},
};

#[derive(Clone)]
pub struct MetadataBuilder {
    version:         MetadataVersion,
    epoch:           u64,
    gas_limit:       u64,
    gas_price:       u64,
    interval:        u64,
    verifier_list:   Vec<ValidatorExtend>,
    propose_ratio:   u64,
    prevote_ratio:   u64,
    precommit_ratio: u64,
    brake_ratio:     u64,
    tx_num_limit:    u64,
    max_tx_size:     u64,
}

impl Default for MetadataBuilder {
    fn default() -> Self {
        MetadataBuilder {
            version:         MetadataVersion {
                start: 1,
                end:   100,
            },
            epoch:           0,
            gas_limit:       4294967295000,
            gas_price:       1,
            interval:        3000,
            verifier_list:   vec![],
            propose_ratio:   15,
            prevote_ratio:   10,
            precommit_ratio: 10,
            brake_ratio:     10,
            tx_num_limit:    2000,
            max_tx_size:     409600000,
        }
    }
}

impl MetadataBuilder {
    pub fn build(self) -> Metadata {
        Metadata {
            version:         self.version,
            epoch:           self.epoch,
            gas_limit:       self.gas_limit,
            gas_price:       self.gas_price,
            interval:        self.interval,
            verifier_list:   self.verifier_list,
            propose_ratio:   self.propose_ratio,
            prevote_ratio:   self.prevote_ratio,
            precommit_ratio: self.precommit_ratio,
            brake_ratio:     self.brake_ratio,
            tx_num_limit:    self.tx_num_limit,
            max_tx_size:     self.max_tx_size,
            propose_counter: vec![],
        }
    }

    pub fn version(mut self, start: u64, end: u64) -> Self {
        self.version = MetadataVersion { start, end };
        self
    }

    pub fn epoch(mut self, epoch: u64) -> Self {
        self.epoch = epoch;
        self
    }

    pub fn gas_limit(mut self, gas_limit: u64) -> Self {
        self.gas_limit = gas_limit;
        self
    }

    pub fn gas_price(mut self, gas_price: u64) -> Self {
        self.gas_price = gas_price;
        self
    }

    pub fn interval(mut self, interval: u64) -> Self {
        self.interval = interval;
        self
    }

    pub fn verifier_list(mut self, public_keys: Vec<NodePubKey>) -> Self {
        self.verifier_list = public_keys
            .iter()
            .map(|pk| ValidatorExtend {
                bls_pub_key:    pk.bls_pub_key.clone(),
                pub_key:        pk.pub_key.clone(),
                address:        H256(keccak_256(&pk.pub_key)).into(),
                propose_weight: 1,
                vote_weight:    1,
            })
            .collect();

        self
    }

    pub fn propose_ratio(mut self, propose_ratio: u64) -> Self {
        self.propose_ratio = propose_ratio;
        self
    }

    pub fn prevote_ratio(mut self, prevote_ratio: u64) -> Self {
        self.prevote_ratio = prevote_ratio;
        self
    }

    pub fn precommit_ratio(mut self, precommit_ratio: u64) -> Self {
        self.precommit_ratio = precommit_ratio;
        self
    }

    pub fn brake_ratio(mut self, brake_ratio: u64) -> Self {
        self.brake_ratio = brake_ratio;
        self
    }

    pub fn tx_num_limit(mut self, tx_num_limit: u64) -> Self {
        self.tx_num_limit = tx_num_limit;
        self
    }

    pub fn max_tx_size(mut self, max_tx_size: u64) -> Self {
        self.max_tx_size = max_tx_size;
        self
    }
}

#[derive(Default)]
pub struct CkbRelatedInfoBuilder {
    metadata_type_id:     H256,
    checkpoint_type_id:   H256,
    xudt_args:            H256,
    stake_smt_type_id:    H256,
    delegate_smt_type_id: H256,
    reward_smt_type_id:   H256,
}

impl CkbRelatedInfoBuilder {
    pub fn build(self) -> CkbRelatedInfo {
        CkbRelatedInfo {
            metadata_type_id:     self.metadata_type_id,
            checkpoint_type_id:   self.checkpoint_type_id,
            xudt_args:            self.xudt_args,
            stake_smt_type_id:    self.stake_smt_type_id,
            delegate_smt_type_id: self.delegate_smt_type_id,
            reward_smt_type_id:   self.reward_smt_type_id,
        }
    }

    pub fn metadata_type_id(mut self, metadata_type_id: H256) -> Self {
        self.metadata_type_id = metadata_type_id;
        self
    }

    pub fn checkpoint_type_id(mut self, checkpoint_type_id: H256) -> Self {
        self.checkpoint_type_id = checkpoint_type_id;
        self
    }

    pub fn xudt_args(mut self, xudt_args: H256) -> Self {
        self.xudt_args = xudt_args;
        self
    }

    pub fn stake_smt_type_id(mut self, stake_smt_type_id: H256) -> Self {
        self.stake_smt_type_id = stake_smt_type_id;
        self
    }

    pub fn delegate_smt_type_id(mut self, delegate_smt_type_id: H256) -> Self {
        self.delegate_smt_type_id = delegate_smt_type_id;
        self
    }

    pub fn reward_smt_type_id(mut self, reward_smt_type_id: H256) -> Self {
        self.reward_smt_type_id = reward_smt_type_id;
        self
    }
}

impl From<Metadata> for abi::Metadata {
    fn from(value: Metadata) -> Self {
        abi::Metadata {
            version:         value.version.into(),
            epoch:           value.epoch,
            gas_limit:       value.gas_limit,
            gas_price:       value.gas_price,
            interval:        value.interval,
            verifier_list:   value.verifier_list.into_iter().map(Into::into).collect(),
            propose_ratio:   value.propose_ratio,
            prevote_ratio:   value.prevote_ratio,
            precommit_ratio: value.precommit_ratio,
            brake_ratio:     value.brake_ratio,
            tx_num_limit:    value.tx_num_limit,
            max_tx_size:     value.max_tx_size,
            propose_counter: value.propose_counter.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<MetadataVersion> for abi::MetadataVersion {
    fn from(version: MetadataVersion) -> Self {
        abi::MetadataVersion {
            start: version.start,
            end:   version.end,
        }
    }
}

impl From<ValidatorExtend> for abi::ValidatorExtend {
    fn from(value: ValidatorExtend) -> Self {
        abi::ValidatorExtend {
            bls_pub_key:    value.bls_pub_key.into(),
            pub_key:        value.pub_key.into(),
            address:        value.address,
            propose_weight: value.propose_weight,
            vote_weight:    value.vote_weight,
        }
    }
}

impl From<ProposeCount> for abi::ProposeCount {
    fn from(pc: ProposeCount) -> Self {
        abi::ProposeCount {
            address: pc.address,
            count:   pc.count,
        }
    }
}

impl From<CkbRelatedInfo> for abi::CkbRelatedInfo {
    fn from(value: CkbRelatedInfo) -> Self {
        abi::CkbRelatedInfo {
            metadata_type_id:     value.metadata_type_id.into(),
            checkpoint_type_id:   value.checkpoint_type_id.into(),
            xudt_args:            value.xudt_args.into(),
            stake_smt_type_id:    value.stake_smt_type_id.into(),
            delegate_smt_type_id: value.delegate_smt_type_id.into(),
            reward_smt_type_id:   value.reward_smt_type_id.into(),
        }
    }
}
