## CURRENT SESSION: bull-lima
- **Identity**: bull-lima (Sun Mar 29 12:42:22 UTC 2026)
- **Agent**: gemini (gemini-2.5-flash)
- **Persona**: rust-architect
- **Environment**: PATH re-hydrated from .env.agent

# r-jai: Agent Operational Guide (Heart)

## Current Session Context
- **Toolchain Status:** Rust 1.94 (needs `source .env.agent`)
- **Active Phase:** Phase 0 (Infrastructure Verification & Alignment)
- **Previous Agent:** `bull-lima` (Environment Handoff Fixer)

## Operational Rules
1. **Re-hydration:** At the start of every session, run:
   ```bash
   [ -f .env.agent ] && source .env.agent
   ```
2. **Snapshotting:** If you install new system tools in `/root`, add `!!NEEDS_SNAPSHOT!!` to `progress.txt` and signal the human.
3. **No Magic:** All path or environment changes MUST be appended to `.env.agent`.

## Troubleshooting
- **Missing `cargo`:** If re-hydration fails, Rust is not in the current image. Check `specs/000-infrastructure.md` for re-install instructions.
- **Syscall Reference:** Use `/root/mothership/references/syscalls_x86_64.md` for mapping `rustix` or `libc` calls.
