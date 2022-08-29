# Escrow Program

## Overview

### Alice's transaction (Initialize)
Alice (Initializer) can send a transaction to the escrow program to initialize the Vault. 
In this transaction, two new accounts: Vault and EscrowAccount, will be created and toekns (Token A) to be exchanged will be transfered from Initializer to Vault. 

### Cancel
Alice (Initializer) can also send a transaction to escrow program to cancel the demand of escrow. 
The tokens will be transfered back to Alice and Vault and EscrowAccount will be closed in this case. 

### Exchange
Bob (Taker) can send a transaction to the escrow to exchange Token B for Token A. Firstly, Token B will be transfered from Bob to Alice.
Afterward, Token A kept in the vault will be transfered to Bob. Finally, Vault and EscrowAccount will be closed.


## Build, Deploy and Test

Install dependencies
```bash
yarn
```

Get program id
```bash
anchor keys list
```

Build the program
```bash
anchor build
```

Test the program
```bash
anchor test
```

## References

1. https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
2. https://hackmd.io/@ironaddicteddog/solana-anchor-escrow#Overview

