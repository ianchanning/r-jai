# Stage 1: New Mount API Wrappers

## Goal
Implement safe Rust wrappers for the Linux "New Mount API" syscalls using `rustix`.

## Tasks
- [x] Implement fsopen(fsname, flags)
- [x] Implement fsconfig(fd, cmd, key, value, flags)
- [x] Implement `fsmount(fd, flags, attr_flags)`
- [x] Implement `move_mount(from_fd, from_path, to_fd, to_path, flags)`
- [ ] Implement `mount_setattr(fd, path, flags, attr, size)`

## Verification
- Code should compile on Ubuntu 22.04 (via rustix).
- Test by creating a `tmpfs` mount in a private namespace.
