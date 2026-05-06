// Rust does not have a `defer` keyword. Instead, Rust uses
// RAII (Resource Acquisition Is Initialization): cleanup
// happens automatically when a value goes out of scope via
// the `Drop` trait. This is deterministic and guaranteed.

use std::fs::File;
use std::io::Write;

// A generic guard that runs a closure when dropped.
// This mimics Go's `defer` pattern.
struct DeferGuard<F: FnMut()>(F);

impl<F: FnMut()> Drop for DeferGuard<F> {
    fn drop(&mut self) {
        (self.0)();
    }
}

fn create_file(p: &str) -> File {
    println!("creating");
    std::fs::create_dir_all("./tmp").unwrap();
    File::create(p).expect("failed to create file")
}

fn write_file(f: &mut File) {
    println!("writing");
    writeln!(f, "data").expect("failed to write");
}

fn main() {
    let mut f = create_file("./tmp/defer.txt");

    // Register a deferred close using DeferGuard.
    // The guard will run when it goes out of scope at the end of main,
    // after `write_file` has finished — matching Go's defer behavior.
    //
    // In idiomatic Rust, `File` closes automatically when dropped (RAII);
    // the DeferGuard here makes the ordering explicit, just like Go's defer.
    let _guard = DeferGuard(|| println!("closing"));

    write_file(&mut f);

    // `_guard` drops here at the end of main, printing "closing"
    // Output order: creating, writing, closing
}
