When we run the program the `"ping"` message is successfully passed from one thread to another via `std::sync::mpsc`. By default, `recv()` blocks until the sender sends a value, matching Go's unbuffered channel behavior. `mpsc::channel()` creates an unbuffered channel; `tx.send()` and `rx.recv()` replace Go's `<-` syntax.
___
##### Run Command:

`$ cargo run`

##### Results:

`ping`
