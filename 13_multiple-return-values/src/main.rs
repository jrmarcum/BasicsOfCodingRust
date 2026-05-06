fn vals() -> (i32, i32) {
    (3, 7)
}

fn main() {
    let (a, b) = vals();
    println!("{}", a);
    println!("{}", b);

    let (_, c) = vals();
    println!("{}", c);
}
