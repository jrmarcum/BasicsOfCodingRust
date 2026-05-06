// SHA256 hashes are frequently used to compute short identities for binary
// or text blobs. For example, TLS/SSL certificates use SHA256 to compute
// a certificate's signature. Here's how to compute SHA256 hashes in Rust.
//
// Rust implements hash functions via the `sha2` crate (part of the RustCrypto
// project), mirroring Go's `crypto/sha256` package.

use sha2::{Digest, Sha256};

fn main() {
    let s = "sha256 this string";

    // Create a new SHA256 hasher and feed it the input bytes.
    let mut hasher = Sha256::new();
    hasher.update(s.as_bytes());

    // Finalize the hash and get the result as a byte array.
    let result = hasher.finalize();

    // Print the input string and the hex-encoded hash digest.
    println!("{}", s);
    println!("{}", hex::encode(result));
}
