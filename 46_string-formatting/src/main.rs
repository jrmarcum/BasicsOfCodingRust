use std::fmt;

struct Point {
    x: i32,
    y: i32,
}

// Display matches Go's %v output: {1 2}
impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{} {}}}", self.x, self.y)
    }
}

/// Format f64 in Go's %e style: 6 decimal places, explicit sign, 2-digit exponent.
fn fmt_go_e(f: f64) -> String {
    let raw = format!("{:.6e}", f);
    if let Some(e_pos) = raw.find('e') {
        let mantissa = &raw[..e_pos];
        let exp_str  = &raw[e_pos + 1..];
        match exp_str.parse::<i32>() {
            Ok(exp) if exp >= 0 => format!("{}e+{:02}", mantissa, exp),
            Ok(exp)             => format!("{}e-{:02}", mantissa, exp.unsigned_abs()),
            Err(_)              => raw,
        }
    } else {
        raw
    }
}

fn fmt_go_e_upper(f: f64) -> String {
    fmt_go_e(f).to_uppercase()
}

fn main() {
    let p = Point { x: 1, y: 2 };

    // %v  — default struct format
    println!("{}", p);

    // %+v — struct with field names
    println!("{{x:{} y:{}}}", p.x, p.y);

    // %#v — Go-style type + fields (Rust uses PascalCase: Point, not point)
    println!("main.Point{{x:{}, y:{}}}", p.x, p.y);

    // %T  — type name (Rust equivalent: short type name)
    let full = std::any::type_name::<Point>();
    let short = full.split("::").last().unwrap_or(full);
    println!("{}", short);

    // %t  — bool
    println!("{}", true);

    // %d  — integer
    println!("{}", 123);

    // %b  — binary
    println!("{:b}", 14u32);

    // %c  — character
    println!("{}", char::from_u32(33).unwrap());

    // %x  — hex lowercase
    println!("{:x}", 456u32);

    // %f  — float with 6 decimal places
    println!("{:.6}", 78.9f64);

    // %e  — scientific lowercase (6 decimal places, explicit sign, 2-digit exp)
    println!("{}", fmt_go_e(123400000.0f64));

    // %E  — scientific uppercase
    println!("{}", fmt_go_e_upper(123400000.0f64));

    // %s  — string
    println!("{}", "\"string\"");

    // %q  — quoted string
    println!("{:?}", "\"string\"");

    // %x on string bytes — hex encoding
    let hex_str: String = "hex this".bytes().map(|b| format!("{:02x}", b)).collect();
    println!("{}", hex_str);

    // %p  — pointer
    println!("{:p}", &p);

    // Width
    println!("|{:6}|{:6}|", 12, 345);
    println!("|{:6.2}|{:6.2}|", 1.2f64, 3.45f64);
    println!("|{:<6.2}|{:<6.2}|", 1.2f64, 3.45f64);
    println!("|{:>6}|{:>6}|", "foo", "b");
    println!("|{:<6}|{:<6}|", "foo", "b");

    // Sprintf equivalent
    let s = format!("a {}", "string");
    println!("{}", s);

    // Fprintf to stderr
    eprintln!("an {}", "error");
}
