fn fact(n: u64) -> u64 {
    if n == 0 {
        return 1;
    }
    n * fact(n - 1)
}

fn main() {
    println!("{}", fact(7));
}
