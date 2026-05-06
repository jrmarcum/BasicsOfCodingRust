fn int_seq() -> impl FnMut() -> i32 {
    let mut i = 0;
    move || {
        i += 1;
        i
    }
}

fn main() {
    let mut next_int = int_seq();
    println!("{}", next_int());
    println!("{}", next_int());
    println!("{}", next_int());

    let mut new_ints = int_seq();
    println!("{}", new_ints());
}
