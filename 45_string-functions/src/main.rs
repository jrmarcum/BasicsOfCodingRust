// Rust's standard library provides many useful string
// methods directly on `str` and `String`. Here are
// equivalents for Go's `strings` package functions.

fn main() {
    // In Rust, string operations are methods on &str / String,
    // rather than functions in a separate package.

    println!("Contains:  {}", "test".contains("es"));
    println!("Count:     {}", "test".matches('t').count());
    println!("HasPrefix: {}", "test".starts_with("te"));
    println!("HasSuffix: {}", "test".ends_with("st"));
    // find() returns Some(byte_index); here index of 'e' is 1
    println!("Index:     {}", "test".find('e').unwrap_or(usize::MAX));
    println!("Join:      {}", ["a", "b"].join("-"));
    println!("Repeat:    {}", "a".repeat(5));
    // replace with count -1 in Go means replace all
    println!("Replace:   {}", "foo".replace('o', "0"));
    // replacen(old, new, 1) replaces only the first occurrence
    println!("Replace:   {}", "foo".replacen('o', "0", 1));
    // split and collect into a Vec, then debug-print
    let parts: Vec<&str> = "a-b-c-d-e".split('-').collect();
    println!("Split:     [{}]", parts.join(" "));
    println!("ToLower:   {}", "TEST".to_lowercase());
    println!("ToUpper:   {}", "test".to_uppercase());

    // len() returns byte length (same as Go's len for ASCII)
    println!("Len: {}", "hello".len());
    // Index a byte by position; byte at index 1 of "hello" is b'e' = 101
    println!("Char: {}", "hello".as_bytes()[1]);
}
