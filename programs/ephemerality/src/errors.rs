use crate::*;

#[error_code]
pub enum EphemeralityError {
    #[msg("Invalid calculation")]
    InvalidCalculation,
    #[msg("Destroy timestamp has not been exceeded")]
    DestroyTimestampNotExceeded
}
