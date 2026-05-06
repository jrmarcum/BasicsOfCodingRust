// Unit testing is an important part of writing principled Rust programs.
// The built-in test framework provides `#[test]` attributes and the
// `cargo test` command runs tests.
//
// For the sake of demonstration, the code under test and the tests live
// in the same file. Typically test code lives alongside the code it tests,
// in a `#[cfg(test)]` module at the bottom of the file.

/// Integer minimum: returns the smaller of `a` and `b`.
fn int_min(a: i32, b: i32) -> i32 {
    if a < b {
        a
    } else {
        b
    }
}

fn main() {
    // Tests run via `cargo test`, not `cargo run`.
    println!("Run `cargo test` to execute the test suite.");
    println!("Run `cargo test -- --nocapture` to see output during tests.");
}

#[cfg(test)]
mod tests {
    use super::*;

    // A basic test for IntMin.
    #[test]
    fn test_int_min_basic() {
        let ans = int_min(2, -2);
        assert_eq!(ans, -2, "int_min(2, -2) = {}; want -2", ans);
    }

    // Writing tests can be repetitive, so it's idiomatic to use a
    // *table-driven style*, where test inputs and expected outputs are
    // listed in a table and a single loop walks over them.
    #[test]
    fn test_int_min_table_driven() {
        let tests = vec![
            (0, 1, 0),
            (1, 0, 0),
            (2, -2, -2),
            (0, -1, -1),
            (-1, 0, -1),
        ];

        for (a, b, want) in tests {
            let ans = int_min(a, b);
            assert_eq!(
                ans, want,
                "int_min({}, {}) = {}, want {}",
                a, b, ans, want
            );
        }
    }
}
