pub mod builder;
mod image_cell_abi;
mod light_client_abi;
mod metadata_abi;

pub mod address;

pub use image_cell_abi::{
    BlockRollback as CellBlockRollback, BlockUpdate as CellBlockUpdate, CellInfo, OutPoint,
    RollbackCall as CellRollbackCall, UpdateCall as CellUpdateCall,
};
pub use light_client_abi::{
    Header, RollbackCall as HeaderRollbackCall, SetStateCall, UpdateCall as HeaderUpdateCall,
};

use ethers::abi::AbiEncode;

use crate::image_cell_abi::ImageCellContractCalls;
use crate::light_client_abi::CkbLightClientContractCalls;

impl CellUpdateCall {
    pub fn new(cells: Vec<CellBlockUpdate>) -> Self {
        Self { blocks: cells }
    }

    pub fn abi_encode(self) -> Vec<u8> {
        ImageCellContractCalls::Update(self).encode()
    }
}

impl CellRollbackCall {
    pub fn new(cells: Vec<CellBlockRollback>) -> Self {
        Self { blocks: cells }
    }

    pub fn abi_encode(self) -> Vec<u8> {
        ImageCellContractCalls::Rollback(self).encode()
    }
}

impl CellBlockUpdate {
    pub fn new(block_number: u64, inputs: Vec<OutPoint>, outputs: Vec<CellInfo>) -> Self {
        Self {
            block_number,
            tx_inputs: inputs,
            tx_outputs: outputs,
        }
    }
}

impl CellBlockRollback {
    pub fn new(inputs: Vec<OutPoint>, outputs: Vec<OutPoint>) -> Self {
        Self {
            tx_inputs:  inputs,
            tx_outputs: outputs,
        }
    }
}

impl HeaderUpdateCall {
    pub fn new(headers: Vec<Header>) -> Self {
        Self { headers }
    }

    pub fn abi_encode(self) -> Vec<u8> {
        CkbLightClientContractCalls::Update(self).encode()
    }
}

impl HeaderRollbackCall {
    pub fn new(hashes: Vec<[u8; 32]>) -> Self {
        Self {
            block_hashes: hashes,
        }
    }

    pub fn abi_encode(self) -> Vec<u8> {
        CkbLightClientContractCalls::Rollback(self).encode()
    }
}

impl SetStateCall {
    pub fn new(allow_read: bool) -> Self {
        Self { allow_read }
    }

    pub fn abi_encode(self) -> Vec<u8> {
        CkbLightClientContractCalls::SetState(self).encode()
    }
}

impl From<ckb_jsonrpc_types::Script> for image_cell_abi::Script {
    fn from(value: ckb_jsonrpc_types::Script) -> Self {
        image_cell_abi::Script {
            code_hash: value.code_hash.0,
            hash_type: value.hash_type as u8,
            args:      value.args.into_bytes().into(),
        }
    }
}
