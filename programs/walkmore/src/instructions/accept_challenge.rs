use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::ChallengeOneVOne;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct AcceptChallengeOneVOne<'info> {
    #[account(mut)]
    pub player2: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
        mut,
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
        associated_token::authority = player2,
        associated_token::token_program = token_program
   )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info>  AcceptChallengeOneVOne<'info> {
    pub fn accept_challenge(&mut self) -> Result<()> {
        let currnet_time = Clock::get()?.slot;

        self.challenge.start_time = Some(currnet_time);
        self.challenge.end_time = Some(currnet_time.checked_add(self.challenge.duration).unwrap(););


        Ok(())
    }

    pub fn deposit(&mut self) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.key(),
                TransferChecked {
                    from: self.user_ata.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.vault_ata.to_account_info(),
                    authority: self.player2.to_account_info(),
                },
            ),
            self.challenge.fee,
            self.mint.decimals,
        )
    }
}