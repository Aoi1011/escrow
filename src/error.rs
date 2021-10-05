use thiserror::Error;

#[derive(Error, Debug, Copy, Clone)]
pub enum EscrowError {
    /// invalud instruction
    #[error("Invalid Instruction")]
    InvalidInstruction,
}
