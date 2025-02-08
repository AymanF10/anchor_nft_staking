use anchor_lang::prelude::*;
use anchor_spl::{associated_token::AssociatedToken, metadata:: { Metadata, MetadataAccount}, token::{mint_to, Mint, MintTo, Token, TokenAccount}};
use crate::state::user_account::UserAccount;
use crate::state::stake_account::StakeAccount;
use crate::state::stake_config::StakeConfig;

use crate::StakeError;

#[derive(Accounts)]
pub struct Claim<'info> {
    #[account(mut)] // so he can pay
    pub user: Signer<'info>,
    #[account(
        mut,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump = user_account.bump,
    )]
    pub user_account: Account<'info, UserAccount>,
    #[account(
        mut,
        seeds = [b"rewards".as_ref(), config.key().as_ref()],
        bump = config.rewards_bump,
    )]
    pub rewards_mint: Account<'info, Mint>,
   
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = rewards_mint,
        associated_token::authority = user,
    )]
    pub rewards_ata: Account<'info, TokenAccount>,
    #[account(
        seeds = [b"config".as_ref()],
        bump = config.bump
    )]
    pub config: Account<'info, StakeConfig>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub metadata_program: Program<'info, Metadata>,
}
impl <'info> Claim<'info> {
    pub fn claim(&mut self) -> Result<()> {

        let cpi_program: AccountInfo<'_> = self.token_program.to_account_info();
        
        let seeds: &[&[u8]; 2] = &[
            b"config".as_ref(),
            &[self.config.bump]
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_accounts = MintTo {
            mint: self.rewards_mint.to_account_info(),
            to: self.rewards_ata.to_account_info(),
            authority: self.config.to_account_info()
        };
        let cpi_ctx = CpiContext::new_with_signer(cpi_program, cpi_accounts, signer_seeds);
        
        let amount = self.user_account.points_per_stake as u64 * 10_u64.pow(self.rewards_mint.decimals as u32);
        mint_to(cpi_ctx, amount)?;
        self.user_account.points_per_stake =0;
        Ok(()) 
    }
}