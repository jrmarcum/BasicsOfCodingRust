// Sometimes our Rust programs need to spawn other processes.
// `std::process::Command` is the Rust equivalent of Go's `os/exec` package.

use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    // We'll start with a simple command that prints the current date.
    // On Windows we use `cmd /C date /T`; on Unix `date`.
    #[cfg(windows)]
    let date_output = Command::new("cmd")
        .args(["/C", "date /T"])
        .output()
        .expect("failed to run date");

    #[cfg(unix)]
    let date_output = Command::new("date")
        .output()
        .expect("failed to run date");

    println!("> date");
    print!("{}", String::from_utf8_lossy(&date_output.stdout));

    // Demonstrate handling a command that exits with a non-zero code.
    // On Windows, `cmd /C exit 1` exits with code 1.
    #[cfg(windows)]
    let err_result = Command::new("cmd").args(["/C", "exit", "1"]).status();
    #[cfg(unix)]
    let err_result = Command::new("date").arg("-x").status();

    match err_result {
        Ok(status) if !status.success() => {
            println!("command exit rc = {}", status.code().unwrap_or(-1));
        }
        Ok(_) => {}
        Err(e) => println!("failed executing: {}", e),
    }

    // Next we pipe data to an external process on its stdin and read stdout.
    // On Windows we use `findstr` (similar to grep); on Unix `grep`.
    #[cfg(windows)]
    let mut grep_cmd = Command::new("findstr")
        .arg("hello")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn findstr");

    #[cfg(unix)]
    let mut grep_cmd = Command::new("grep")
        .arg("hello")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .expect("failed to spawn grep");

    // Write to stdin, close it, then read stdout.
    {
        let stdin = grep_cmd.stdin.as_mut().unwrap();
        stdin
            .write_all(b"hello grep\ngoodbye grep")
            .expect("failed to write to stdin");
    }

    let grep_out = grep_cmd.wait_with_output().expect("failed to wait on grep");
    println!("> grep hello");
    print!("{}", String::from_utf8_lossy(&grep_out.stdout));

    // Spawn a shell command string using the platform shell.
    #[cfg(windows)]
    let ls_output = Command::new("cmd")
        .args(["/C", "dir"])
        .output()
        .expect("failed to run dir");

    #[cfg(unix)]
    let ls_output = Command::new("bash")
        .args(["-c", "ls -a -l -h"])
        .output()
        .expect("failed to run ls");

    #[cfg(windows)]
    println!("> dir");
    #[cfg(unix)]
    println!("> ls -a -l -h");

    print!("{}", String::from_utf8_lossy(&ls_output.stdout));
}
