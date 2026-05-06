#### To try out our line filter, first make a file with a few lowercase lines. If "lines.txt" file exists delete it before doing the exercise.
___
##### Run Command:

`$ echo 'hello'   > ./tmp/lines.txt`

`$ echo 'filter' >> ./tmp/lines.txt`

___
#### Then use the line filter to get uppercase lines.
___
##### Run Command:

`$ cat ./tmp/lines.txt | cargo run`

##### Results:

`HELLO`
`FILTER`
___
#### Rust uses `std::io::BufRead::lines()` on a locked stdin handle, equivalent to Go's `bufio.NewScanner(os.Stdin)` with `scanner.Scan()`. Each line is uppercased with `.to_uppercase()`, matching Go's `strings.ToUpper`. Errors are printed to stderr and exit with status 1, matching Go's behavior.
