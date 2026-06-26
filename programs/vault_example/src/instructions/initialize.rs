use crate::VaultState;
use anchor_lang::{prelude::*, system_program::Transfer};

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    // mutable user who signed for interaction (private key)
    pub user: Signer<'info>,

    #[account(
        init,
        payer = user,
        space = 8 + VaultState::INIT_SPACE,
        seeds = [b"state", user.key().as_ref()],
        bump
    )]
    pub vault_state: Account<'info, VaultState>,

    #[account(mut)]
    pub vault: SystemAccount<'info>,

    pub system_program: Program<'info, System>,
}

pub fn initialize_handler(ctx: Context<Initialize>) -> Result<()> {
    msg!("Greetings from: {:?}", ctx.program_id);
    let cpi_program = ctx.accounts.system_program.to_account_info();
    let cpi_accounts = Transfer {
        from: ctx.accounts.user.to_account_info(),
        to: ctx.accounts.vault.to_account_info(),
    };
    let cpi_ctx = CpiContext::new(cpi_program.key(), cpi_accounts);
    // pay rent
    anchor_lang::system_program::transfer(cpi_ctx, 100_000_000)?;
    ctx.accounts.vault_state.state_bump = ctx.bumps.vault_state;
    ctx.accounts.vault_state.authority = ctx.accounts.user.key();
    Ok(())
}
