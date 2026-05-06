#### Create a new file with some data to read.
___
##### Run Command:

`$ mkdir -p ./tmp`

`$ echo "hello" > ./tmp/dat.txt`

`$ echo "rust" >> ./tmp/dat.txt`

___
##### Run Command:

`$ cargo run`

##### Results:

`hello`
`rust`
`5 bytes: hello`
`2 bytes @ 6: ru`
`2 bytes @ 6: ru`
`5 bytes: hello`
___
#### Note: Requires `./tmp/dat.txt` to exist with content `hello\nrust\n`. Rust uses `std::fs::read_to_string` (equivalent to Go's `ioutil.ReadFile`), `File::open` + `Read::read` for byte-level control, `Seek` for repositioning, and `BufReader` for buffered reads. Files close automatically when dropped (RAII), equivalent to Go's `defer f.Close()`. The file content uses `rust` instead of `go` to match the Rust project.
