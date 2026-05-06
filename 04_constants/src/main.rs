const S: &str = "constant";

/// Format an f64 in Go's `%g` scientific notation style (e.g. `6e+11`).
fn fmt_go_sci(f: f64) -> String {
    let raw = format!("{:e}", f);
    // Rust gives "6e11"; Go gives "6e+11". Insert "+" before positive exponents.
    if let Some(e_pos) = raw.find('e') {
        let mantissa = &raw[..e_pos];
        let exp_str  = &raw[e_pos + 1..];
        match exp_str.parse::<i32>() {
            Ok(exp) if exp >= 0 => format!("{}e+{}", mantissa, exp),
            Ok(exp)             => format!("{}e-{}", mantissa, exp.unsigned_abs()),
            Err(_)              => raw,
        }
    } else {
        raw
    }
}

fn main() {
    println!("{}", S);

    const N: i64 = 500_000_000;
    const D: f64 = 3e20 / N as f64;
    println!("{}", fmt_go_sci(D));
    println!("{}", D as i64);
    println!("{}", f64::sin(N as f64));
}
