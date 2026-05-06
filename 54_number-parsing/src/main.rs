// Parsing numbers from strings is a basic but common task
// in many programs. Here's how to do it in Rust.

fn main() {
    // strconv.ParseFloat("1.234", 64) -> "1.234".parse::<f64>()
    let f: f64 = "1.234".parse().unwrap();
    println!("{}", f);

    // strconv.ParseInt("123", 0, 64) -> "123".parse::<i64>()
    let i: i64 = "123".parse().unwrap();
    println!("{}", i);

    // strconv.ParseInt("0x1c8", 0, 64) -> i64::from_str_radix(stripped, 16)
    let d = i64::from_str_radix("1c8", 16).unwrap();
    println!("{}", d);

    // strconv.ParseUint("789", 0, 64) -> "789".parse::<u64>()
    let u: u64 = "789".parse().unwrap();
    println!("{}", u);

    // strconv.Atoi("135") -> "135".parse::<i32>() (or i64)
    let k: i32 = "135".parse().unwrap();
    println!("{}", k);

    // Parse functions return an error on bad input.
    let e = "wat".parse::<i32>();
    match e {
        Ok(_) => {}
        Err(err) => println!("strconv.Atoi: parsing \"wat\": {}", err),
    }
}
