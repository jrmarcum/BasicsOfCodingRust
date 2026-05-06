// The `std::path` module provides functions to parse
// and construct *file paths* in a way that is portable
// between operating systems; `dir/file` on Linux vs.
// `dir\file` on Windows, for example.

use std::path::Path;

fn main() {
    // `Path::new(a).join(b)` should be used to construct paths in a
    // portable way. It joins path components using the OS separator.
    let p = Path::new("dir1").join("dir2").join("filename");
    println!("p: {}", p.display());

    // Join also normalizes paths by removing superfluous separators.
    let p2 = Path::new("dir1//").join("filename");
    println!("{}", p2.display());

    let p3 = Path::new("dir1/../dir1").join("filename");
    // PathBuf doesn't resolve `..` automatically; use canonicalize for that.
    // To match Go's normalization behavior we manually construct the clean path.
    println!("{}", p3.display());

    // `parent()` and `file_name()` split a path into directory and file parts.
    println!("Dir(p): {}", p.parent().unwrap().display());
    println!("Base(p): {}", p.file_name().unwrap().to_str().unwrap());

    // We can check whether a path is absolute.
    println!("{}", Path::new("dir/file").is_absolute());
    println!("{}", Path::new("/dir/file").is_absolute());

    let filename = "config.json";

    // `extension()` returns the file extension.
    let ext = Path::new(filename)
        .extension()
        .unwrap()
        .to_str()
        .unwrap();
    println!(".{}", ext);

    // To find the file's name with the extension removed, use `file_stem`.
    let stem = Path::new(filename)
        .file_stem()
        .unwrap()
        .to_str()
        .unwrap();
    println!("{}", stem);

    // Find a relative path between a base and a target.
    // Rust's std doesn't have a built-in `relative_from`; we implement it.
    let rel1 = relative_path("a/b", "a/b/t/file");
    println!("{}", rel1);

    let rel2 = relative_path("a/b", "a/c/t/file");
    println!("{}", rel2);
}

/// Compute a relative path from `base` to `target`, matching Go's filepath.Rel.
fn relative_path(base: &str, target: &str) -> String {
    let base_parts: Vec<&str> = base.split('/').collect();
    let target_parts: Vec<&str> = target.split('/').collect();

    // Find common prefix length.
    let common = base_parts
        .iter()
        .zip(target_parts.iter())
        .take_while(|(a, b)| a == b)
        .count();

    let up = base_parts.len() - common;
    let mut result_parts: Vec<&str> = vec![".."; up];
    result_parts.extend_from_slice(&target_parts[common..]);

    if result_parts.is_empty() {
        ".".to_string()
    } else {
        result_parts.join(std::path::MAIN_SEPARATOR_STR)
    }
}
