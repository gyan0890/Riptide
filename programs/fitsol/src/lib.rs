use anchor_lang::prelude::*;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");
//Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS
// <Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS, Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS, Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS>
#[program]
pub mod fitsol {

    use super::*;

    pub fn lock_in(ctx: Context<Deposit>, amount: u64) -> ProgramResult {
        let ix = anchor_lang::solana_program::system_instruction::transfer(
            &ctx.accounts.from.key(),
            &ctx.accounts.to.key(),
            amount,
        );

        anchor_lang::solana_program::program::invoke(
            &ix,
            &[
                ctx.accounts.from.to_account_info(),
                ctx.accounts.to.to_account_info(),
            ],
        );
        Ok(())
    }

    pub fn create_challenge(ctx:Context<Create>, creator: String, amount: u64, duration: i64, num_steps: i64) -> ProgramResult {
        let challenge = &mut ctx.accounts.challenge;
        challenge.state = "Created".to_string();
        challenge.creator = creator;
        challenge.lock_in = amount; 
        challenge.duration = duration;
        challenge.num_steps = num_steps;
        Ok(())
    }

    pub fn join_challenge(ctx:Context<Join>, participant: String) -> ProgramResult {
        
        let challenge = &mut ctx.accounts.challenge;
        challenge.state = "Joined".to_string();
        challenge.participants = participant;
        Ok(())
    }


}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer=user, space= 8+64+64+64)]
    pub challenge: Account<'info, Challenge>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Join<'info> {
    #[account(mut)]
    pub challenge: Account<'info, Challenge>,
}

#[derive(Accounts)]
pub struct Deposit<'info> {
    #[account(mut)]
    pub from: Signer<'info>,
    pub to:Account<'info, Treasury>,
}

#[account]
pub struct Challenge {
    pub id: i64,
    pub state: String,
    pub creator: String,
    // pub name: String,
    // pub description: String,
    pub lock_in: u64,
    pub participants: String,
    pub start_time: i64,
    pub duration: i64,
    pub winners:Vec<String>,
    pub num_steps: i64, 
}

#[account]
pub struct Treasury {
    pub balance: i64
}