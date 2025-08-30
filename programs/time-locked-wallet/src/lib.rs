use anchor_lang::prelude::*;

declare_id!("JCLD5JBByyLnmndKw6R4iua1Fer8yRfJqp3DGRcVRUd6");

#[program]
pub mod time_locked_wallet {
    use super::*;

    pub fn initialize_lock(
        ctx: Context<InitializeLock>,
        amount: u64,
        unlock_timestamp: i64,
    ) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;

        require!(
            unlock_timestamp > current_time,
            ErrorCode::InvalidUnlockTime
        );

        require!(amount > 0, ErrorCode::InvalidAmount);

        // Transfer SOL to PDA
        anchor_lang::system_program::transfer(
            CpiContext::new(
                ctx.accounts.system_program.to_account_info(),
                anchor_lang::system_program::Transfer {
                    from: ctx.accounts.owner.to_account_info(),
                    to: ctx.accounts.time_locked_wallet.to_account_info(),
                },
            ),
            amount,
        )?;

        let time_locked_wallet = &mut ctx.accounts.time_locked_wallet;
        time_locked_wallet.owner = ctx.accounts.owner.key();
        time_locked_wallet.amount = amount;
        time_locked_wallet.unlock_timestamp = unlock_timestamp;
        time_locked_wallet.bump = ctx.bumps.time_locked_wallet;
        time_locked_wallet.created_at = current_time;

        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>) -> Result<()> {
        let current_time = Clock::get()?.unix_timestamp;
        let time_locked_wallet = &ctx.accounts.time_locked_wallet;

        require!(
            current_time >= time_locked_wallet.unlock_timestamp,
            ErrorCode::FundsStillLocked
        );

        // Transfer all lamports from PDA back to owner
        let wallet_lamports = time_locked_wallet.to_account_info().lamports();
        **time_locked_wallet.to_account_info().try_borrow_mut_lamports()? -= wallet_lamports;
        **ctx.accounts.owner.to_account_info().try_borrow_mut_lamports()? += wallet_lamports;

        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeLock<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        init,
        payer = owner,
        space = TimeLockedWallet::LEN,
        seeds = [TimeLockedWallet::SEED, owner.key().as_ref()],
        bump
    )]
    pub time_locked_wallet: Account<'info, TimeLockedWallet>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Withdraw<'info> {
    #[account(mut)]
    pub owner: Signer<'info>,

    #[account(
        mut,
        seeds = [TimeLockedWallet::SEED, owner.key().as_ref()],
        bump = time_locked_wallet.bump,
        close = owner,
        has_one = owner
    )]
    pub time_locked_wallet: Account<'info, TimeLockedWallet>,
}

#[account]
pub struct TimeLockedWallet {
    pub owner: Pubkey,
    pub amount: u64,
    pub unlock_timestamp: i64,
    pub bump: u8,
    pub created_at: i64,
}

impl TimeLockedWallet {
    pub const LEN: usize = 8 + 32 + 8 + 8 + 1 + 8;
    pub const SEED: &'static [u8] = b"time_locked_wallet";
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unlock timestamp must be in the future")]
    InvalidUnlockTime,
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
    #[msg("Funds are still locked")]
    FundsStillLocked,
}