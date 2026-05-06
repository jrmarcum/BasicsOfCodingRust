// A Rust &str is a UTF-8 encoded byte slice. The language treats strings as
// sequences of Unicode scalar values (chars). Rust's char type corresponds to
// Go's rune — a Unicode code point.

fn examine_rune(r: char) {
    // Rust uses char literals for comparison, exactly like Go's rune literals.
    if r == 't' {
        println!("found tee");
    } else if r == 'ส' {
        println!("found so sua");
    }
}

fn main() {
    // `s` is the Thai word "สวัสดี" (hello).
    let s = "สวัสดี";

    // s.len() returns the byte length, not the character count.
    println!("Len: {}", s.len());

    // Iterate over raw bytes and print each as hex, matching Go's byte loop.
    for b in s.as_bytes() {
        print!("{:x} ", b);
    }
    println!();

    // Count Unicode scalar values (chars), equivalent to utf8.RuneCountInString.
    println!("Rune count: {}", s.chars().count());

    // char_indices() yields (byte_offset, char) pairs — equivalent to Go's
    // range over a string which decodes runes with their byte offsets.
    for (idx, c) in s.char_indices() {
        println!("U+{:04X} '{}' starts at {}", c as u32, c, idx);
    }

    println!();
    println!("Using DecodeRuneInString");

    // Iterate again explicitly, mirroring Go's utf8.DecodeRuneInString loop.
    for (idx, c) in s.char_indices() {
        println!("U+{:04X} '{}' starts at {}", c as u32, c, idx);
        examine_rune(c);
    }
}
