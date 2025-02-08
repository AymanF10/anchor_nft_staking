use anchor_lang::prelude::*;
use crate::state::UserAccount;
use crate::error_code;

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)] // so he can pay
    pub user: Signer<'info>,
    #[account(
        init,
        payer = user,
        seeds = [b"user".as_ref(), user.key().as_ref()],
        bump,
        space = 8 + UserAccount::INIT_SPACE,
    )]
    pub user_account: Account<'info, UserAccount>,

    
    pub system_program: Program<'info, System>,


}
impl<'info> InitializeUser<'info> {
    pub fn initialize_user(&mut self, bumps: &InitializeUserBumps) -> Result<()> {
    self.user_account.set_inner(UserAccount {
        points_per_stake: 0,
        amount_staked: 0,
        bump: bumps.user_account,
    });
    Ok(())
}
}