// Writing files in Rust follows similar patterns to
// the ones we saw earlier for reading.

use std::fs::{self, File};
use std::io::{BufWriter, Write};

fn check<T>(result: std::io::Result<T>) -> T {
    result.expect("file operation failed")
}

fn main() {
    // Ensure the tmp directory exists.
    check(fs::create_dir_all("./tmp"));

    // Dump a string (or bytes) into a file.
    // Go: ioutil.WriteFile -> fs::write
    let d1 = b"hello\nrust\n";
    check(fs::write("./tmp/dat1.txt", d1));

    // For more granular writes, open a file for writing.
    // Go: os.Create -> File::create
    let mut f = check(File::create("./tmp/dat2.txt"));
    // File closes automatically at end of scope (RAII, like defer f.Close()).

    // Write a byte slice.
    // bytes: 's'=115, 'o'=111, 'm'=109, 'e'=101, '\n'=10
    let d2: &[u8] = &[115, 111, 109, 101, 10];
    let n2 = check(f.write(d2));
    println!("wrote {} bytes", n2);

    // WriteString equivalent.
    let n3 = check(f.write(b"writes\n"));
    println!("wrote {} bytes", n3);

    // Sync to stable storage (f.Sync() equivalent).
    check(f.sync_all());

    // Buffered writer (bufio.NewWriter equivalent).
    let mut w = BufWriter::new(&f);
    let n4 = check(w.write(b"buffered\n"));
    println!("wrote {} bytes", n4);

    // Flush buffered writer.
    check(w.flush());
}
