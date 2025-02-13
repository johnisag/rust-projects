use anchor_lang::prelude::*;

declare_id!("CRbfZBE9mF4HAKXrR23x7bqHNn2WckzzJ2wRaqeZfuPW");

#[program]
pub mod first_anchor_project {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
