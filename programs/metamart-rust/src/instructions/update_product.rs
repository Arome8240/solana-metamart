use crate::states::*;
use anchor_lang::prelude::*;

//Update Product
#[derive(Accounts)]
pub struct UpdateProduct<'info> {
    #[account(mut, has_one = seller)]
    pub product: Account<'info, Product>,
    #[account(signer)]
    pub seller: Signer<'info>,
}

pub fn update_product(
    ctx: Context<UpdateProduct>,
    name: String,
    description: String,
    quantity: u64,
    price: u64,
) -> Result<()> {
    let product = &mut ctx.accounts.product;
    product.title = name;
    product.description = description;
    product.quantity = quantity;
    product.price = price;
    Ok(())
}
