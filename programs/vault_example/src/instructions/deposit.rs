// get VaultState state (created in state.rs)
use crate::VaultState;
// invoke prelude*
use anchor_lang::prelude::*;
// invoke offical transfer lib
use anchor_lang::system_program::Transfer;

#[derive(Accounts)]
pub struct Deposit<'info> {
    // get the signer
    #[account(mut)]
    pub user: Signer<'info>,

    // vault state account, with constraints
    #[account(
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    // vault account, mutable
    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,
    // system program
    pub system_program: Program<'info, System>,
}

pub fn deposit_handler(ctx: Context<Deposit>, amount: u64) -> Result<()> {
    msg!("Deposit by: {:?}", ctx.accounts.user.key());
    // create program
    let cpi_program = ctx.accounts.system_program.to_account_info();
    // this is total series of accounts in the transaction
    let cpi_accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program.key(), cpi_accounts);
    // pay rent
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    Ok(())
}
