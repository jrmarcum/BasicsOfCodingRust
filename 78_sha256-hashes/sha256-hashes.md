#### Running the program computes the hash and prints it in a human-readable hex format.
___
##### Run Command:

`$ cargo run`

##### Results:

`sha256 this string`
`1af1dfa857bf1d8814fe1af8983c18080019922e557f15a8a0d3db739d77aacb`
___
#### Note: The `sha2` crate from the RustCrypto project mirrors Go's `crypto/sha256` package. `Sha256::new()` corresponds to `sha256.New()`, `hasher.update()` to `h.Write()`, and `hasher.finalize()` to `h.Sum(nil)`. The `hex::encode` function produces the same lowercase hex string as Go's `fmt.Printf("%x", bs)`. The SHA256 digest of `"sha256 this string"` is deterministic and matches Go's output exactly.
