use anchor_lang::prelude::*;
use solana_program::pubkey;

// Replace it as a real public key when deploying
pub const ADMIN_PUBKEY: Pubkey = pubkey!("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx");

#[account]
#[derive(Default)]
pub struct TokenPool {
    pub admin: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

#[account]
#[derive(Default)]
pub struct UserInfo {
    pub user: Pubkey,
    pub mint: Pubkey,
    pub amount: u64,
    pub bump: u8,
}

impl TokenPool {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;

    pub const TOKEN_POOL: &'static str = "token_pool";
}

impl UserInfo {
    pub const LEN: usize = 8 + 32 + 32 + 8 + 1;

    pub const USER_INFO: &'static str = "user_info";
}
