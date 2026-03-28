# Stage 3: Workspace & Overlays

## Goal
Implement host-directory mapping and COW storage.

## Tasks
- [ ] Implement workspace mounting (Bind mounts).
- [ ] Implement `overlayfs` for "Golden Image" protection.
- [ ] Ensure proper permission handling for mounted volumes.

## Verification
- Host files should be visible inside the sandbox.
- Changes made inside should be persistent if requested.
