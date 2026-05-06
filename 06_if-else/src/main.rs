fn main() {
    if 7 % 2 == 0 {
        println!("7 is even");
    } else {
        println!("7 is odd");
    }

    if 8 % 4 == 0 {
        println!("8 is divisible by 4");
    }

    let num = 9;
    if num < 0 {
        println!("{} is negative", num);
    } else if num < 10 {
        println!("{} has 1 digit", num);
    } else {
        println!("{} has multiple digits", num);
    }
}
