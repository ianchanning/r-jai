# r-jai: Rust-powered Sandbox for AI Agents

## ⚠️ MANDATORY SETUP (Read Before Anything Else)
The Rust toolchain is installed in `/root/.cargo/bin`, but the shell environment is **not persistent** across agent turns or session restarts until the next template snapshot.

**Every agent must run the following before any cargo command:**
```bash
source $HOME/.cargo/env
```

If `cargo` is still missing, it means the snapshot did not include the toolchain, and it must be re-installed using the instructions in `specs/000-infrastructure.md`.

## Project Overview
A rewrite of the Stanford [jai](https://jai.scs.stanford.edu/) project in Rust. This tool provides a modern, high-performance, and portable sandbox for AI orchestration.

## Documentation Entry Points
- **Architecture & Roadmap:** [PLAN.md](PLAN.md)
- **Phase 0 (Infrastructure):** [specs/000-infrastructure.md](specs/000-infrastructure.md)
- **Phase 1 (Syscall Bridge):** [specs/001-syscalls.md](specs/001-syscalls.md)
- **Current Status:** [progress.txt](progress.txt)

## Development Workflow
1. **Source the Environment:** `source $HOME/.cargo/env`
2. **Build/Check:** `cargo check`
3. **No Hidden State:** Do not modify `.bashrc` or other system files without explicit documentation. If you change something about the environment, update **this file** and `PLAN.md`.
