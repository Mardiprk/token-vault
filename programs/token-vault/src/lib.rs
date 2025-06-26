use anchor_lang::prelude::*;

declare_id!("8sNqEgxShvZ4q8t51Jrsg2prbRrcsDVbYKaz5MqwR7bQ");

#[program]
pub mod token_vault {
    use super::*;

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        //amount shouldn't be negative
        require!(amount > 0, ErrorCode::InvalidAmount);

        //transfer tokens from user to vault
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.user_token_account.to_account_info(),
                    to: ctx.accounts.vault_token_account.to_account_info(),
                    authority: ctx.accounts.user.to_account_info(),
                },
            ),
            amount,
        )?;

        msg!("Deposited {} tokens", amount);

        Ok(())
    }
}

#[derive(Account)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub user_token_account: Account<'info, TokenAccount>,

    #[account(mut)]
    pub vault_token_account: Account<'info, TokenAccount>,

    pub user: Signer<'info>,

    pub token_program: Program<'info, System>,
}

#[error_code]
enum ErrorCode {
    #[msg("Amount must be greater than 0")]
    InvalidAmount,
}