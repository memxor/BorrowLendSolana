use anchor_lang::prelude::*;
use crate::state::UserAcc;
use crate::state::BorrowedToken;
use crate::constants::USER_SEED;

pub fn borrow(ctx: Context<Borrow>, borrow: BorrowedToken) -> Result<()> 
{
    let user = &mut ctx.accounts.user;

    user.borrowed_tokens.push(borrow);

    Ok(())
}

#[derive(Accounts)]
pub struct Borrow<'info>
{
    #[account(mut)]
    pub signer: Signer<'info>,

    #[account(
        mut,
        seeds=[USER_SEED, signer.key().as_ref()],
        bump,
        realloc = 8 + UserAcc::LEN + ((user.lended_tokens.len() + 1) * BorrowedToken::LEN),
        realloc::payer = signer,
        realloc::zero = true)
    ]
    pub user: Account<'info, UserAcc>,

    pub system_program: Program<'info, System>
}