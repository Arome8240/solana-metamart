use anchor_lang::prelude::*;

#[account]
pub struct Wallet {
    pub owner: Pubkey,
    pub balance: u64,
    pub total_withdrawn: u64,
}
