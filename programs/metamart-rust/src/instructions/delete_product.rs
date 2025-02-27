use crate::states::*;
use anchor_lang::prelude::*;

//Delete Product
#[derive(Accounts)]
pub struct DeleteProduct<'info> {
    #[account(mut, close = seller, has_one = seller)]
    pub product: Account<'info, Product>,
    #[account(signer)]
    pub seller: Signer<'info>,
}

pub fn delete_product(ctx: Context<DeleteProduct>) -> Result<()> {
    Ok(())
}
