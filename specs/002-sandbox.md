# Stage 2: Sandbox Isolation

## Goal
Implement the core namespace orchestration for process isolation.

## Tasks
- [ ] Create a `unshare` wrapper for: CLONE_NEWNS, CLONE_NEWUSER, CLONE_NEWPID, CLONE_NEWUTS.
- [ ] Implement `User Namespace` mapping (Host UID -> Sandbox Root).
- [ ] Implement `pivot_root` logic to change the sandbox's root filesystem.
- [ ] Configure `proc` mount inside the PID namespace.

## Verification
- Running a command inside `r-jai` should report a custom hostname.
- Running `whoami` inside should report `root`.
- Running `ps` should only show the sandbox processes.
