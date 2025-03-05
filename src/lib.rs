mod block;
mod errors;
mod evm;
mod jumpdest;
mod log;
mod memory;
mod opcode;
mod state;
mod storage;
mod tx;
mod utils;

use std::collections::HashMap;

use crate::tx::TxData;
use block::BlockData;
use evm::Evm;
use primitive_types::U256;
use state::State;
use storage::Storage;

pub use log::Log;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
    pub logs: Vec<Log>,
}

pub fn evm(
    _code: impl AsRef<[u8]>,
    _tx_data: Vec<Vec<u8>>,
    _block_data: Vec<Vec<u8>>,
    _state_data: HashMap<Vec<u8>, (usize, Vec<u8>, Vec<u8>)>,
) -> EvmResult {
    let stack: Vec<U256> = Vec::new();

    let code = _code.as_ref();
    let mut evm = Evm::new(
        Box::from(code),
        stack,
        TxData::new(_tx_data),
        BlockData::new(_block_data),
        State::new(_state_data),
        Storage::new(),
        vec![],
    );

    let result = evm.execute();
    match result {
        evm::ExecutionResult::Success => EvmResult {
            stack: evm.stack(),
            success: true,
            logs: evm.logs,
        },
        evm::ExecutionResult::Halt => EvmResult {
            stack: evm.stack(),
            success: true,
            logs: evm.logs,
        },
        evm::ExecutionResult::Revert => EvmResult {
            stack: evm.stack(),
            success: false,
            logs: evm.logs,
        },
    }
}
