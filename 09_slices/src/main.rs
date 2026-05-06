fn fmt_slice_str(s: &[String]) -> String {
    format!("[{}]", s.join(" "))
}

fn fmt_slice_i32(s: &[i32]) -> String {
    format!("[{}]", s.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))
}

fn main() {
    let mut s: Vec<String> = vec![String::new(); 3];
    println!("emp: {}", fmt_slice_str(&s));

    s[0] = "a".to_string();
    s[1] = "b".to_string();
    s[2] = "c".to_string();
    println!("set: {}", fmt_slice_str(&s));
    println!("get: {}", s[2]);

    println!("len: {}", s.len());

    s.push("d".to_string());
    s.push("e".to_string());
    s.push("f".to_string());
    println!("apd: {}", fmt_slice_str(&s));

    let c = s.clone();
    println!("cpy: {}", fmt_slice_str(&c));

    println!("sl1: {}", fmt_slice_str(&s[2..5]));
    println!("sl2: {}", fmt_slice_str(&s[..5]));
    println!("sl3: {}", fmt_slice_str(&s[2..]));

    let t = vec!["g".to_string(), "h".to_string(), "i".to_string()];
    println!("dcl: {}", fmt_slice_str(&t));

    let mut two_d: Vec<Vec<i32>> = Vec::new();
    for i in 0..3 {
        let inner_len = i + 1;
        let mut inner = Vec::new();
        for j in 0..inner_len {
            inner.push((i + j) as i32);
        }
        two_d.push(inner);
    }
    let rows: Vec<String> = two_d.iter().map(|row| fmt_slice_i32(row)).collect();
    println!("2d:  [{}]", rows.join(" "));
}
