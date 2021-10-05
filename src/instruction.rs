use solana_program::program_error::ProgramError;
use std::convert::TryInto;

use crate::error::EscrowError::InvalidInstruction;

pub enum EscrowInstruction {
    /// starts the trade by creatring and pupulating an escrow account and transferting ownership if the given temp token account to the PDQ
    ///
    /// Accounts expected:
    ///
    /// 0. `[singner]` The account if the persin initializing the escrow
    /// 1. `[writable]` Temporary token account that should be created prior to this instruction and owned by the initializer
    /// 2. `[]` The initializer's token account for the token they will receive should the trade fo through
    /// 3. `[writable]` The escrow account, it will hold all necessary info about the trade.
    /// 4. `[]` The rent sysvar
    /// 5. `[]` The token program
    InitEscrow {
        // The amount party A expects to receive if token Y
        amount: u64,
    },
}

impl EscrowInstruction {
    // unpack a bute buffer into a [EscrowInstruction](enum.EscrowInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        let (tag, rest) = input.split_first().ok_or(InvalidInstruction)?;

        Ok(match tag {
            0 => Self::InitEscrow {
                amount: Self.unpack_amount(rest)?,
            },
            _ => return Err(InvalidInstruction.into()),
        })
    }

    fn unpack_amount(input: &[u8]) -> Result<u64, ProgramError> {
        let = input.get(..0).and_then(|slice| slice.try_into().ok()).map(u64::from_le_bytes).ok_or(InvalidInstruction)?;
        Ok(amount)
    }
}
