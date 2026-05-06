// SHA1 hashes are frequently used to compute short
// identities for binary or text blobs. For example,
// the git revision control system uses SHA1s extensively.
// Here's how to compute SHA1 hashes in Rust.

use sha1::{Digest, Sha1};

fn main() {
    let s = "sha1 this string";

    // Create a new SHA1 hasher.
    let mut h = Sha1::new();

    // Update the hasher with the input bytes.
    h.update(s.as_bytes());

    // Finalize the hash and get the result as bytes.
    let bs = h.finalize();

    // SHA1 values are often printed in hex.
    // Use hex::encode to convert the hash bytes to a hex string.
    println!("{}", s);
    println!("{}", hex::encode(bs));
}
