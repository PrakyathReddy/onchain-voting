use anchor_lang::prelude::*;

declare_id!("AMbx16jhMdsJ9k2GvmaL3SXBkS7hD1nYUsHyRCYSxWUt");

#[program]
pub mod onchain_voting {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
