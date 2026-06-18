use anchor_lang::prelude::*;

use crate::state::Pool;

#[derive(Accounts)]
#[instruction(seed: u128)]
pub struct CreatePool<'info> {
    #[account(mut)]
    pub maker: Signer<'info>,

    #[account(
        init, 
        payer =  maker,
        seeds = [b"pool", maker.key().as_ref(), seed.to_le_bytes().as_ref()],
        space = Pool::INIT_SPACE + Pool::DISCRIMINATOR.len(),
        bump
    )]
    pub pool: Account<'info, Pool>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreatePool<'info> {
    pub fn create_pool(&mut self, seed: u128, start_time: u64, end_time: u64, entry_fee: u64, target: u32, mint: Pubkey, bumps: &CreatePoolBumps) -> Result<()> {

        self.pool.set_inner(Pool {
            maker: self.maker.key(),
            entry_fee,
            end_time,
            seed,
            mint,
            total_participants: 0,
            start_time,
            target,
            winner_count: 0,
            bump: bumps.pool
        });
        Ok(())
    }
}
