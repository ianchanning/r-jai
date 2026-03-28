mod mount;

use anyhow::Result;
use rustix::fd::AsRawFd; // Need to import AsRawFd

fn main() -> Result<()> {
    println!("r-jai: booting...");

    let fs_type = "tmpfs";
    match mount::fsopen_wrapper(fs_type) {
        Ok(fd) => {
            println!("Successfully opened filesystem context for '{}'. FD: {:?}", fs_type, fd);

            // Call the libc-based fsconfig_set_string
            mount::fsconfig_set_string(
                fd.as_raw_fd(), // Use as_raw_fd() to get c_int
                "source",
                fs_type,
            )?;

            println!("Successfully configured filesystem context for '{}' with source '{}'.", fs_type, fs_type);

            Ok(())
        },
        Err(e) => {
            eprintln!("Error with filesystem context for '{}': {}", fs_type, e);
            Err(e)
        }
    }
}
