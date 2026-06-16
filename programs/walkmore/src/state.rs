use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct Pool {
    pub maker: Pubkey,
    pub entry_fee: u64,
    pub seed: u128,
    pub mint: Pubkey,
    pub end_time: u64,
    pub total_participants: u16,
    pub winner_count: u16,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct ChallengeOneVOne {
    pub seed: u64,
    pub entry_fee: u64,
    pub end_time: Option<u64>,
    pub start_time: Option<u64>,
    pub mint: Pubkey,
    pub duration: u64,
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserData {
    pub steps: u64,
    pub challenge: Pubkey,
    pub last_update: u64,
    pub claimed: bool,
    pub bump: u8,
}
