use anchor_lang::prelude::*;

declare_id!("HP3qqvaAUHj9cBRBzzNXVe2TNZjWy1iG2ZUUzUT2Bwmu");

#[program]
pub mod borrow_lend {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
