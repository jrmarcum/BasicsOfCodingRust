fn fmt_slice<T: std::fmt::Display>(s: &[T]) -> String {
    format!("[{}]", s.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))
}

fn index(vs: &[&str], t: &str) -> i32 {
    for (i, &v) in vs.iter().enumerate() {
        if v == t { return i as i32; }
    }
    -1
}

fn include(vs: &[&str], t: &str) -> bool {
    index(vs, t) >= 0
}

fn any(vs: &[&str], f: impl Fn(&str) -> bool) -> bool {
    for &v in vs { if f(v) { return true; } }
    false
}

fn all(vs: &[&str], f: impl Fn(&str) -> bool) -> bool {
    for &v in vs { if !f(v) { return false; } }
    true
}

fn filter<'a>(vs: &[&'a str], f: impl Fn(&str) -> bool) -> Vec<&'a str> {
    vs.iter().filter(|&&v| f(v)).copied().collect()
}

fn map_strs(vs: &[&str], f: impl Fn(&str) -> String) -> Vec<String> {
    vs.iter().map(|&v| f(v)).collect()
}

fn main() {
    let strs = vec!["peach", "apple", "pear", "plum"];

    println!("{}", index(&strs, "pear"));
    println!("{}", include(&strs, "grape"));
    println!("{}", any(&strs, |v| v.starts_with('p')));
    println!("{}", all(&strs, |v| v.starts_with('p')));
    println!("{}", fmt_slice(&filter(&strs, |v| v.contains('e'))));
    println!("{}", fmt_slice(&map_strs(&strs, |v| v.to_uppercase())));
}
