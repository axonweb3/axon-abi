use axon_tools::keccak_256;
use axon_tools::types::{MetadataVersion, NodePubKey, ValidatorExtend, H256};

use crate::image_cell_abi::{CellInfo, CellOutput, OutPoint, Script};
use crate::light_client_abi::Header;
use crate::metadata_abi::{self, CkbRelatedInfo, Metadata};

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

#[derive(Default, Clone, Debug)]
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
        let verifiers = self
            .verifier_list
            .iter()
            .map(|v| metadata_abi::ValidatorExtend {
                bls_pub_key:    v.bls_pub_key.clone().into(),
                pub_key:        v.pub_key.clone().into(),
                address:        v.address,
                propose_weight: v.propose_weight,
                vote_weight:    v.vote_weight,
            })
            .collect::<Vec<_>>();

        Metadata {
            version:         metadata_abi::MetadataVersion {
                start: self.version.start,
                end:   self.version.end,
            },
            epoch:           self.epoch,
            gas_limit:       self.gas_limit,
            gas_price:       self.gas_price,
            interval:        self.interval,
            verifier_list:   verifiers,
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
    metadata_type_id:     [u8; 32],
    checkpoint_type_id:   [u8; 32],
    xudt_args:            [u8; 32],
    stake_smt_type_id:    [u8; 32],
    delegate_smt_type_id: [u8; 32],
    reward_smt_type_id:   [u8; 32],
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

    pub fn metadata_type_id(mut self, metadata_type_id: [u8; 32]) -> Self {
        self.metadata_type_id = metadata_type_id;
        self
    }

    pub fn checkpoint_type_id(mut self, checkpoint_type_id: [u8; 32]) -> Self {
        self.checkpoint_type_id = checkpoint_type_id;
        self
    }

    pub fn xudt_args(mut self, xudt_args: [u8; 32]) -> Self {
        self.xudt_args = xudt_args;
        self
    }

    pub fn stake_smt_type_id(mut self, stake_smt_type_id: [u8; 32]) -> Self {
        self.stake_smt_type_id = stake_smt_type_id;
        self
    }

    pub fn delegate_smt_type_id(mut self, delegate_smt_type_id: [u8; 32]) -> Self {
        self.delegate_smt_type_id = delegate_smt_type_id;
        self
    }

    pub fn reward_smt_type_id(mut self, reward_smt_type_id: [u8; 32]) -> Self {
        self.reward_smt_type_id = reward_smt_type_id;
        self
    }
}
