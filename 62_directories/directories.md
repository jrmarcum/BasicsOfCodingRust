##### Run Command:

`$ cargo run`

##### Results:

`Listing subdir/parent`
`  child true`
`  file2 false`
`  file3 false`
`Listing subdir/parent/child`
`  file4 false`
`Visiting subdir`
`  subdir true`
`  subdir\file1 false`
`  subdir\parent true`
`  subdir\parent\child true`
`  subdir\parent\child\file4 false`
`  subdir\parent\file2 false`
`  subdir\parent\file3 false`

___
#### Note: On Windows, path separators in the "Visiting subdir" output appear as `\` instead of `/`. The directory is created in and cleaned up from the current working directory at the time of running (`cargo run` runs from the package root). Unlike Go's `filepath.Walk`, Rust's `fs::read_dir` is not recursive by default — a helper function `visit_dir` provides the recursive walk.
