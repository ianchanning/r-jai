mod mount;

use anyhow::Result;

fn main() -> Result<()> {
    println!("r-jai: booting...");

    let fs_type = "tmpfs"; // No need for null terminator here, CString handles it
    match mount::fsopen_wrapper(fs_type) {
        Ok(fd) => {
            println!("Successfully opened filesystem context for '{}'. FD: {:?}", fs_type, fd);
            // In a real scenario, you'd do more with the fd, and close it.
            // For now, we'll just print and exit.
            // Consider adding a proper close mechanism later.
            Ok(())
        },
        Err(e) => {
            eprintln!("Error opening filesystem context for '{}': {}", fs_type, e);
            Err(e)
        }
    }
}
