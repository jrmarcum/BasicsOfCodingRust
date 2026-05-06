##### Run Command:

`$ cargo run`

##### Results:

`Value: some text`
`Value: 5`
`Value: ["Go", "Rust", "C++", "C#"]`
`Name: Jane Doe`
`Name: Mickey Mouse`
`yes `
`no `
`Range: Go Rust C++ C# `
___
#### Note: The `tera` crate uses Jinja2-like syntax (`{{ variable }}`, `{% if %}`, `{% for %}`) instead of Go's `{{.}}` and `{{range}}` syntax. When rendering an array/slice as a plain value (t1 with an array), Tera outputs JSON-style `["Go", "Rust", "C++", "C#"]` rather than Go's `[Go Rust C++ C#]`. The conditional `{% if value %}` treats an empty string as falsy, matching Go's `{{if .}}` behavior.
