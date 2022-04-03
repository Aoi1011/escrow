# Solana Escrow

## Environment Setup

1. Install Rust from https://rustup.rs/
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

## References

1.  https://paulx.dev/blog/2021/01/14/programming-on-solana-an-introduction/
