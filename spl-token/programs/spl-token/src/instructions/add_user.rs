use {
    crate::state::{TokenPool, UserInfo},
    anchor_lang::prelude::*,
    anchor_spl::token::{Mint, Token},
};

#[derive(Accounts)]
pub struct AddUser<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        address = token_pool.mint,
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        seeds = [
            TokenPool::TOKEN_POOL.as_bytes(),
            token_pool.admin.key().as_ref(),
            mint_account.key().as_ref(),
        ],
        bump = token_pool.bump,
    )]
    pub token_pool: Account<'info, TokenPool>,

    #[account(
        init,
        seeds = [
            UserInfo::USER_INFO.as_bytes(),
            depositor.key().as_ref(),
            mint_account.key().as_ref()
        ],
        bump,
        payer = depositor,
        space = UserInfo::LEN,
    )]
    pub user_info: Account<'info, UserInfo>,

    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
}

pub fn add_user_handler(ctx: Context<AddUser>) -> Result<()> {
    msg!("add user info");

    let user_info = &mut ctx.accounts.user_info;
    user_info.amount = 0;
    user_info.user = ctx.accounts.depositor.key();
    user_info.mint = ctx.accounts.mint_account.key();
    user_info.bump = ctx.bumps.user_info;

    Ok(())
}
