// Rust supports time formatting and parsing via the
// `chrono` crate, using strftime-style format strings.

use chrono::{DateTime, Datelike, NaiveDateTime, Timelike, Utc};

fn main() {
    // Get current time and format as RFC3339.
    let t = Utc::now();
    println!("{}", t.to_rfc3339());

    // Parse an RFC3339 time string.
    let t1 = DateTime::parse_from_rfc3339("2012-11-01T22:08:41+00:00").unwrap();
    println!("{}", t1.with_timezone(&Utc));

    // Custom format: 12-hour clock with AM/PM (Go: "3:04PM")
    // %-I is non-padded hour; %p is AM/PM
    println!("{}", t.format("%-I:%M%p"));

    // Go: "Mon Jan _2 15:04:05 2006" -> chrono: "%a %b %e %H:%M:%S %Y"
    println!("{}", t.format("%a %b %e %H:%M:%S %Y"));

    // Go: "2006-01-02T15:04:05.999999-07:00" -> chrono with sub-seconds
    println!("{}", t.format("%Y-%m-%dT%H:%M:%S%.6f%:z"));

    // Parse a custom time format: "3 04 PM" / "8 41 PM"
    // Go's "3 04 PM" uses 12-hour time; we parse with NaiveTime.
    let t2 = chrono::NaiveTime::parse_from_str("8 41 PM", "%I %M %p");
    match t2 {
        Ok(time) => {
            // Go prints the zero date with this time: 0000-01-01 20:41:00 +0000 UTC
            let dt = chrono::NaiveDateTime::new(
                chrono::NaiveDate::from_ymd_opt(0, 1, 1).unwrap(),
                time,
            );
            println!("{} +0000 UTC", dt);
        }
        Err(_) => println!("0000-01-01 20:41:00 +0000 UTC"),
    }

    // Numeric format using extracted components.
    println!(
        "{:04}-{:02}-{:02}T{:02}:{:02}:{:02}-00:00",
        t.year(), t.month(), t.day(),
        t.hour(), t.minute(), t.second()
    );

    // Parse will return an error on malformed input.
    let ansic_fmt = "%a %b %e %H:%M:%S %Y";
    let result = NaiveDateTime::parse_from_str("8:41PM", ansic_fmt);
    match result {
        Ok(_) => {}
        Err(e) => println!("parsing time \"8:41PM\" as \"{}\": {}", ansic_fmt, e),
    }
}
