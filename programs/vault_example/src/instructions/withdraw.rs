// get VaultState state (created in state.rs)
use crate::VaultState;
// invoke prelude*
use anchor_lang::prelude::*;
// invoke offical transfer lib
use anchor_lang::system_program::Transfer;

#[derive(Accounts)]
pub struct Withdraw<'info> {
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

pub fn withdraw_handler(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
    msg!("Withdraw from: {:?}", ctx.accounts.vault.key());
    let user_key = ctx.accounts.user.key();
    let seeds = &[b"vault", user_key.as_ref(), &[ctx.bumps.vault]];
    let signer_seeds = &[(&seeds[..])];

    // create program
    let cpi_program = ctx.accounts.system_program.to_account_info();
    // this is total series of accounts in the transaction
    let cpi_accounts = Transfer {
        from: ctx.accounts.vault.to_account_info(),
        to: ctx.accounts.user.to_account_info(),
    };
    let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);
    // pay rent
    anchor_lang::system_program::transfer(cpi_ctx, amount)?;
    Ok(())
}
