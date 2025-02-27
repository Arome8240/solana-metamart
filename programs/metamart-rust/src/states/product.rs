use anchor_lang::prelude::*;

#[account]
pub struct Product {
    pub id: Pubkey,
    pub seller: Pubkey,
    pub title: String,
    pub description: String,
    pub price: u64,
    pub quantity: u64,
    pub amount_sold: u64,
    pub image_cid: String,
    pub is_active: bool,
}
