use crate::errors::ErrorCode;
use crate::states::*;
use anchor_lang::prelude::*;

#[derive(Accounts)]
pub struct PlaceOrder<'info> {
    #[account(init, payer = buyer, space = 8 + 80)]
    pub order: Account<'info, Order>,
    #[account(mut)]
    pub buyer: Signer<'info>,
    #[account(mut)]
    pub product: Account<'info, Product>,
    #[account(mut)]
    pub seller_wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub buyer_wallet: Account<'info, Wallet>,
    #[account(mut)]
    pub admin: Account<'info, Admin>,
    pub system_program: Program<'info, System>,
}

pub fn place_order(ctx: Context<PlaceOrder>, product_id: Pubkey) -> Result<()> {
    let order = &mut ctx.accounts.order;
    let product = &mut ctx.accounts.product;
    let buyer_wallet = &mut ctx.accounts.buyer_wallet;
    let seller_wallet = &mut ctx.accounts.seller_wallet;
    let admin = &ctx.accounts.admin;

    require!(
        buyer_wallet.balance >= product.price,
        ErrorCode::InsufficientBalance
    );
    require!(product.quantity > 0, ErrorCode::OutOfStock);

    let transfer_amount = product.price + admin.service_fee;
    buyer_wallet.balance -= transfer_amount;
    seller_wallet.balance += product.price;

    product.quantity -= 1;
    product.amount_sold += 1;

    order.buyer = *ctx.accounts.buyer.key;
    order.product = product_id;
    order.amount_paid = product.price;
    order.status = "Completed".to_string();

    Ok(())
}
