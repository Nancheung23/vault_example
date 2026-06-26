// get VaultState state (created in state.rs)
use crate::VaultState;
// invoke prelude*
use anchor_lang::prelude::*;
use anchor_lang::system_program::Transfer;

#[derive(Accounts)]
pub struct Close<'info> {
    #[account(mut)]
    pub user: Signer<'info>,

    // vault state account, with constraints
    #[account(
        mut,
        close = user,
        seeds = [b"state", user.key().as_ref()],
        bump = vault_state.state_bump,
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(
        mut,
        seeds = [b"vault", user.key().as_ref()],
        bump
    )]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn close_handler(ctx: Context<Close>) -> Result<()> {
    msg!("closing account: {:?}", ctx.accounts.vault.key());
    let balance = ctx.accounts.vault.lamports();
    if balance > 0 {
        let user_key = ctx.accounts.user.key();
        let vault_bump = ctx.bumps.vault;
        let seeds = &[b"vault", user_key.as_ref(), &[vault_bump]];
        let signer_seeds = &[&seeds[..]];

        let cpi_program = ctx.accounts.system_program.to_account_info();
        let cpi_accounts = Transfer {
            from: ctx.accounts.vault.to_account_info(),
            to: ctx.accounts.user.to_account_info(),
        };

        let cpi_ctx = CpiContext::new_with_signer(cpi_program.key(), cpi_accounts, signer_seeds);
        anchor_lang::system_program::transfer(cpi_ctx, balance)?;
    }

    Ok(())
}
