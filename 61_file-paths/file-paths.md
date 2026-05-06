##### Run Command:

`$ cargo run`

##### Results:

`p: dir1\dir2\filename`
`dir1\filename`
`dir1\..\dir1\filename`
`Dir(p): dir1\dir2`
`Base(p): filename`
`false`
`false`
`.\json`
`config`
`t\file`
`..\c\t\file`

___
#### Note: On Windows, `std::path::Path` uses `\` as the separator, so output differs from Go's Unix-style `/` separators shown in the Go `.md`. Also, `Path::new("/dir/file").is_absolute()` returns `false` on Windows because a leading `/` without a drive letter is not considered absolute. The `..` in `dir1/../dir1` is not resolved by `Path::join` (unlike Go's `filepath.Join` which cleans the path); use `canonicalize()` on an existing path to resolve it. The extension is returned without a leading dot by `Path::extension()`, so `.` is prepended manually to match Go output.
