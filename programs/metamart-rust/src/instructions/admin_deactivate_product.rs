use crate::states::*;
use anchor_lang::prelude::*;

//Admin Deactivate Product
#[derive(Accounts)]
pub struct AdminDeactivateProduct<'info> {
    #[account(mut)]
    pub product: Account<'info, Product>,
    #[account(signer)]
    pub admin: Signer<'info>,
}

pub fn admin_deactivate_product(ctx: Context<AdminDeactivateProduct>) -> Result<()> {
    let product = &mut ctx.accounts.product;
    product.is_active = false;
    Ok(())
}
