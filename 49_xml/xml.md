##### Run Command:

`$ cargo run`

##### Results:

` <plant id="27">`
`   <name>Coffee</name>`
`   <origin>Ethiopia</origin>`
`   <origin>Brazil</origin>`
` </plant>`
`<?xml version="1.0" encoding="UTF-8"?>`
` <plant id="27">`
`   <name>Coffee</name>`
`   <origin>Ethiopia</origin>`
`   <origin>Brazil</origin>`
` </plant>`
`Plant id=27, name=Coffee, origin=["Ethiopia", "Brazil"]`
` <nesting>`
`   <parent>`
`     <child>`
`       <plant id="27">`
`         <name>Coffee</name>`
`         <origin>Ethiopia</origin>`
`         <origin>Brazil</origin>`
`       </plant>`
`       <plant id="81">`
`         <name>Tomato</name>`
`         <origin>Mexico</origin>`
`         <origin>California</origin>`
`       </plant>`
`     </child>`
`   </parent>`
` </nesting>`
___
#### Note: Rust uses `quick-xml` with `serde` for XML encoding/decoding. Go's `xml.MarshalIndent` and `xml.Unmarshal` map to `quick_xml::se::to_string` and `quick_xml::de::from_str`. The Display output for Plant uses Rust's debug format for the origin Vec: `["Ethiopia", "Brazil"]` vs Go's `[Ethiopia Brazil]`. The XML structure and indentation match Go's output exactly.
