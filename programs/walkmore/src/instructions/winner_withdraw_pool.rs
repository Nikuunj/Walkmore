use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked}};

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

        require!(
            currnet_slot > self.pool.end_time
                || (self.pool.target <= self.user_data.steps && self.user_data.completed),
            ErrorCode::CustomError
        );
        require!(!self.user_data.claimed, ErrorCode::AlreadyClaimed);

        // [b"pool", pool.maker.as_ref(), pool.seed.to_le_bytes().as_ref()]

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"pool",
            self.pool.maker.as_ref(),
            &self.pool.seed.to_le_bytes(),
            &[self.pool.bump]
        ]];

        // tranfer 
        // enetry fee + loser user amount / winner + 1(1 is for contract get paid);
        let loser_amount = (self.pool.total_participants.checked_sub(self.pool.winner_count).unwrap() as u64).checked_mul(self.pool.entry_fee).unwrap().checked_div((self.pool.winner_count + 1 )as u64).unwrap();
        let mut winning_amount = self.pool.entry_fee;



        winning_amount = winning_amount + loser_amount;

        transfer_checked(
            CpiContext::new_with_signer(self.token_program.key(), 
            TransferChecked { from: self.pool_vault.to_account_info(), mint: self.mint.to_account_info(), to: self.user_ata.to_account_info(), authority: self.pool.to_account_info() }, signer_seeds), winning_amount, self.mint.decimals)
        
    }
}
