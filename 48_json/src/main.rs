// Rust supports JSON encoding and decoding via the
// `serde` and `serde_json` crates.

use serde::{Deserialize, Serialize};
use serde_json::{self, Value};
use std::collections::HashMap;

// We'll use these two structs to demonstrate encoding and
// decoding of custom types below.

// Fields without rename use their Rust name as the JSON key.
#[derive(Serialize, Deserialize, Debug)]
struct Response1 {
    #[serde(rename = "Page")]
    page: i32,
    #[serde(rename = "Fruits")]
    fruits: Vec<String>,
}

// With serde rename, keys become lowercase in JSON.
#[derive(Serialize, Deserialize, Debug)]
struct Response2 {
    page: i32,
    fruits: Vec<String>,
}

fn main() {
    // Encode basic data types to JSON strings.
    println!("{}", serde_json::to_string(&true).unwrap());
    println!("{}", serde_json::to_string(&1).unwrap());
    println!("{}", serde_json::to_string(&2.34).unwrap());
    println!("{}", serde_json::to_string(&"vector").unwrap());

    // Slices and maps encode to JSON arrays and objects.
    let slc = vec!["apple", "peach", "pear"];
    println!("{}", serde_json::to_string(&slc).unwrap());

    // BTreeMap preserves insertion order alphabetically for deterministic output.
    let mut map = std::collections::BTreeMap::new();
    map.insert("apple", 5);
    map.insert("lettuce", 7);
    println!("{}", serde_json::to_string(&map).unwrap());

    // Encode custom struct (field names capitalized via rename).
    let res1 = Response1 {
        page: 1,
        fruits: vec!["apple".into(), "peach".into(), "pear".into()],
    };
    println!("{}", serde_json::to_string(&res1).unwrap());

    // Encode with lowercase JSON keys via serde field names.
    let res2 = Response2 {
        page: 1,
        fruits: vec!["apple".into(), "peach".into(), "pear".into()],
    };
    println!("{}", serde_json::to_string(&res2).unwrap());

    // Decode JSON data into a generic map.
    let byt = r#"{"num":6.13,"strs":["a","b"]}"#;
    let dat: HashMap<String, Value> = serde_json::from_str(byt).unwrap();
    // Go prints map[num:6.13 strs:[a b]]; we match the structure.
    let num = dat["num"].as_f64().unwrap();
    let strs = dat["strs"].as_array().unwrap();
    println!("map[num:{} strs:{:?}]", num, strs.iter().map(|v| v.as_str().unwrap()).collect::<Vec<_>>());

    // Access the num value directly.
    println!("{}", num);

    // Access nested data.
    let str1 = strs[0].as_str().unwrap();
    println!("{}", str1);

    // Decode JSON into a typed struct.
    let str_json = r#"{"page": 1, "fruits": ["apple", "peach"]}"#;
    let res: Response2 = serde_json::from_str(str_json).unwrap();
    println!("{{{} {:?}}}", res.page, res.fruits);
    println!("{}", res.fruits[0]);

    // Stream JSON encoding to stdout (json.NewEncoder equivalent).
    let mut stream_map = std::collections::BTreeMap::new();
    stream_map.insert("apple", 5);
    stream_map.insert("lettuce", 7);
    println!("{}", serde_json::to_string(&stream_map).unwrap());
}
