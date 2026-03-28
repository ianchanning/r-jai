mod mount;

use anyhow::Result;

fn main() -> Result<()> {
    println!("r-jai: booting...");

    let fs_type = "tmpfs";
    match mount::fsopen_wrapper(fs_type) {
        Ok(fd) => {
            println!("Successfully opened filesystem context for '{}'. FD: {:?}", fs_type, fd);

            // Temporarily removed fsconfig_set_string call due to rustix API issues.
            // Will re-implement with nix or a more stable rustix API version.

            Ok(())
        },
        Err(e) => {
            eprintln!("Error with filesystem context for '{}': {}", fs_type, e);
            Err(e)
        }
    }
}
