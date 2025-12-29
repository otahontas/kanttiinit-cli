# Run cargo formatter
format:
    cargo fmt --all

# Run clippy linter
lint:
    cargo clippy --all-targets --all-features -- -D warnings

# Run tests
test:
    cargo test --all-features --verbose
