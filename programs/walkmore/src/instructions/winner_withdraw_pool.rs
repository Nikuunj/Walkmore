use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface}};

use crate::{Pool, UserData, error::ErrorCode};

#[derive(Accounts)]
pub struct WinnerWithdrawPool<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        has_one = mint,
        seeds = [b"pool", pool.maker.as_ref(), pool.seed.to_le_bytes().as_ref()],
        bump = pool.bump
    )]
    pub pool: Account<'info, Pool>,

    #[account(
        mut,
        close = user,
        seeds = [b"user_data", user.key().as_ref(), pool.key().as_ref()],
        bump = user_data.bump
    )]
    pub user_data: Account<'info, UserData>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = user
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,
    
    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::token_program = token_program, 
        associated_token::authority = pool
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> WinnerWithdrawPool<'info> {
    pub fn winner_withdraw_pool(&mut self) -> Result<()> {
        
        let currnet_slot = Clock::get()?.slot;

        require!(currnet_slot >= self.pool.end_time, ErrorCode::CustomError);
        require!(self.pool.target <= self.user_data.steps, ErrorCode::CustomError);

        // tranfer 
        // enetry fee + loser user amount / winner + 1(1 is for contract get paid);

        

        Ok(())
    }
}