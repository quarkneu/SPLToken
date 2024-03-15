use anchor_lang::prelude::*;

mod errors;
mod state;

pub mod instructions;

use instructions::*;

declare_id!("FUQzNT2qpciT71Mqj8Ewi58Pkase3ANt5d1S8x87SPfB");

#[program]
pub mod spl_token {
    use super::*;

    pub fn set(ctx: Context<Set>) -> Result<()> {
        set::set_handler(ctx)
    }

    pub fn add_user(ctx: Context<AddUser>) -> Result<()> {
        add_user::add_user_handler(ctx)
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        deposit::deposit_handler(ctx, amount)
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        withdraw::withdraw_handler(ctx, amount)
    }
}
