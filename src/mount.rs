// src/mount.rs

use anyhow::{Result, Context};
use rustix::mount::{FsOpenFlags, fsopen};
use rustix::fd::OwnedFd; // Only OwnedFd is needed
use std::ffi::{CStr, CString};

/// Creates a new filesystem context for a new mount.
///
/// This function wraps the `fsopen` syscall to create a new filesystem context
/// for a mount operation. It takes a filesystem type string (e.g., "ext4", "tmpfs")
/// and returns an `OwnedFd` representing the new filesystem context.
///
/// # Arguments
/// * `fs_name` - The name of the filesystem type to open.
///
/// # Returns
/// A `Result` containing the `OwnedFd` of the new filesystem context on success,
/// or an `anyhow::Error` on failure.
pub fn fsopen_wrapper(fs_name: &str) -> Result<OwnedFd> { // Changed return type to OwnedFd
    let fs_name_cstring = CString::new(fs_name)
        .context("Filesystem name contained null bytes")?;
    let fs_name_cstr = CStr::from_bytes_with_nul(fs_name_cstring.as_bytes_with_nul())
        .map_err(|e| anyhow::anyhow!("Internal error creating CStr: {}", e))?;

    let fd = fsopen(fs_name_cstr, FsOpenFlags::empty())
        .context(format!("Failed to open filesystem context for '{}'", fs_name))?;

    Ok(fd)
}

// TODO: Implement fsconfig, fsmount, move_mount, mount_setattr
