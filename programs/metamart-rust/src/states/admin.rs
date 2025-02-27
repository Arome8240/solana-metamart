use anchor_lang::prelude::*;

#[account]
pub struct Admin {
    pub owner: Pubkey,
    pub service_fee: u64,
}
