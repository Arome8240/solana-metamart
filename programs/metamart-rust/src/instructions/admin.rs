use crate::states::*;
use anchor_lang::prelude::*;

//Initialize Account
#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(init, payer = owner, space = 8 + 40)]
    pub admin: Account<'info, Admin>,
    #[account(mut)]
    pub owner: Signer<'info>,
    pub system_program: Program<'info, System>,
}

pub fn initialize(ctx: Context<Initialize>, service_fee: u64) -> Result<()> {
    let admin = &mut ctx.accounts.admin;
    admin.owner = *ctx.accounts.owner.key;
    admin.service_fee = service_fee;
    Ok(())
}

//Set Service Fee
#[derive(Accounts)]
pub struct SetServiceFee<'info> {
    #[account(mut, has_one = owner)]
    pub admin: Account<'info, Admin>,
    #[account(signer)]
    pub owner: Signer<'info>,
}

pub fn set_service_fee(ctx: Context<SetServiceFee>, new_fee: u64) -> Result<()> {
    let admin = &mut ctx.accounts.admin;
    admin.service_fee = new_fee;
    Ok(())
}
