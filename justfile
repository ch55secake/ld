alias b := build
alias cc := clean
alias c := check
alias cb := cbuild
alias f := fmt
alias t := test

# Run tests in project
test:
    cargo test

# Lint with clippy
lint:
    cargo clippy --

# Run dev build and check rust files
build:
    cargo build --workspace
    just check

# Run clean and then dev build and check rust files
cbuild:
    just clean
    cargo build --workspace
    just check

# Clean and build optimized release binary
build_release:
    just clean
    cargo build --workspace --release

# Format any rust code in the project
fmt:
    cargo fmt --

# Clean target directory
clean:
    cargo clean

# Run cargo check to anaylze compiler errors
check:
    cargo check

# Setup/update pre-commit hooks (optional)
setup_precommit:
    pre-commit install --install-hooks



