// Rust has several useful functions for working with
// *directories* in the file system via `std::fs`.

use std::fs;
use std::path::Path;

fn main() {
    // Create a new sub-directory in the current working directory.
    fs::create_dir("subdir").unwrap();

    // Use a closure to ensure cleanup even if we panic.
    // (Rust doesn't have `defer`, but the directory is removed at the end.)

    // Helper closure to create a new empty file.
    let create_empty_file = |name: &str| {
        fs::write(name, b"").unwrap();
    };

    create_empty_file("subdir/file1");

    // We can create a hierarchy of directories, including
    // parents with `fs::create_dir_all`. This is similar to `mkdir -p`.
    fs::create_dir_all("subdir/parent/child").unwrap();

    create_empty_file("subdir/parent/file2");
    create_empty_file("subdir/parent/file3");
    create_empty_file("subdir/parent/child/file4");

    // `fs::read_dir` lists directory contents, returning an iterator
    // of `DirEntry` objects. We collect and sort for consistent output.
    let mut entries: Vec<_> = fs::read_dir("subdir/parent")
        .unwrap()
        .map(|e| e.unwrap())
        .collect();
    entries.sort_by_key(|e| e.file_name());

    println!("Listing subdir/parent");
    for entry in &entries {
        let metadata = entry.metadata().unwrap();
        println!("  {} {}", entry.file_name().to_str().unwrap(), metadata.is_dir());
    }

    // List contents of subdir/parent/child.
    let mut child_entries: Vec<_> = fs::read_dir("subdir/parent/child")
        .unwrap()
        .map(|e| e.unwrap())
        .collect();
    child_entries.sort_by_key(|e| e.file_name());

    println!("Listing subdir/parent/child");
    for entry in &child_entries {
        let metadata = entry.metadata().unwrap();
        println!("  {} {}", entry.file_name().to_str().unwrap(), metadata.is_dir());
    }

    // We can also visit a directory *recursively*,
    // including all its sub-directories.
    println!("Visiting subdir");
    visit_dir(Path::new("subdir")).unwrap();

    // Clean up the directory tree.
    fs::remove_dir_all("subdir").unwrap();
}

/// Recursively visit a directory, printing each entry's path and whether it is a directory.
fn visit_dir(dir: &Path) -> std::io::Result<()> {
    println!("  {} {}", dir.display(), dir.is_dir());

    if dir.is_dir() {
        let mut entries: Vec<_> = fs::read_dir(dir)?
            .map(|e| e.unwrap())
            .collect();
        entries.sort_by_key(|e| e.file_name());

        for entry in entries {
            visit_dir(&entry.path())?;
        }
    }

    Ok(())
}
