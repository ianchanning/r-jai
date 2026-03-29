# r-jai: Rust-powered Sandbox for AI Agents (RALPH-ALIGNED v1.0)

## ⚠️ MANDATORY SETUP (RALPH_ALIGNMENT)
This project is aligned with the `Safer Ralph` Mothership protocol. 

**Every agent MUST run the following before any work:**
```bash
[ -f .env.agent ] && source .env.agent
```

## The Holy Trinity of State
- **The Intent:** [specs/](specs/) (Topic of Concern)
- **The Strategy:** [IMPLEMENTATION_PLAN.md](IMPLEMENTATION_PLAN.md) (Instruction Pointer)
- **The Heart:** [AGENTS.md](AGENTS.md) (Operational Guide)

## Current Status
See [progress.txt](progress.txt) for the latest work and **!!NEEDS_SNAPSHOT!!** signals.

## Persistence Rule
Do not modify `.bashrc`. All environment mutations (PATH, exports) MUST be appended to **.env.agent**.
```bash
echo 'source $HOME/.cargo/env' >> .env.agent
```
