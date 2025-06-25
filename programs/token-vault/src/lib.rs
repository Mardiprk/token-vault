use anchor_lang::prelude::*;

declare_id!("8sNqEgxShvZ4q8t51Jrsg2prbRrcsDVbYKaz5MqwR7bQ");

#[program]
pub mod token_vault {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
