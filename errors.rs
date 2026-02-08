use anchor_lang::prelude::*;

#[error_code]
pub enum MaintainerError {
    #[msg("Unauthorized")]
    Unauthorized,
}
