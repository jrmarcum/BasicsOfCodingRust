#### Run the TCP server in the background, then send data and capture the response. On Windows use PowerShell; on Unix use netcat.
___
##### Run Command:

`$ cargo run`

On Unix:

`$ echo "Hello from netcat" | nc localhost 8090`

On Windows PowerShell:

`$ "Hello from netcat" | nc localhost 8090`

##### Results:

`ACK: HELLO FROM NETCAT`
___
#### Note: This mirrors Go's `net.Listen("tcp", ":8090")` TCP server example. Each incoming connection is handled in a separate `thread::spawn` call, analogous to Go's `go handleConnection(conn)`. The server reads one newline-terminated line from the client, converts it to uppercase, and responds with `ACK: <MESSAGE>`. The server runs until interrupted with Ctrl+C.
