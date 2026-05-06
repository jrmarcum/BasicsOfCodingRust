#### The string encodes to slightly different values with the standard and URL base64 encoders (trailing `+` vs `-`) but they both decode to the original string as desired.
___
##### Run Command:

`$ cargo run`

##### Results:

`YWJjMTIzIT8kKiYoKSctPUB+`
`abc123!?$*&()'-=@~`

`YWJjMTIzIT8kKiYoKSctPUB-`
`abc123!?$*&()'-=@~`
___
#### Rust uses the `base64` crate with `general_purpose::STANDARD` and `general_purpose::URL_SAFE` engines. Go's `b64.StdEncoding.EncodeToString` maps to `general_purpose::STANDARD.encode`; `b64.URLEncoding.EncodeToString` maps to `general_purpose::URL_SAFE.encode`. The output is identical to Go's.
