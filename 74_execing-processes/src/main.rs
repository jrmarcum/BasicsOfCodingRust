// Sometimes we want to completely replace the current process with another.
// Go achieves this with `syscall.Exec` (Unix only), which replaces the running
// process image. On Unix, Rust can do the same via `std::os::unix::process::CommandExt::exec()`.
//
// On Windows, there is no true `exec` syscall. Instead we spawn a child process,
// wait for it to finish, and exit with the same code — effectively handing off execution.

use std::process::Command;

fn main() {
    // On Unix, `ls -a -l -h` lists files. On Windows we use `cmd /C dir`.
    // We try `ls` first (available in Git Bash / WSL); fall back to `cmd /C dir`.
    #[cfg(unix)]
    {
        use std::os::unix::process::CommandExt;
        // `exec` replaces the current process — nothing after this line runs if successful.
        let err = Command::new("ls")
            .args(["-a", "-l", "-h"])
            .exec();
        // Only reached if exec failed.
        eprintln!("exec error: {}", err);
        std::process::exit(1);
    }

    #[cfg(windows)]
    {
        // On Windows: spawn the process and exit with its exit code.
        let status = Command::new("cmd")
            .args(["/C", "dir"])
            .status()
            .unwrap_or_else(|e| {
                eprintln!("Failed to execute process: {}", e);
                std::process::exit(1);
            });

        std::process::exit(status.code().unwrap_or(1));
    }
}
