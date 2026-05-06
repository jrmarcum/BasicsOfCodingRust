fn plus(a: i32, b: i32) -> i32 {
    a + b
}

fn plus_plus(a: i32, b: i32, c: i32) -> i32 {
    a + b + c
}

fn main() {
    let res = plus(1, 2);
    println!("1+2 = {}", res);

    let res = plus_plus(1, 2, 3);
    println!("1+2+3 = {}", res);
}
