use crate::states::*;
use anchor_lang::prelude::*;

//Deactive Product
#[derive(Accounts)]
pub struct DeactivateProduct<'info> {
    #[account(mut, has_one = seller)]
    pub product: Account<'info, Product>,
    #[account(signer)]
    pub seller: Signer<'info>,
}

pub fn deactivate_product(ctx: Context<DeactivateProduct>) -> Result<()> {
    let product = &mut ctx.accounts.product;
    product.is_active = false;
    Ok(())
}
