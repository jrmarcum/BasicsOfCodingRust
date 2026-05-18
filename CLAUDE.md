# Basics of Coding Rust — Project Context

## Purpose

Multi-language comparative study of programming syntax, language simplicity,
lines of code required, and compile/run performance. Rust is one of several
languages implemented against the same set of example programs, enabling
direct side-by-side comparison.

## Licensing Summary

This project contains two tiers of content with different licenses:

- **CC BY 3.0** — lesson files and code examples adapted from "Go by Example"
  by Mark McGranaghan (https://github.com/mmcgrana/gobyexample), via
  "Basics of Coding Go" by Jon Marcum (https://github.com/jrmarcum/BasicsOfCodingGo).
  License: http://creativecommons.org/licenses/by/3.0/

- **CC0 1.0** — original contributions by Jon Marcum (project structure,
  README, comparative-study additions, and any lessons not derived from
  Go by Example). See LICENSE.

Attribution for derived content is provided centrally in README.md and
NOTICE — do **not** add a per-file attribution footer to lesson `.md` files.

## Upstream Reference

BasicsOfCodingGo is tracked as a git submodule at `upstream/basicsofcodinggo`.
Submodule URL: <https://github.com/jrmarcum/BasicsOfCodingGo>

Each lesson in this repo mirrors a corresponding lesson in that repo. Read
the upstream `.go` file and `.md` file for the original logic and expected
output, then translate to idiomatic Rust.

To initialize the submodule after cloning:
```
git submodule update --init --recursive
```

## Project Structure

```
BasicsOfCodingRust/
├── CLAUDE.md              — this file; canonical project context for Claude sessions
├── Cargo.toml             — workspace manifest listing all lesson crates
├── LICENSE                — CC0 (applies to Jon Marcum's original contributions)
├── NOTICE                 — attribution notice for CC BY 3.0 derived content
├── README.md              — project overview, attribution section, license table
├── upstream/
│   └── basicsofcodinggo/  — git submodule: BasicsOfCodingGo (source of truth)
└── ##_topic-name/
    ├── Cargo.toml         — crate manifest with [package] and [dependencies]
    ├── src/
    │   └── main.rs        — runnable Rust source for the lesson
    └── topic-name.md      — lesson explanation (run command + expected output)
```

Lessons are numbered with a two-digit prefix (e.g., `01_hello-world`). Numbers
match BasicsOfCodingGo exactly — gaps exist where topics were skipped.

## Running Lessons

Each lesson is a Cargo crate in the workspace. Run from within a lesson folder:

```
cd 01_hello-world
cargo run
```

Or build/run a specific lesson from the workspace root:

```
cargo run -p hello-world
```

## Cargo Workspace

The root `Cargo.toml` declares the workspace and shared `[workspace.dependencies]`.
Individual lesson `Cargo.toml` files reference shared deps with `{ workspace = true }`.

External crates used across lessons:
- `serde` + `serde_json` — JSON encoding/decoding (lesson 48)
- `quick-xml` — XML processing (lesson 49)
- `regex` — regular expressions (lesson 47)
- `rand` — random numbers (lesson 53)
- `chrono` — time formatting/parsing (lessons 50–52)
- `url` — URL parsing (lesson 55)
- `sha1` / `sha2` / `hex` — hashing (lessons 56, 78)
- `base64` — base64 encoding (lesson 57)
- `reqwest` — HTTP client, blocking mode (lesson 69)
- `clap` — command-line argument parsing (lessons 65–66)
- `ctrlc` — signal handling (lesson 76)
- `tempfile` — temporary files and directories (lesson 63)
- `tokio` — async runtime (lesson 71)
- `tera` — text templates (lesson 73)
- `log` + `env_logger` — structured logging (lesson 39)

## Translation Notes

- Go `fmt.Println` → Rust `println!`
- Go `fmt.Printf` → Rust `print!` with format string (using `{}` not `%v`)
- Go goroutines → `std::thread::spawn` or `tokio::spawn`
- Go channels → `std::sync::mpsc` channels
- Go `select` → no native `select!` in std; use `crossbeam-channel` or manual `try_recv` loops
- Go interfaces → Rust `trait` objects (`dyn Trait`)
- Go struct embedding → Rust composition + `Deref` or delegation methods
- Go `defer` → Rust RAII via `Drop` trait or scoped guards
- Go `recover` / `panic` → `std::panic::catch_unwind`
- Go goroutine WaitGroup → `JoinHandle` collection and `.join()`
- Go maps → `std::collections::HashMap`
- Go slices → `Vec<T>` (dynamic) or `&[T]` (slice reference)
- Array/slice debug output format differs: Go uses `[a b c]`, Rust uses `[a, b, c]`
- Map debug output format differs: Go uses `map[k:v]`, Rust uses `{"k": v}`

## .gitignore

The project `.gitignore` covers:

- `target/` — Cargo build output directory
- `*.exe` — Windows binaries (produced by cargo build on Windows)
- `tmp/` — temporary directory used by file I/O lessons (58–60)

## Notes for Future Claude Sessions

- The root `LICENSE` file is CC0 but does **not** cover the derived lesson content.
  Always refer to NOTICE and README for the full picture.
- Lesson `.md` files use a consistent format: optional description line,
  `___` divider, `##### Run Command:`, run command in backticks, blank line,
  `##### Results:`, each output line in backticks. No per-file footer.
- All lessons run with `cargo run` from within the lesson directory. The
  source file is at `src/main.rs` per standard Cargo convention.
- Lesson 68 (testing) uses `cargo test` instead of `cargo run`.
- Lessons 58–60 (reading-files, writing-files, line-filters) use `./tmp/` for
  file I/O. The `tmp/` directory is gitignored and must exist at runtime; for
  lesson 58, `tmp/dat.txt` must be pre-populated (content: `hello\nrust\n`).
- Output format differences between Go and Rust are noted in the `.md`
  description lines where relevant (e.g., array/slice formatting, map output).
- Concurrency lessons (27–37) use `std::thread`, `std::sync::mpsc`, and
  `std::sync::{Mutex, Arc, atomic}`. No external async runtime needed for
  most of these.
