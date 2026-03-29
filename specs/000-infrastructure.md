# Stage 0: Infrastructure Forge (Rust Toolchain)

## Goal
Prepare a specialized sandbox environment equipped with the Rust toolchain (rustup, cargo, rustc, clippy).

## Tasks
- [x] Install the Rust toolchain via `rustup`.
- [x] Configure environment variables in `.bashrc` or a persistent profile.
- [x] Verify installation with `rustc --version` and `cargo --version`.
- [x] (Self-Termination) Signal completion for the "Great Freeze" (Template Saving).

## Instructions for the Identity
1. Use `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --no-modify-path --default-toolchain stable`.
2. Ensure you source the environment: `source $HOME/.cargo/env`.
3. Test a simple `cargo build` on the existing boilerplate.
4. If successful, update `progress.txt` with: "PHASE ZERO COMPLETE: Environment ready for snapshotting."

## Verification
- `cargo --version` must return a valid version.
- `clippy` must be available.

## Agent Handoff & Persistence (RALPH_ALIGNMENT v1.0)
**⚠️ MANDATORY PROTOCOL:** 
1. **Persistence:** The Rust toolchain in `/root/.cargo/bin` is ephemeral. Every agent **MUST** ensure the environment is re-hydrated by checking for or updating `.env.agent` in the project root.
   - To persist: `echo 'source $HOME/.cargo/env' >> .env.agent`
2. **Snapshot Signal:** If the toolchain is missing or was just installed, the agent MUST add the following marker to `progress.txt`:
   - `!!NEEDS_SNAPSHOT!!`
3. **Verification:** Always run `source .env.agent` before any `cargo` command.
