Our running program shows the 5 jobs being executed by various workers. The program takes ~2 seconds despite 5 seconds of total work because 3 workers run concurrently. Worker and job assignment order may vary due to thread scheduling. `Arc<Mutex<Receiver>>` shares the job channel across workers; dropping the sender closes the channel, signaling workers to stop.
___
##### Run Command:

`$ cargo run`

##### Results:

`worker 1 started  job 1`
`worker 2 started  job 2`
`worker 3 started  job 3`
`worker 1 finished job 1`
`worker 1 started  job 4`
`worker 2 finished job 2`
`worker 2 started  job 5`
`worker 3 finished job 3`
`worker 1 finished job 4`
`worker 2 finished job 5`
