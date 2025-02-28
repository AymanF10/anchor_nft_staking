use anchor_lang::prelude::*;

declare_id!("ALfPg8d22KpbGWXk9mF8Zqfq3L2WSJs3JfHoYWYoGpFq");

pub mod context;
use context::*;
pub mod state;
use state::*;
pub mod error;
use error::*;

#[program]
    pub mod nft_staking {
    use super::*;

    pub fn initialize_config(ctx: Context<InitializeConfig>, points_per_stake: u8, max_stake: u8, freeze_period: u32) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        ctx.accounts.initialize_config(points_per_stake, max_stake, freeze_period, &ctx.bumps)?;
        Ok(())
    }
    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        ctx.accounts.initialize_user(&ctx.bumps);
        Ok(()) 
    }
    pub fn stake(ctx: Context<Stake>) -> Result<()> {
        ctx.accounts.stake(&ctx.bumps)
    }
    pub fn unstake(ctx: Context<Unstake>) -> Result<()> {
        ctx.accounts.unstake()
    }
    pub fn claim(ctx: Context<Claim>) -> Result<()> {
        ctx.accounts.claim()?;
        Ok(())
    }
}

//Program Id: ALfPg8d22KpbGWXk9mF8Zqfq3L2WSJs3JfHoYWYoGpFq                                                                
//Signature: 4feAVwDjEvQpuZuSMCUDWXvQwa4r5f6VxknwgAkp9PBJxtWvaA1v4JfJ7dEdLbXN4QunBA1LMQfZn25L2FTPr1Mq
