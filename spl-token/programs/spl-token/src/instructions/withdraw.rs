use {
    crate::{
        errors::ErrorCode,
        state::{TokenPool, UserInfo},
    },
    anchor_lang::prelude::*,
    anchor_spl::{
        associated_token::AssociatedToken,
        token::{transfer, Mint, Token, TokenAccount, Transfer},
    },
};

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        address = token_pool.mint
    )]
    pub mint_account: Account<'info, Mint>,

    #[account(
        mut,
        seeds = [
            TokenPool::TOKEN_POOL.as_bytes(),
            token_pool.admin.key().as_ref(),
            mint_account.key().as_ref(),
        ],
        bump = token_pool.bump,
    )]
    pub token_pool: Account<'info, TokenPool>,

    #[account(
        mut,
        has_one = user,
        constraint = user_info.mint == token_pool.mint,
    )]
    pub user_info: Account<'info, UserInfo>,

    #[account(
        seeds = [b"vault", token_pool.admin.key().as_ref()],
        bump,
    )]
    pub token_vault: SystemAccount<'info>,

    #[account(
        mut,
        associated_token::authority = token_vault,
        associated_token::mint = mint_account,
    )]
    pub vault_ata: Account<'info, TokenAccount>,

    #[account(
        mut,
        associated_token::mint = mint_account,
        associated_token::authority = user,
    )]
    pub recipient_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn withdraw_handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    msg!("Withdraw {:?} SPL tokens", amount);

    require_gte!(
        ctx.accounts.user_info.amount,
        amount,
        ErrorCode::InvalidWithdrawTokenAmountError
    );

    let token_pool = &mut ctx.accounts.token_pool;
    let token_vault_bump = ctx.bumps.token_vault;
    let admin_key = token_pool.admin.key();
    let seeds = &[
        b"vault",
        admin_key.as_ref(),
        &[token_vault_bump],
    ];
    let signer_seeds = &[&seeds[..]];

    transfer(
        CpiContext::new_with_signer(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.vault_ata.to_account_info(),
                to: ctx.accounts.recipient_token_account.to_account_info(),
                authority: ctx.accounts.token_vault.to_account_info(),
            },
            signer_seeds,
        ),
        amount * 10u64.pow(ctx.accounts.mint_account.decimals as u32),
    )?;

    token_pool.amount = token_pool.amount.checked_sub(amount).unwrap();

    let user_info = &mut ctx.accounts.user_info;
    user_info.amount = user_info.amount.checked_sub(amount).unwrap();

    msg!("There are {:?} tokens in token pool", token_pool.amount);
    msg!(
        "User {:?} withdraws {:?} tokens from token pool",
        ctx.accounts.user,
        amount
    );

    Ok(())
}
