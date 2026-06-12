use anchor_lang::prelude::*;

#[account]
#[derive(InitSpace)]
pub struct ChallengeOneVOne {
    pub seed: u64,
    pub end_time: Option<i64>,
    pub start_time: Option<i64>,
    pub duration: i64,
    pub player1: Pubkey,
    pub player2: Pubkey,
    pub bump: u8,
    pub vault_bump: u8,
}

#[account]
#[derive(InitSpace)]
pub struct UserData {
    pub steps: u64,
    pub challenge: Pubkey,
    pub last_update: i64,
    pub bump: u8,
}
