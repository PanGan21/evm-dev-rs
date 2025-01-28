use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("execution halt")]
    Halt,
    #[error("stack underflow")]
    StackUnderflow,
}
