build:
    cargo +nightly build

debug:
    env RUST_LOG=trace cargo +nightly run --profile unopt

format:
    cargo +nightly fmt

alias b := build
alias d := debug
alias f := format
