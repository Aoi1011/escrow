use anchor_lang::prelude::*;

pub mod instructions;
pub mod state;

declare_id!("FLuSqw7vR8RJUQ8kDvnp3BxqJ8vUqqz7Zx4AvpQzEKe2");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}
