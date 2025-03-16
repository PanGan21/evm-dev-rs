use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("invalid opcode")]
    InvalidOpcode,
    #[error("invalid jump destination")]
    InvalidJumpDestination,
    #[error("execution halt")]
    Halt,
    #[error("stack underflow")]
    StackUnderflow,
    #[error("integer underflow")]
    IntegerOverflow,
    #[error("revert opcode")]
    Revert,
    #[error("execution is read only")]
    ReadOnly,
    #[error("contract address already exists")]
    ContractAddressCollision,
}
