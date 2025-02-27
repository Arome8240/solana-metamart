use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub seller: Signer<'info>,
    #[account(mut)]
    pub seller_wallet: Account<'info, Wallet>,
}

pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    let seller_wallet = &mut ctx.accounts.seller_wallet;

    require!(
        seller_wallet.balance >= amount,
        ErrorCode::InsufficientBalance
    );

    seller_wallet.balance -= amount;
    seller_wallet.total_withdrawn += amount;

    Ok(())
}
