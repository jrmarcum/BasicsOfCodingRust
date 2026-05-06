##### Run Command:

`$ cargo run`

##### Results:

`Response status: 200 OK`
`<!DOCTYPE html>`
`<html>`
`  <head>`
`    <meta charset="utf-8">`
___
#### Note: Requires network access. Output varies depending on the server response at the time of the request. The `reqwest` crate's `blocking::get` corresponds to Go's `http.Get`. The response status format matches Go's `resp.Status` output (`200 OK`).
