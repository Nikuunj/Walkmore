use anchor_lang::prelude::*;

use crate::{Pool, UserData};

#[derive(Accounts)]
pub struct CreateUserDataAcc<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    #[account(
        mut,
        seeds = [b"pool", pool.maker.as_ref(), pool.seed.to_le_bytes().as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        init,
        payer = user,
        seeds = [b"use-data", user.key().as_ref(), pool.key().as_ref()],
        space = UserData::INIT_SPACE + UserData::DISCRIMINATOR.len(),
        bump
    )]
    pub user_data: Account<'info, UserData>,

    pub system_program: Program<'info, System>,
}

impl<'info> CreateUserDataAcc<'info> {
    pub fn create_user_data_acc(&mut self, bumps: &CreateUserDataAccBumps) -> Result<()> {
        self.user_data.set_inner(UserData {
            steps: 0,
            challenge: self.pool.key(),
            last_update: Clock::get()?.slot,
            completed: false,
            claimed: false,
            bump: bumps.user_data,
        });
        Ok(())
    }
}
