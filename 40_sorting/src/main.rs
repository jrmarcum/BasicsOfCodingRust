fn fmt_slice<T: std::fmt::Display>(s: &[T]) -> String {
    format!("[{}]", s.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))
}

fn main() {
    let mut strs = vec!["c", "a", "b"];
    strs.sort();
    println!("Strings: {}", fmt_slice(&strs));

    let mut ints = vec![7, 2, 4];
    ints.sort();
    println!("Ints:    {}", fmt_slice(&ints));

    let sorted = ints.windows(2).all(|w| w[0] <= w[1]);
    println!("Sorted:  {}", sorted);
}
