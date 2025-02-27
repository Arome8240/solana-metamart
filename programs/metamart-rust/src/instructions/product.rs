use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct AddProduct<'info> {
    #[account(init, payer = seller, space = 8 + 150)]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub seller: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn add_product(
    ctx: Context<AddProduct>,
    title: String,
    description: String,
    price: u64,
    quantity: u64,
    image_cid: String,
) -> Result<()> {
    let product = &mut ctx.accounts.product;
    product.seller = *ctx.accounts.seller.key;
    product.title = title;
    product.description = description;
    product.price = price;
    product.quantity = quantity;
    product.amount_sold = 0;
    product.image_cid = image_cid;
    Ok(())
}
