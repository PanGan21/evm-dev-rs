mod block;
mod errors;
mod evm;
mod jumpdest;
mod memory;
mod opcode;
mod tx;
mod utils;

use crate::tx::TxData;
use block::BlockData;
use evm::Evm;
use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(
    _code: impl AsRef<[u8]>,
    _tx_data: Vec<Vec<u8>>,
    _block_data: Vec<Vec<u8>>,
) -> EvmResult {
    let stack: Vec<U256> = Vec::new();

    let code = _code.as_ref();
    let mut evm = Evm::new(
        Box::from(code),
        stack,
        TxData::new(_tx_data),
        BlockData::new(_block_data),
    );

    let result = evm.execute();
    match result {
        evm::ExecutionResult::Success => EvmResult {
            stack: evm.stack(),
            success: true,
        },
        evm::ExecutionResult::Halt => EvmResult {
            stack: evm.stack(),
            success: true,
        },
        evm::ExecutionResult::Revert => EvmResult {
            stack: evm.stack(),
            success: false,
        },
    }
}
