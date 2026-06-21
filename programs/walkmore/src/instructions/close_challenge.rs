use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{close_account, CloseAccount, Mint, TokenAccount, TokenInterface},
};

use crate::{error::ErrorCode, ChallengeOneVOne, UserData};

#[derive(Accounts)]
pub struct CloseChallenge<'info> {
    #[account(mut)]
    pub player1: Signer<'info>,

    /// CHECK: this is player2 account
    pub player2: UncheckedAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = player1,
        has_one = player1,
        has_one = player2,
        has_one = mint,
        seeds = [b"challenge", challenge.seed.to_le_bytes().as_ref()],
        bump =  challenge.bump
    )]
    pub challenge: Account<'info, ChallengeOneVOne>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = challenge,
        associated_token::token_program = token_program
    )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        mut,
        close = player1,
        has_one = challenge,
        seeds = [b"user_data", player1.key().as_ref(), challenge.key().as_ref()],
        bump = player1_data.bump
    )]
    pub player1_data: Account<'info, UserData>,

    #[account(
        mut,
        close = player1,
        has_one = challenge,
        seeds = [b"user_data", player2.key().as_ref(), challenge.key().as_ref()],
        bump = player2_data.bump
    )]
    pub player2_data: Account<'info, UserData>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> CloseChallenge<'info> {
    pub fn close_challenge(&mut self) -> Result<()> {
        let currnet_slot = Clock::get()?.slot;

        require!(
            self.challenge.completed
                || (self.challenge.end_time.is_some()
                    && currnet_slot >= self.challenge.end_time.unwrap()
                    && self.player1_data.steps == self.player2_data.steps),
            ErrorCode::ChallengeNotCompleted
        );

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"challenge",
            &self.challenge.seed.to_le_bytes(),
            &[self.challenge.bump],
        ]];

        close_account(CpiContext::new_with_signer(
            self.token_program.key(),
            CloseAccount {
                account: self.vault_ata.to_account_info(),
                destination: self.player1.to_account_info(),
                authority: self.challenge.to_account_info(),
            },
            signer_seeds,
        ))
    }
}
