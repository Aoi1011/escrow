use anchor_lang::prelude::*;

pub mod error;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod anchor_escrow {
    use super::*;

    pub fn initiEscrow(ctx: Context<InitEscrow>) -> Result<()> {
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitEscrow<'info> {}
