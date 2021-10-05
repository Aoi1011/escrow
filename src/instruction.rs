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
