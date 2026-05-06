// Reading and writing files are basic tasks needed for
// many Rust programs. First we'll look at some examples
// of reading files.

use std::fs::File;
use std::io::{BufRead, BufReader, Read, Seek, SeekFrom};

fn check<T>(result: std::io::Result<T>) -> T {
    result.expect("file operation failed")
}

fn main() {
    // The most basic file reading task: load the entire
    // contents into memory.
    // Go: ioutil.ReadFile -> std::fs::read_to_string
    let dat = check(std::fs::read_to_string("./tmp/dat.txt"));
    print!("{}", dat);

    // Open a file to get a File handle for more control.
    let mut f = check(File::open("./tmp/dat.txt"));

    // Read some bytes from the beginning of the file.
    let mut b1 = [0u8; 5];
    let n1 = check(f.read(&mut b1));
    println!("{} bytes: {}", n1, std::str::from_utf8(&b1[..n1]).unwrap());

    // Seek to a known location and read from there.
    let o2 = check(f.seek(SeekFrom::Start(6)));
    let mut b2 = [0u8; 2];
    let n2 = check(f.read(&mut b2));
    println!("{} bytes @ {}: {}", n2, o2, std::str::from_utf8(&b2[..n2]).unwrap());

    // Read at least N bytes (io.ReadAtLeast equivalent).
    let o3 = check(f.seek(SeekFrom::Start(6)));
    let mut b3 = [0u8; 2];
    let mut total = 0;
    while total < 2 {
        let n = check(f.read(&mut b3[total..]));
        if n == 0 { break; }
        total += n;
    }
    println!("{} bytes @ {}: {}", total, o3, std::str::from_utf8(&b3[..total]).unwrap());

    // Rewind to beginning (Seek(0, 0) equivalent).
    check(f.seek(SeekFrom::Start(0)));

    // Buffered reader for efficient small reads (bufio.NewReader + Peek equivalent).
    let mut r4 = BufReader::new(f);
    // Read first 5 bytes without advancing — peek via fill_buf.
    {
        let buf = check(r4.fill_buf());
        let peek: Vec<u8> = buf[..5].to_vec();
        println!("5 bytes: {}", std::str::from_utf8(&peek).unwrap());
    }
    // File closes automatically when `r4` goes out of scope (RAII).
}
