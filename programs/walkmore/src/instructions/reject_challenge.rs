use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked},
};

use crate::ChallengeOneVOne;

#[derive(Accounts)]
pub struct RejectChallenge<'info> {
    #[account(mut)]
    pub player2: Signer<'info>,

    #[account(mut)]
    pub player1: SystemAccount<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
        has_one = player2,
        has_one = player1,
        has_one = mint,
        close = player2,
        seeds = [b"challenge", challenge.seed.to_le_bytes().as_ref()],
        bump = challenge.bump
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
        associated_token::mint = mint,
        associated_token::authority = player1,
        associated_token::token_program = token_program
    )]
    pub player1_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> RejectChallenge<'info> {
    pub fn reject_challenge(&mut self) -> Result<()> {
        let signer_seeds: &[&[&[u8]]] = &[&[
            b"challenge",
            &self.challenge.seed.to_le_bytes(),
            &[self.challenge.bump],
        ]];

        transfer_checked(
            CpiContext::new_with_signer(
                self.token_program.key(),
                TransferChecked {
                    to: self.player1_ata.to_account_info(),
                    mint: self.mint.to_account_info(),
                    from: self.vault_ata.to_account_info(),
                    authority: self.challenge.to_account_info(),
                },
                signer_seeds
            ),
            self.vault_ata.amount,
            self.mint.decimals,
        )
    }
}
