# Stage 0: Infrastructure Forge (Rust Toolchain)

## Goal
Prepare a specialized sandbox environment equipped with the Rust toolchain (rustup, cargo, rustc, clippy).

## Tasks
- [x] Install the Rust toolchain via `rustup`.
- [ ] Configure environment variables in `.bashrc` or a persistent profile.
- [ ] Verify installation with `rustc --version` and `cargo --version`.
- [ ] (Self-Termination) Signal completion for the "Great Freeze" (Template Saving).

## Instructions for the Identity
1. Use `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable`.
2. Ensure you source the environment: `source $HOME/.cargo/env`.
3. Test a simple `cargo build` on the existing boilerplate.
4. If successful, update `progress.txt` with: "PHASE ZERO COMPLETE: Environment ready for snapshotting."

## Verification
- `cargo --version` must return a valid version.
- `clippy` must be available.
