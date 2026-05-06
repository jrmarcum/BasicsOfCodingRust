use std::collections::HashMap;

fn main() {
    let nums = vec![2, 3, 4];
    let sum: i32 = nums.iter().sum();
    println!("sum: {}", sum);

    for (i, &num) in nums.iter().enumerate() {
        if num == 3 {
            println!("index: {}", i);
        }
    }

    let mut kvs: HashMap<&str, &str> = HashMap::new();
    kvs.insert("a", "apple");
    kvs.insert("b", "banana");
    let mut pairs: Vec<(&&str, &&str)> = kvs.iter().collect();
    pairs.sort_by_key(|&(k, _)| k);
    for (k, v) in pairs {
        println!("{} -> {}", k, v);
    }

    let mut keys: Vec<&&str> = kvs.keys().collect();
    keys.sort();
    for k in keys {
        println!("key: {}", k);
    }

    for (i, c) in "go".chars().enumerate() {
        println!("{} {}", i, c as u32);
    }
}
