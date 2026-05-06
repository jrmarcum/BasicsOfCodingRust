##### Run Command:

`$ cargo run`

##### Results:

`Temp file name: C:\Users\...\AppData\Local\Temp\.tmpXXXXXX`
`Temp dir name: C:\Users\...\AppData\Local\Temp\.tmpXXXXXX`

___
#### Note: Temp file and directory names will vary with each run. The `tempfile` crate automatically deletes temporary files and directories when their handles are dropped (go out of scope), unlike Go where explicit `defer os.Remove(...)` calls are needed.
