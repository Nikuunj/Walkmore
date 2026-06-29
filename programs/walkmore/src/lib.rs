pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("2147VBdRR2xGS6BnY8RsprPhh4MHPoApgnzqgNm2LLFu");

#[program]
pub mod walkmore {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        initialize::handler(ctx)
    }

    pub fn create_pool(
        ctx: Context<CreatePool>,
        seed: u128,
        start_time: u64,
        end_time: u64,
        entry_fee: u64,
        target: u32,
        mint: Pubkey,
    ) -> Result<()> {
        ctx.accounts.create_pool(
            seed, start_time, end_time, entry_fee, target, mint, &ctx.bumps,
        )
    }

    pub fn create_user_account(ctx: Context<CreateUserDataAcc>) -> Result<()> {
        ctx.accounts.create_user_data_acc(&ctx.bumps)
    }

    pub fn join_pool(ctx: Context<JoinPool>) -> Result<()> {
        ctx.accounts.join_pool()
    }

    pub fn create_challenge_onevone(
        ctx: Context<CreateChallengeOneVOne>,
        seed: u64,
        player2: Pubkey,
        duration: u64,
        fee: u64
    ) -> Result<()> {

        ctx.accounts
            .create_challenge(seed, player2, duration, fee, &ctx.bumps)?;

        ctx.accounts.deposit(fee)
    }

    pub fn accept_challenge(ctx: Context<AcceptChallengeOneVOne>) -> Result<()> {
        ctx.accounts.accept_challenge()
    }

    pub fn accept_challenge_deposit(ctx: Context<AcceptChallengeOneVOne>) -> Result<()> {
        ctx.accounts.deposit()
    }

    pub fn update_steps_challenge(ctx: Context<UpdateStepsChallenege>, steps: u32) -> Result<()> {
        ctx.accounts.update_steps_challenege(steps)
    }

    pub fn update_steps_pool(ctx: Context<UpdateStepsPool>, steps: u32) -> Result<()> {
        ctx.accounts.update_steps_pool(steps)
    }

    pub fn reject_challenge(ctx: Context<RejectChallenge>) -> Result<()> {
        ctx.accounts.reject_challenge()
    }

    pub fn close_challenge(ctx: Context<CloseChallenge>) -> Result<()> {
        ctx.accounts.close_challenge()
    }

    pub fn winner_withdraw_onevone(
        ctx: Context<WinnerWithdrawOneVOne>,
        player1: Pubkey,
        player2: Pubkey,
    ) -> Result<()> {
        ctx.accounts.winner_withdraw_onevone(player1, player2)
    }

    pub fn winner_withdraw_pool(ctx: Context<WinnerWithdrawPool>) -> Result<()> {
        ctx.accounts.winner_withdraw_pool()
    }
}
