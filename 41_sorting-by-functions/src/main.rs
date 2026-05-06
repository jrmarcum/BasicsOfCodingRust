fn fmt_slice<T: std::fmt::Display>(s: &[T]) -> String {
    format!("[{}]", s.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))
}

fn main() {
    let mut fruits = vec!["peach", "banana", "kiwi"];
    fruits.sort_by(|a, b| a.len().cmp(&b.len()));
    println!("{}", fmt_slice(&fruits));
}
