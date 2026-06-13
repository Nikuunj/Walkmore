use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{transfer_checked, Mint, TokenAccount, TokenInterface, TransferChecked},
};

use crate::ChallengeOneVOne;

#[derive(Accounts)]
#[instruction(seed: u64)]
pub struct CreateChallengeOneVOne<'info> {
    #[account(mut)]
    pub creator: Signer<'info>,

    pub mint: InterfaceAccount<'info, Mint>,

    #[account(
      init,
      payer = creator,
      space = ChallengeOneVOne::INIT_SPACE + ChallengeOneVOne::DISCRIMINATOR.len(),
      seeds = [b"challenge", seed.to_le_bytes().as_ref()],
      bump
   )]
    pub challenge: Account<'info, ChallengeOneVOne>,

    #[account(
      init,
      payer = creator,
      associated_token::mint = mint,
      associated_token::authority = challenge,
      associated_token::token_program = token_program
   )]
    pub vault_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
      mut,
      associated_token::mint = mint,
      associated_token::authority = creator,
      associated_token::token_program = token_program
   )]
    pub player1_ata: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl<'info> CreateChallengeOneVOne<'info> {
    pub fn create_challenge(
        &mut self,
        seed: u64,
        player2: Pubkey,
        duration: u64,
        fee: u64,
        bumps: &CreateChallengeOneVOneBumps,
    ) -> Result<()> {
        self.challenge.set_inner(ChallengeOneVOne {
            seed,
            entry_fee: fee,
            end_time: None,
            start_time: None,
            mint: self.mint.key(),

            // want slot so first second * 1000 / 400;
            duration,
            player1: self.creator.key(),
            player2,
            bump: bumps.challenge,
        });
        Ok(())
    }

    pub fn deposit(&mut self, fee: u64) -> Result<()> {
        transfer_checked(
            CpiContext::new(
                self.token_program.key(),
                TransferChecked {
                    from: self.player1_ata.to_account_info(),
                    mint: self.mint.to_account_info(),
                    to: self.vault_ata.to_account_info(),
                    authority: self.creator.to_account_info(),
                },
            ),
            fee,
            self.mint.decimals,
        )
    }
}
