##### Run Command:

`$ cargo test`

##### Results:

`running 2 tests`
`test tests::test_int_min_basic ... ok`
`test tests::test_int_min_table_driven ... ok`
``
`test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out`
___
#### Note: Rust uses `#[test]` attributes inside a `#[cfg(test)]` module instead of Go's `TestXxx` naming convention. Go's table-driven subtests use `t.Run(name, func)` to create named sub-tests; Rust uses a loop with `assert_eq!` inside a single `#[test]` function. Go's `BenchmarkIntMin` requires the `testing.B` type — Rust benchmarks require nightly (`#[bench]`); on stable Rust use the `criterion` crate for benchmarking.
