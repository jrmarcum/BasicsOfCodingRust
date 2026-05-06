The order of workers starting up and finishing is likely to be different for each invocation due to non-deterministic thread scheduling. Rust's `Vec<JoinHandle>` + `.join()` replaces Go's `sync.WaitGroup`: collecting handles and joining them achieves the same "wait for all" semantics.
___
##### Run Command:

`$ cargo run`

##### Results:

`Worker 1 starting`
`Worker 2 starting`
`Worker 3 starting`
`Worker 4 starting`
`Worker 5 starting`
`Worker 3 done`
`Worker 1 done`
`Worker 2 done`
`Worker 5 done`
`Worker 4 done`
