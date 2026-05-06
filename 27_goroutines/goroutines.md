When we run this program, we see the output of the blocking call first, then the output of the two threads. The threads' output may be interleaved because threads run concurrently. In Rust, `thread::spawn` replaces Go's `go` keyword; `handle.join().unwrap()` replaces `time.Sleep` as a more robust way to wait for threads to finish.
___
##### Run Command:

`$ cargo run`

##### Results:

`direct : 0`
`direct : 1`
`direct : 2`
`goroutine : 0`
`going`
`goroutine : 1`
`goroutine : 2`
`done`
