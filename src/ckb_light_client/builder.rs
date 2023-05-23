use crate::ckb_light_client::image_cell_abi::{CellInfo, CellOutput, OutPoint, Script};
use crate::ckb_light_client::light_client_abi::Header;

#[derive(Default, Clone, Debug)]
pub struct CkbHeaderBuilder {
    version:           u32,
    compact_target:    u32,
    timestamp:         u64,
    number:            u64,
    epoch:             u64,
    parent_hash:       [u8; 32],
    transactions_root: [u8; 32],
    proposals_hash:    [u8; 32],
    extra_hash:        [u8; 32],
    dao:               [u8; 32],
    nonce:             u128,
    extension:         Vec<u8>,
    block_hash:        [u8; 32],
}

impl CkbHeaderBuilder {
    pub fn build(self) -> Header {
        Header {
            version:           self.version,
            compact_target:    self.compact_target,
            timestamp:         self.timestamp,
            number:            self.number,
            epoch:             self.epoch,
            parent_hash:       self.parent_hash,
            transactions_root: self.transactions_root,
            proposals_hash:    self.proposals_hash,
            extra_hash:        self.extra_hash,
            dao:               self.dao,
            nonce:             self.nonce,
            extension:         self.extension.into(),
            block_hash:        self.block_hash,
        }
    }

    pub fn version(mut self, version: u32) -> Self {
        self.version = version;
        self
    }

    pub fn compact_target(mut self, compact_target: u32) -> Self {
        self.compact_target = compact_target;
        self
    }

    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.timestamp = timestamp;
        self
    }

    pub fn number(mut self, number: u64) -> Self {
        self.number = number;
        self
    }

    pub fn epoch(mut self, epoch: u64) -> Self {
        self.epoch = epoch;
        self
    }

    pub fn parent_hash(mut self, parent_hash: [u8; 32]) -> Self {
        self.parent_hash = parent_hash;
        self
    }

    pub fn transactions_root(mut self, transactions_root: [u8; 32]) -> Self {
        self.transactions_root = transactions_root;
        self
    }

    pub fn proposals_hash(mut self, proposals_hash: [u8; 32]) -> Self {
        self.proposals_hash = proposals_hash;
        self
    }

    pub fn extra_hash(mut self, extra_hash: [u8; 32]) -> Self {
        self.extra_hash = extra_hash;
        self
    }

    pub fn dao(mut self, dao: [u8; 32]) -> Self {
        self.dao = dao;
        self
    }

    pub fn nonce(mut self, nonce: u128) -> Self {
        self.nonce = nonce;
        self
    }

    pub fn extension(mut self, extension: Vec<u8>) -> Self {
        self.extension = extension;
        self
    }

    pub fn block_hash(mut self, block_hash: [u8; 32]) -> Self {
        self.block_hash = block_hash;
        self
    }
}

#[derive(Default, Clone, Debug)]
pub struct OutPointBuilder {
    tx_hash: [u8; 32],
    index:   u32,
}

impl OutPointBuilder {
    pub fn build(self) -> OutPoint {
        OutPoint {
            tx_hash: self.tx_hash,
            index:   self.index,
        }
    }

    pub fn tx_hash(mut self, tx_hash: [u8; 32]) -> Self {
        self.tx_hash = tx_hash;
        self
    }

    pub fn index(mut self, index: u32) -> Self {
        self.index = index;
        self
    }
}

#[derive(Default, Clone, Debug)]
pub struct ScriptBuilder {
    code_hash: [u8; 32],
    hash_type: u8,
    args:      Vec<u8>,
}

impl ScriptBuilder {
    pub fn build(self) -> Script {
        Script {
            code_hash: self.code_hash,
            hash_type: self.hash_type,
            args:      self.args.into(),
        }
    }

    pub fn code_hash(mut self, code_hash: [u8; 32]) -> Self {
        self.code_hash = code_hash;
        self
    }

    pub fn hash_type(mut self, hash_type: u8) -> Self {
        self.hash_type = hash_type;
        self
    }

    pub fn args(mut self, args: Vec<u8>) -> Self {
        self.args = args;
        self
    }
}

#[derive(Default, Clone, Debug)]
pub struct CellOutputBuilder {
    capacity: u64,
    lock:     Script,
    type_:    Option<Script>,
}

impl CellOutputBuilder {
    pub fn build(self) -> CellOutput {
        CellOutput {
            capacity: self.capacity,
            lock:     self.lock,
            type_:    self.type_.map(|s| vec![s]).unwrap_or_default(),
        }
    }

    pub fn capacity(mut self, capacity: u64) -> Self {
        self.capacity = capacity;
        self
    }

    pub fn lock(mut self, lock: Script) -> Self {
        self.lock = lock;
        self
    }

    pub fn type_(mut self, type_: Option<Script>) -> Self {
        self.type_ = type_;
        self
    }
}

pub struct CellInfoBuilder {
    out_point: OutPoint,
    output:    CellOutput,
    data:      Vec<u8>,
}

impl CellInfoBuilder {
    pub fn build(self) -> CellInfo {
        CellInfo {
            out_point: self.out_point,
            output:    self.output,
            data:      self.data.into(),
        }
    }

    pub fn out_point(mut self, out_point: OutPoint) -> Self {
        self.out_point = out_point;
        self
    }

    pub fn output(mut self, output: CellOutput) -> Self {
        self.output = output;
        self
    }

    pub fn data(mut self, data: Vec<u8>) -> Self {
        self.data = data;
        self
    }
}
