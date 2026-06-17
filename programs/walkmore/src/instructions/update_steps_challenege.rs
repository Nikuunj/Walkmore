use anchor_lang::prelude::*;

use crate::{error::ErrorCode, ChallengeOneVOne, UserData};

#[derive(Accounts)]
pub struct UpdateStepsChallenege<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
      mut,
      seeds = [b"challenge", challenge.seed.to_le_bytes().as_ref()],
      bump = challenge.bump
   )]
    pub challenge: Account<'info, ChallengeOneVOne>,

    #[account(
        mut,
        has_one = challenge,
        seeds = [b"user_data", user.key().as_ref(), challenge.key().as_ref()],
        bump = user_data.bump
    )]
    pub user_data: Account<'info, UserData>,
}

impl<'info> UpdateStepsChallenege<'info> {
    pub fn update_steps_challenege(&mut self, steps: u32) -> Result<()> {
        require!(
            self.user.key() == self.challenge.player1 || self.user.key() == self.challenge.player2,
            ErrorCode::InvalidUser
        );

        let currnet_slot = Clock::get()?.slot;

        require!(currnet_slot <= self.challenge.end_time.unwrap(), ErrorCode::Timeout);

        require!(currnet_slot >= self.challenge.start_time.unwrap(), ErrorCode::TimeEarly);

        self.user_data.steps = steps;
        self.user_data.last_update = currnet_slot;
        Ok(())
    }
}

