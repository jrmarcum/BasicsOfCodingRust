// Rust offers extensive support for times and durations
// via the `chrono` crate. Here are some examples.

use chrono::{DateTime, Datelike, Duration, TimeZone, Timelike, Utc};

fn main() {
    // Get the current time.
    let now = Utc::now();
    println!("{}", now);

    // Build a specific time: 2009-11-17 20:34:58.651387237 UTC
    // chrono uses nanoseconds for sub-second precision.
    let then: DateTime<Utc> = Utc
        .with_ymd_and_hms(2009, 11, 17, 20, 34, 58)
        .unwrap()
        + Duration::nanoseconds(651387237);
    println!("{}", then);

    // Extract components.
    println!("{}", then.year());
    println!("{}", then.format("%B")); // Month name
    println!("{}", then.day());
    println!("{}", then.hour());
    println!("{}", then.minute());
    println!("{}", then.second());
    println!("{}", then.nanosecond());
    println!("{}", then.timezone());

    // Weekday (chrono Debug format: Tue, Wed, etc.)
    println!("{:?}", then.weekday());

    // Compare two times.
    println!("{}", then < now);  // then.Before(now)
    println!("{}", then > now);  // then.After(now)
    println!("{}", then == now); // then.Equal(now)

    // Duration between two times.
    let diff = now.signed_duration_since(then);
    let total_secs = diff.num_seconds().abs();
    let hours = diff.num_hours();
    let minutes = (total_secs % 3600) / 60;
    let secs = total_secs % 60;
    // nanosecond remainder within the last second
    let nanos = diff.num_nanoseconds().unwrap_or(0).abs() % 1_000_000_000;
    println!("{}h{}m{}.{}s", hours, minutes, secs, nanos);

    // Duration in various units.
    let total_hours = diff.num_hours() as f64
        + (diff.num_minutes() % 60) as f64 / 60.0
        + (total_secs % 60) as f64 / 3600.0;
    println!("{}", total_hours);
    let total_minutes = diff.num_minutes() as f64
        + (diff.num_seconds() % 60) as f64 / 60.0;
    println!("{}", total_minutes);
    println!("{}", diff.num_seconds() as f64);
    println!("{}", diff.num_nanoseconds().unwrap_or(0));

    // Add/subtract duration from then.
    println!("{}", then + diff);
    println!("{}", then - diff);
}
