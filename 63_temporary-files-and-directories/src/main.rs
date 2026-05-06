// Throughout program execution, we often want to create
// data that isn't needed after the program exits.
// *Temporary files and directories* are useful for this
// purpose since they don't pollute the file system over time.

use std::io::Write;
use std::path::Path;

fn main() {
    // The easiest way to create a temporary file is by calling
    // `tempfile::NamedTempFile::new()`. It creates a file and opens it
    // for reading and writing. The file is placed in the system's default
    // temporary directory.
    let mut f = tempfile::NamedTempFile::new().unwrap();

    // Display the name of the temporary file. On Windows the directory
    // will likely be in `%TEMP%`. The file name contains a random component
    // to ensure concurrent calls always create different file names.
    println!("Temp file name: {}", f.path().display());

    // We can write some data to the file.
    f.write_all(&[1, 2, 3, 4]).unwrap();

    // The file is automatically deleted when `f` is dropped at the end of scope.

    // If we intend to write many temporary files, we may prefer to create a
    // temporary *directory*. `tempfile::TempDir::new()` returns a handle
    // to a temporary directory that is deleted when dropped.
    let dname = tempfile::TempDir::new().unwrap();
    println!("Temp dir name: {}", dname.path().display());

    // Now we can synthesize temporary file names by combining them
    // with our temporary directory.
    let fname = dname.path().join("file1");
    std::fs::write(&fname, &[1u8, 2]).unwrap();

    // Both `f` and `dname` are dropped here, cleaning up automatically.
}
