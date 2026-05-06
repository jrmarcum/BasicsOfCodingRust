// Rust provides base64 encoding/decoding via the `base64` crate.

use base64::{engine::general_purpose, Engine as _};

fn main() {
    // The string we'll encode/decode.
    let data = "abc123!?$*&()'-=@~";

    // Encode using the standard base64 encoder.
    // Go: b64.StdEncoding.EncodeToString([]byte(data))
    let s_enc = general_purpose::STANDARD.encode(data.as_bytes());
    println!("{}", s_enc);

    // Decode back from standard base64.
    let s_dec = general_purpose::STANDARD.decode(&s_enc).unwrap();
    println!("{}", String::from_utf8(s_dec).unwrap());
    println!();

    // Encode using the URL-safe base64 encoder.
    // Go: b64.URLEncoding.EncodeToString([]byte(data))
    let u_enc = general_purpose::URL_SAFE.encode(data.as_bytes());
    println!("{}", u_enc);

    // Decode back from URL-safe base64.
    let u_dec = general_purpose::URL_SAFE.decode(&u_enc).unwrap();
    println!("{}", String::from_utf8(u_dec).unwrap());
}
