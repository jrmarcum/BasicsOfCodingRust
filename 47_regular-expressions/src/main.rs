// Rust supports regular expressions via the `regex` crate.
// Here are examples of common regexp-related tasks in Rust.

use regex::Regex;

fn main() {
    // Test whether a pattern matches a string.
    let matched = Regex::new(r"p([a-z]+)ch").unwrap().is_match("peach");
    println!("{}", matched);

    // For repeated use, compile the regex once.
    let r = Regex::new(r"p([a-z]+)ch").unwrap();

    // Match test.
    println!("{}", r.is_match("peach"));

    // Find the first match string.
    println!("{}", r.find("peach punch").map(|m| m.as_str()).unwrap_or(""));

    // Find the start and end indexes of the first match.
    if let Some(m) = r.find("peach punch") {
        println!("[{} {}]", m.start(), m.end());
    }

    // FindStringSubmatch: returns the whole match and submatches.
    let input1 = "peach punch";
    if let Some(caps) = r.captures(input1) {
        let matches: Vec<&str> = caps.iter()
            .filter_map(|m| m.map(|m| m.as_str()))
            .collect();
        println!("{:?}", matches);
    }

    // FindStringSubmatchIndex: indexes of match and submatches.
    if let Some(caps) = r.captures(input1) {
        let mut indexes = Vec::new();
        for m in caps.iter().flatten() {
            indexes.push(m.start());
            indexes.push(m.end());
        }
        println!("{:?}", indexes);
    }

    // Find all matches.
    let input2 = "peach punch pinch";
    let all_matches: Vec<&str> = r.find_iter(input2)
        .map(|m| m.as_str())
        .collect();
    println!("{:?}", all_matches);

    // All submatch indexes.
    let all_idx: Vec<Vec<usize>> = r.captures_iter(input2)
        .map(|caps| {
            caps.iter().flatten()
                .flat_map(|m| [m.start(), m.end()])
                .collect()
        })
        .collect();
    println!("{:?}", all_idx);

    // Limit number of matches.
    let limited: Vec<&str> = r.find_iter(input2)
        .take(2)
        .map(|m| m.as_str())
        .collect();
    println!("{:?}", limited);

    // Match on a string directly (byte-level, same as is_match).
    println!("{}", r.is_match("peach"));

    // MustCompile equivalent: Regex::new(...).unwrap()
    let r = Regex::new(r"p([a-z]+)ch").unwrap();
    println!("{}", r.as_str());

    // Replace subsets of strings with other values.
    println!("{}", r.replace_all("a peach", "<fruit>"));

    // Replace with a closure (bytes::ToUpper equivalent).
    let result = r.replace_all("a peach", |caps: &regex::Captures| {
        caps[0].to_uppercase()
    });
    println!("{}", result);
}
