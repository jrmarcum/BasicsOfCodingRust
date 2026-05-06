fn zeroval(ival: i32) {
    let _ival = 0;
    let _ = ival;
}

fn zeroptr(iptr: &mut i32) {
    *iptr = 0;
}

fn main() {
    let mut i = 1;
    println!("initial: {}", i);

    zeroval(i);
    println!("zeroval: {}", i);

    zeroptr(&mut i);
    println!("zeroptr: {}", i);

    println!("pointer: {:p}", &i);
}
