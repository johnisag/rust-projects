//Introduce the pre-import module of the anchor framework
use anchor_lang::prelude::*;

//The on-chain address of the program
declare_id!("CRbfZBE9mF4HAKXrR23x7bqHNn2WckzzJ2wRaqeZfuPW");

//Instruction processing logic
#[program]
mod anchor_counter {
     use super::*;
     pub fn initialize(ctx: Context<InitializeAccounts>, instruction_data: u64) -> Result<()> {
         ctx.accounts.counter.count = instruction_data;
         Ok(())
     }

     pub fn increment(ctx: Context<UpdateAccounts>) -> Result<()> {
         let counter = &mut ctx.accounts.counter;
         msg!("Previous counter: {}", counter.count);
         counter.count = counter.count.checked_add(1).unwrap();
         msg!("Counter incremented. Current count: {}", counter.count);
         Ok(())
     }
}

//The set of accounts involved in the instruction
#[derive(Accounts)]
pub struct InitializeAccounts<'info> {
     #[account(init, seeds = [b"my_seed", user.key.to_bytes().as_ref()], payer = user, space = 8 + 8)]
     pub counter: Account<'info, Counter>,
     #[account(mut)]
     pub user: Signer<'info>,
     pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateAccounts<'info> {
     #[account(mut)]
     pub counter: Account<'info, Counter>,
     pub user: Signer<'info>,
}

// Custom account type
#[account]
pub struct Counter {
     count: u64
}