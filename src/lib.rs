mod evm;
mod opcode;

use evm::Evm;
use primitive_types::U256;

pub struct EvmResult {
    pub stack: Vec<U256>,
    pub success: bool,
}

pub fn evm(_code: impl AsRef<[u8]>) -> EvmResult {
    let stack: Vec<U256> = Vec::new();

    let code = _code.as_ref();
    let mut evm = Evm::new(Box::from(code), stack);

    evm.execute();

    return EvmResult {
        stack: evm.stack,
        success: true,
    };
}
