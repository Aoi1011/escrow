# Solana Escrow

## Detail 
### Alice's transaction
1. create empty account owned by token program
2. initialize empty account as Alice's X token account
3. transfer X tokens from Alice's main X token account to her temporary X token account
4. create empty account owned by escrow program
5. initialize empty account as escrow state and transfer temporary X token account ownership to PDA

## Environment Setup

1. Install Rust from https://www.rust-lang.org/tools/install
2. Install Solana from https://docs.solana.com/cli/install-solana-cli-tools#use-solanas-install-tool

## Build and test for program compiled natively

```
$ cargo build
$ cargo test
```

## Build and test the program compiled for BPF

```
$ cargo build-bpf
$ cargo test-bpf
```

## Code Structure

```
.
├─ src
│  ├─ lib.rs -> registering modules
│  ├─ entrypoint.rs -> entrypoint to the program
│  ├─ instruction.rs -> program API, (de)serializing instruction data
│  ├─ processor.rs -> program logic
│  ├─ state.rs -> program objects, (de)serializing state
│  ├─ error.rs -> program specific errors
├─ .gitignore
├─ Cargo.lock
├─ Cargo.toml
├─ Xargo.toml
```

## Flow of a program

1. Someone calls the entrypoint
2. The entrypoint forwards the arguments to the processor
3. The processor asks instruction.rs to decode the instruction_data argument from the entrypoint function
4. Using the decoded data, the processor will now decide which processing function to use to process the request
5. The processor may use state.rs to encode state into or decode the state of an account which has been passed into the entrypoint


## References

1.  https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/

new token X: BpMy6GjcANwmLmvCAUuTtfHiMqVjAb6QkTowcGgjjwgk
token x account: 6N33FCFbNadD8HepibU7CrsrznroZa6djA9TCWAFzA4k

new token Y: Bh1NVMXDcnK2YUeazBk2CmCMgi4dbjnF3RfnBXaqxoFk
token y account: T7sXEByb6V9AEcBnHWZwu4WJzoboC1itRT1jJsR6mDX

programid: H63SHjKvrVbnViaGTFp95i98jnUR3yk7zkMgVfnG9zVR

115,125,35,186,91,207,15,150,103,120,79,250,95,207,125,173,91,13,76,88,220,209,72,199,54,173,170,0,41,185,177,182,44,53,215,84,107,35,0,91,51,178,124,185,226,172,18,82,181,187,66,17,219,48,49,88,118,155,224,156,206,172,69,50

