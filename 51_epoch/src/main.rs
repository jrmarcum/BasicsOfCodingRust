// A common requirement in programs is getting the number
// of seconds, milliseconds, or nanoseconds since the Unix
// epoch. Here's how to do it in Rust.

use chrono::Utc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

fn main() {
    // Use SystemTime::now() for epoch-relative values,
    // and chrono for display formatting.
    let now = Utc::now();
    let sys_now = SystemTime::now();

    let duration_since_epoch = sys_now
        .duration_since(UNIX_EPOCH)
        .expect("time went backwards");

    let secs = duration_since_epoch.as_secs();
    let nanos = duration_since_epoch.as_nanos() as i64;
    let millis = nanos / 1_000_000;

    // Print current time.
    println!("{}", now);

    // Print seconds, milliseconds, and nanoseconds since epoch.
    println!("{}", secs);
    println!("{}", millis);
    println!("{}", nanos);

    // Convert seconds back to a datetime.
    let from_secs = UNIX_EPOCH + Duration::from_secs(secs);
    let from_secs_chrono: chrono::DateTime<Utc> = from_secs.into();
    println!("{}", from_secs_chrono);

    // Convert nanoseconds back to a datetime.
    let from_nanos = UNIX_EPOCH + Duration::from_nanos(nanos as u64);
    let from_nanos_chrono: chrono::DateTime<Utc> = from_nanos.into();
    println!("{}", from_nanos_chrono);
}
