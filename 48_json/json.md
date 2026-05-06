##### Run Command:

`$ cargo run`

##### Results:

`true`
`1`
`2.34`
`"vector"`
`["apple","peach","pear"]`
`{"apple":5,"lettuce":7}`
`{"Page":1,"Fruits":["apple","peach","pear"]}`
`{"page":1,"fruits":["apple","peach","pear"]}`
`map[num:6.13 strs:["a", "b"]]`
`6.13`
`a`
`{1 ["apple", "peach"]}`
`apple`
`{"apple":5,"lettuce":7}`
___
#### Note: Rust uses `serde` + `serde_json` for JSON encoding/decoding. Go's `json.Marshal` becomes `serde_json::to_string`; `json.Unmarshal` becomes `serde_json::from_str`. Struct field JSON keys are controlled with `#[serde(rename = "...")]` instead of Go's struct tags. The generic map decode output is formatted to match Go's style. Go's `json.NewEncoder(os.Stdout).Encode(d)` adds a trailing newline; the last line is printed without one using `to_string`.
