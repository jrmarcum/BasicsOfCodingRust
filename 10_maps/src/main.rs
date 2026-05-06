use std::collections::HashMap;

fn fmt_map(m: &HashMap<String, i32>) -> String {
    let mut pairs: Vec<String> = m.iter()
        .map(|(k, v)| format!("{}:{}", k, v))
        .collect();
    pairs.sort();
    format!("map[{}]", pairs.join(" "))
}

fn main() {
    let mut m: HashMap<String, i32> = HashMap::new();

    m.insert("k1".to_string(), 7);
    m.insert("k2".to_string(), 13);

    println!("map: {}", fmt_map(&m));

    let v1 = m["k1"];
    println!("v1:  {}", v1);

    println!("len: {}", m.len());

    m.remove("k2");
    println!("map: {}", fmt_map(&m));

    let prs = m.contains_key("k2");
    println!("prs: {}", prs);

    let n: HashMap<String, i32> = [
        ("foo".to_string(), 1),
        ("bar".to_string(), 2),
    ].into_iter().collect();
    println!("map: {}", fmt_map(&n));
}
