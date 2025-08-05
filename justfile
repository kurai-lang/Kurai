# Run and see the llvm ir output
run:
    cargo run --bin test

# Builds the release binary out
build:
    @echo "Checking Rust toolchain.."
    @bash -c 'toolchain=$(rustup show active-toolchain | cut -d" " -f1); if [ "$toolchain" != "stable-x86_64-unknown-linux-gnu" ]; then echo "Switching to stable..."; rustup default stable; fi'

    cargo build --release

# Installs vyn into system
install: build
    bash $(realpath install.sh)

output:
    $(realpath ./vync/exec)
