use anchor_lang::prelude::*;

#[account]
pub struct Order {
    pub buyer: Pubkey,
    pub product: Pubkey,
    pub amount_paid: u64,
    pub status: String,
}
