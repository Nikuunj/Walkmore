use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token_interface::{Mint, TokenAccount, TokenInterface, transfer_checked, TransferChecked},
};

use crate::{Pool};

#[derive(Accounts)]
pub struct JoinPool<'info> {
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
        associated_token::mint = mint,
        associated_token::token_program = token_program,
        associated_token::authority = user
    )]
    pub user_ata: InterfaceAccount<'info, TokenAccount>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = mint,
        associated_token::token_program = token_program, 
        associated_token::authority = pool
    )]
    pub pool_vault: InterfaceAccount<'info, TokenAccount>,

    pub associated_token_program: Program<'info, AssociatedToken>,
    pub token_program: Interface<'info, TokenInterface>,
    pub system_program: Program<'info, System>,
}

impl <'info> JoinPool<'info> {

    pub fn join_pool(&mut self) -> Result<()> {
        self.pool.total_participants = self
            .pool
            .total_participants
            .checked_add(1)
            .unwrap();

        transfer_checked(CpiContext::new(self.token_program.key(),
         TransferChecked {
            to: self.pool_vault.to_account_info(),
            from: self.user_ata.to_account_info(),
            mint: self.mint.to_account_info(),
            authority: self.user.to_account_info()
        }), self.pool.entry_fee, self.mint.decimals)

    }
}
