##### Run Command:

`$ cargo run`

##### Results:

`81,87`
`0.6645600532184904`
`7.123187485356329,8.434115364335547`
`0,28`
`18,82`
`18,82`
___
#### Note: Numbers will vary per run except for the last two lines (fixed seed 42), which always produce the same sequence for a given `StdRng` implementation. Rust uses the `rand` crate; Go uses `math/rand`. Go's `rand.NewSource(42)` maps to `StdRng::seed_from_u64(42)`. The fixed-seed values differ from Go's because Rust's `StdRng` uses a different algorithm than Go's default PRNG. The last two lines will always be identical to each other (same seed repeated), matching Go's behavior.
