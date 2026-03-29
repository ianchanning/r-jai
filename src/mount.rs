use anyhow::{Result, Context};
use rustix::mount::{FsOpenFlags, fsopen};
use rustix::fd::OwnedFd; // Import BorrowedFd
use std::ffi::{CStr, CString};
use libc::{self, c_int, c_long};

// Define the syscall number for fsconfig
#[cfg(target_arch = "x86_64")]
const SYS_FSCONFIG: c_long = 433; // For x86_64

// Define FSCONFIG_SET_STRING command
const FSCONFIG_SET_STRING: c_int = 8;

// Define the syscall number for fsmount
#[cfg(target_arch = "x86_64")]
const SYS_FSMOUNT: c_long = 432; // For x86_64

/// Creates a new detached mount object from a filesystem context.
///
/// This function directly calls the `fsmount` syscall.
///
/// # Arguments
/// * `fs_fd` - The file descriptor for the filesystem context.
/// * `flags` - Mount flags (e.g., FSMOUNT_CLOEXEC).
/// * `attr_flags` - Mount attributes (e.g., MOUNT_ATTR_RDONLY).
///
/// # Returns
/// A `Result` containing the `OwnedFd` of the new detached mount object on success,
/// or an `anyhow::Error` on failure.
pub fn fsmount_wrapper(
    fs_fd: c_int,
    flags: u32,
    attr_flags: u32,
) -> Result<OwnedFd> {
    let res = unsafe {
        libc::syscall(
            SYS_FSMOUNT,
            fs_fd,
            flags,
            attr_flags,
        )
    };

    if res == -1 {
        let errno = unsafe { *libc::__errno_location() };
        Err(anyhow::anyhow!("fsmount syscall failed: {}", std::io::Error::from_raw_os_error(errno)))
    } else {
        Ok(unsafe { OwnedFd::from_raw_fd(res as c_int) })
    }
}

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
pub fn fsopen_wrapper(fs_name: &str) -> Result<OwnedFd> {
    let fs_name_cstring = CString::new(fs_name)
        .context("Filesystem name contained null bytes")?;
    let fs_name_cstr = CStr::from_bytes_with_nul(fs_name_cstring.as_bytes_with_nul())
        .map_err(|e| anyhow::anyhow!("Internal error creating CStr: {}", e))?;

    let fd = fsopen(fs_name_cstr, FsOpenFlags::empty())
        .context(format!("Failed to open filesystem context for '{}'", fs_name))?;

    Ok(fd)
}

/// Sets a string configuration parameter for a filesystem context using libc::syscall.
///
/// This function directly calls the `fsconfig` syscall with `FSCONFIG_SET_STRING` command.
/// It is inherently `unsafe` and carefully wrapped to provide a safe API.
///
/// # Arguments
/// * `fs_fd` - The raw file descriptor of the filesystem context.
/// * `key` - The key of the configuration parameter (e.g., "source", "mode").
/// * `value` - The string value to set for the parameter.
///
/// # Returns
/// A `Result` indicating success or an `anyhow::Error` on failure.
pub fn fsconfig_set_string(
    fs_fd: c_int, // Changed to c_int for raw fd
    key: &str,
    value: &str,
) -> Result<()> {
    let c_key = CString::new(key)
        .context(format!("Key '{}' contained null bytes", key))?;
    let c_value = CString::new(value)
        .context(format!("Value '{}' contained null bytes", value))?;

    let key_ptr = c_key.as_ptr();
    let value_ptr = c_value.as_ptr();

    let res = unsafe {
        libc::syscall(
            SYS_FSCONFIG,
            fs_fd,
            FSCONFIG_SET_STRING,
            key_ptr,
            value_ptr,
            0, // aux is 0 for FSCONFIG_SET_STRING
        )
    };

    if res == -1 {
        let errno = unsafe { *libc::__errno_location() };
        Err(anyhow::anyhow!("fsconfig syscall failed: {}", std::io::Error::from_raw_os_error(errno)))
    } else {
        Ok(())
    }
}

// TODO: Implement fsconfig, fsmount, move_mount, mount_setattr.
