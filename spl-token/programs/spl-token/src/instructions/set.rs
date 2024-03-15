use {
    crate::{
        errors::ErrorCode,
        state::{TokenPool, ADMIN_PUBKEY},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{Mint, Token, TokenAccount},
    },
};

#[derive(Accounts)]
pub struct Set<'info> {
    #[account(
        mut,
        address = ADMIN_PUBKEY @ ErrorCode::NotAdminError,
    )]
    pub admin: Signer<'info>,

    #[account(mut)]
    pub mint_account: Account<'info, Mint>,

    #[account(
        init,
        seeds = [
            TokenPool::TOKEN_POOL.as_bytes(),
            admin.key().as_ref(),
            mint_account.key().as_ref(),
        ],
        bump,
        payer = admin,
        space = TokenPool::LEN,
    )]
    pub token_pool: Account<'info, TokenPool>,

    #[account(
        seeds = [b"vault", token_pool.admin.key().as_ref()],
        bump,
    )]
    pub token_vault: SystemAccount<'info>,

    #[account(
        init_if_needed,
        payer = admin,
        associated_token::authority = token_vault,
        associated_token::mint = mint_account,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn set_handler(ctx: Context<Set>) -> Result<()> {
    msg!("Set SPL token: {:?}", ctx.accounts.mint_account.key());

    let token_pool = &mut ctx.accounts.token_pool;
    token_pool.admin = ctx.accounts.admin.key();
    token_pool.mint = ctx.accounts.mint_account.key();
    token_pool.bump = ctx.bumps.token_pool;

    Ok(())
}
