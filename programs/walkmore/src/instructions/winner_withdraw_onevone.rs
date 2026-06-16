use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{
        close_account, transfer_checked, CloseAccount, Mint, TokenAccount, TokenInterface,
        TransferChecked,
    },
};

use crate::{error::ErrorCode, ChallengeOneVOne, UserData};

#[derive(Accounts)]
#[instruction( player1: Pubkey, player2: Pubkey )]
pub struct WinnerWithdrawOneVOne<'info> {
    #[account(mut)]
    pub winner: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        close = winner,
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
        close = winner,
        seeds = [b"user_data", player1.as_ref(), challenge.key().as_ref()],
        bump = player1_data.bump
    )]
    pub player1_data: Account<'info, UserData>,

    #[account(
        mut,
        close = winner,
        seeds = [b"user_data", player2.as_ref(), challenge.key().as_ref()],
        bump = player2_data.bump
    )]
    pub player2_data: Account<'info, UserData>,

    #[account(
        mut,
        associated_token::mint = mint,
        associated_token::authority = winner,
        associated_token::token_program = token_program
    )]
    pub winner_ata: InterfaceAccount<'info, TokenAccount>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
}

impl<'info> WinnerWithdrawOneVOne<'info> {
    pub fn winner_withdraw_onevone(&mut self, player1: Pubkey, player2: Pubkey) -> Result<()> {

        let currnet_slot = Clock::get()?.slot;

        require!(currnet_slot >= self.challenge.end_time.unwrap(), ErrorCode::CustomError);
        
        require_keys_eq!(player1, self.challenge.player1);
        require_keys_eq!(player2, self.challenge.player2);


        require!(
            self.winner.key() == player1 || self.winner.key() == player2,
            ErrorCode::InvalidWinner
        );

        let winner_check: Option<Pubkey> =
            match self.player1_data.steps.cmp(&self.player2_data.steps) {
                std::cmp::Ordering::Greater => Some(player1),
                std::cmp::Ordering::Less => Some(player2),
                std::cmp::Ordering::Equal => None,
            };

        require_keys_eq!(winner_check.unwrap(), self.winner.key());

        let signer_seeds: &[&[&[u8]]] = &[&[
            b"challenge",
            &self.challenge.seed.to_le_bytes(),
            &[self.challenge.bump],
        ]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.key(),
                TransferChecked {
                    from: self.vault_ata.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.winner_ata.to_account_info(),
                    authority: self.challenge.to_account_info(),
                },
                signer_seeds,
            ),
            self.vault_ata.amount,
            self.mint.decimals,
        )?;

        close_account(CpiContext::new_with_signer(
            self.token_program.key(),
            CloseAccount {
                account: self.vault_ata.to_account_info(),
                destination: self.winner.to_account_info(),
                authority: self.challenge.to_account_info(),
            },
            signer_seeds,
        ))
    }
}
