use anchor_lang::prelude::*;

pub mod constants;
pub mod errors;
pub mod instructions;
pub mod states;

use instructions::*;

declare_id!("9FGTXmm1XEUPMyCne9ugzvmSSQGSpdHQdmmLP4wWwmFd");

#[program]
pub mod metamart {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, service_fee: u64) -> Result<()> {
        admin::initialize(ctx, service_fee)
    }

    pub fn set_service_fee(ctx: Context<SetServiceFee>, new_fee: u64) -> Result<()> {
        admin::set_service_fee(ctx, new_fee)
    }

    //Products

    pub fn add_product(
        ctx: Context<AddProduct>,
        title: String,
        description: String,
        price: u64,
        quantity: u64,
        image_cid: String,
    ) -> Result<()> {
        product::add_product(ctx, title, description, price, quantity, image_cid)
    }

    pub fn update_product(
        ctx: Context<UpdateProduct>,
        name: String,
        description: String,
        quantity: u64,
        price: u64,
    ) -> Result<()> {
        update_product::update_product(ctx, name, description, quantity, price)
    }

    pub fn delete_product(ctx: Context<DeleteProduct>) -> Result<()> {
        delete_product::delete_product(ctx)
    }

    pub fn deactivate_product(ctx: Context<DeactivateProduct>) -> Result<()> {
        deactivate_product::deactivate_product(ctx)
    }

    pub fn admin_deactivate_product(ctx: Context<AdminDeactivateProduct>) -> Result<()> {
        admin_deactivate_product::admin_deactivate_product(ctx)
    }

    //Orders
    pub fn place_order(ctx: Context<PlaceOrder>, product_id: Pubkey) -> Result<()> {
        order::place_order(ctx, product_id)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        wallet::withdraw(ctx, amount)
    }
}
