use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ChallengeOneVOne {
    pub seed: u64,
    pub fee: u64,
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
    pub last_update: i64,
    pub claimed: bool,
    pub bump: u8,
}
