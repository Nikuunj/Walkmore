use anchor_lang::prelude::*;

use crate::{error::ErrorCode, Pool, UserData};

#[derive(Accounts)]
pub struct UpdateStepsPool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"pool", challenge.maker.as_ref(), challenge.seed.to_le_bytes().as_ref()],
        bump
   )]
    pub challenge: Account<'info, Pool>,

    #[account(
        mut,
        has_one = challenge,
        seeds = [b"user_data", user.key().as_ref(), challenge.key().as_ref()],
        bump = user_data.bump
    )]
    pub user_data: Account<'info, UserData>,
}

impl<'info> UpdateStepsPool<'info> {
    pub fn update_steps_pool(&mut self, steps: u32) -> Result<()> {
        let currnet_slot = Clock::get()?.slot;

        require!(currnet_slot <= self.challenge.end_time, ErrorCode::Timeout);

        require!(
            currnet_slot >= self.challenge.start_time,
            ErrorCode::TimeEarly
        );

        self.user_data.steps = steps;
        self.user_data.last_update = currnet_slot;

        if steps >= self.challenge.target {
            self.challenge.winner_count = self.challenge.winner_count + 1;
        }

        Ok(())
    }
}
