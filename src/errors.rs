use thiserror::Error;

#[derive(Debug, Error)]
pub enum ExecutionError {
    #[error("execution halt")]
    Halt,
}
