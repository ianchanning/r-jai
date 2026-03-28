# r-jai: Rust-powered Sandbox for AI Agents

## Objective
Rewrite the Stanford [jai](https://jai.scs.stanford.edu/) project in Rust to provide a modern, high-performance, and portable sandbox for AI orchestration systems like **Safer Ralph**.

## Key Advantages
- **No Glibc Dependency:** Use `rustix` to talk directly to the Linux kernel. No more "old header" errors on Ubuntu.
- **Static Portability:** A single binary that works across any modern Linux distribution.
- **Ownership Alignment:** Use User Namespaces to ensure sandbox files are owned by the host user, eliminating the need for `sudo` or root-cleanup hacks.
- **Sub-millisecond Overhead:** Bypasses the Docker daemon entirely for instant "bubble" creation.

## Technical Roadmap

### Phase 1: The Syscall Bridge (The "Fix")
- **New Mount API:** Implement type-safe wrappers for `fsopen`, `fsconfig`, `fsmount`, `move_mount`, and `mount_setattr`.
- **Crate Selection:** 
  - `rustix`: For raw, high-performance syscalls.
  - `nix`: For standard POSIX/Linux abstractions.
  - `clap`: For a robust CLI interface mirroring the original `jai`.

### Phase 2: Isolation & Orchestration
- **Namespace Logic:** Implement orchestration for:
  - **Mount**: Private filesystem view.
  - **User**: ID mapping (Host User -> Sandbox Root).
  - **PID**: Process isolation.
  - **UTS**: Custom hostname.
  - **Network**: (Optional) Bridge or No-Network modes.
- **Pivot Root:** Securely transition the process into the new rootfs.

### Phase 3: Storage & Overlays
- **Workspace Integration:** Map host directories into the sandbox with proper permissions.
- **Overlayfs Support:** Implement "copy-on-write" layers so agents can't corrupt the "Golden Image."

### Phase 4: Safer Ralph Integration
- **Rootfs Importer:** A tool to "ingest" existing Docker images into an `r-jai` compatible folder.
- **sandbox.sh Update:** Replace `docker run` calls with `r-jai run`.

## Implementation Strategy
1. **Mimic the "expected" pattern:** Use Rust's `Result<T, E>` to replicate the C++23 `std::expected` flow found in the original source.
2. **Safety First:** Minimize `unsafe` blocks, isolating them strictly to the raw syscall wrappers.
3. **Parity Check:** Ensure `r-jai --init` and `r-jai --run` behave exactly like the original Stanford tool.

## Current Blockers & Research
- Verify `rustix` support for the very newest VFS syscalls (6.13+ extensions).
- Analyze `libmount` requirements for complex overlay layouts.
