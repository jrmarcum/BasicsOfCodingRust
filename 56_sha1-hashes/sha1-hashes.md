#### Running the program computes the hash and prints it in a human-readable hex format.
___
##### Run Command:

`$ cargo run`

##### Results:

`sha1 this string`
`cf23df2207d99a74fbe169e3eba035e633b65d94`
___
#### Rust uses the `sha1` crate (implementing the `Digest` trait) and the `hex` crate. Go's pattern of `sha1.New()` + `h.Write(bytes)` + `h.Sum(nil)` maps to `Sha1::new()` + `h.update(bytes)` + `h.finalize()`. The resulting hex string is identical to Go's `%x` format output.
___
#### Note: If you need cryptographically secure hashes, research hash strength carefully. SHA1 is not considered secure for cryptographic purposes.
