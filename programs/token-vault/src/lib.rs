use anchor_lang::prelude::*;

declare_id!("8sNqEgxShvZ4q8t51Jrsg2prbRrcsDVbYKaz5MqwR7bQ");

#[program]
pub mod token_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn deposit(ctx: Context<Deposit>, amount: u64) -> Result<()> {
        Ok(())
    }

    pub fn withdraw(ctx: Context<Withdraw>, amount: u64) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {}

#[derive(Accounts)]
pub struct Deposit<'info> {}

#[derive(Accounts)]
pub struct Withdraw<'info> {}

#[account]
pub struct Vaule {}

