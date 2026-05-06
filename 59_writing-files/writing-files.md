#### Try running the file-writing code.
___
##### Run Command:

`$ cargo run`

##### Results:

`wrote 5 bytes`
`wrote 7 bytes`
`wrote 9 bytes`
___
#### Then check the contents of the written files.
___
##### Run Command:

`$ cat ./tmp/dat1.txt`

##### Results:

`hello`
`rust`
___
##### Run Command:

`$ cat ./tmp/dat2.txt`

##### Results:

`some`
`writes`
`buffered`
___
#### Note: Rust uses `std::fs::write` (equivalent to Go's `ioutil.WriteFile`), `File::create` for granular writes, and `BufWriter` for buffered output. Files close automatically when dropped (RAII), replacing Go's `defer f.Close()`. `f.sync_all()` is equivalent to `f.Sync()`. The `dat1.txt` file contains `hello\nrust\n` (using `rust` instead of Go's `go`).
