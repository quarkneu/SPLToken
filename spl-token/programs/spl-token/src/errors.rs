use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("You are not admin")]
    NotAdminError,
    #[msg("The amount token that you deposit should not be 0")]
    InvalidDepositTokenError,
    #[msg("The amount token that you withdraw is invalid")]
    InvalidWithdrawTokenAmountError,
}
