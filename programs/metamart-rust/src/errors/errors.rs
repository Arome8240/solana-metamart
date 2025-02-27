use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("Insufficient balance for withdrawal.")]
    InsufficientBalance,
    #[msg("Product is out of stock.")]
    OutOfStock,
}
