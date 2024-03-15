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
pub struct Deposit<'info> {
    #[account(mut)]
    pub depositor: Signer<'info>,

    #[account(
        mut,
        address = token_pool.mint,
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
        seeds = [
            UserInfo::USER_INFO.as_bytes(),
            depositor.key().as_ref(),
            mint_account.key().as_ref()
        ],
        bump = user_info.bump,
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
        associated_token::authority = depositor,
    )]
    pub depositor_token_account: Account<'info, TokenAccount>,

    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

pub fn deposit_handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    msg!("Deposit {:?} SPL tokens", amount);

    require_gt!(amount, 0, ErrorCode::InvalidWithdrawTokenAmountError);

    transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.depositor_token_account.to_account_info(),
                to: ctx.accounts.vault_ata.to_account_info(),
                authority: ctx.accounts.depositor.to_account_info(),
            },
        ),
        amount * 10u64.pow(ctx.accounts.mint_account.decimals as u32),
    )?;

    let token_pool = &mut ctx.accounts.token_pool;
    token_pool.amount = token_pool.amount.checked_add(amount).unwrap();

    let user_info = &mut ctx.accounts.user_info;
    user_info.amount = user_info.amount.checked_add(amount).unwrap();

    msg!("There are {:?} tokens in token pool", token_pool.amount);
    msg!(
        "User {:?} deposits {:?} tokens to token pool",
        ctx.accounts.depositor,
        amount
    );

    Ok(())
}
