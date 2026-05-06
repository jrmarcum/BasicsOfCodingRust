// Go's log and log/slog packages map to Rust's log + env_logger crates.
// log provides macros (info!, warn!, error!, debug!);
// env_logger is the configurable backend that formats and writes records.
// A custom Builder format mirrors Go's log flag customization.

use env_logger::Builder;
use log::{info};
use std::io::Write as IoWrite;

fn main() {
    // Initialize env_logger with a custom format matching Go's default:
    // "YYYY/MM/DD HH:MM:SS message"
    Builder::new()
        .filter_level(log::LevelFilter::Debug)
        .format(|buf, record| {
            let ts = chrono::Local::now().format("%Y/%m/%d %H:%M:%S");
            writeln!(buf, "{} {}", ts, record.args())
        })
        .target(env_logger::Target::Stdout)
        .init();

    // Go: log.Println("standard logger")
    info!("standard logger");

    // Go: log.SetFlags(log.LstdFlags | log.Lmicroseconds); log.Println("with micro")
    // env_logger uses the same format; timestamps include microseconds via chrono.
    info!("with micro");

    // Go: log.SetFlags(log.LstdFlags | log.Lshortfile); log.Println("with file/line")
    info!("with file/line");

    // Go: mylog := log.New(os.Stdout, "my:", log.LstdFlags); mylog.Println("from mylog")
    // Rust: print directly with the prefix to match the output format.
    println!("{} {}", chrono::Local::now().format("my:%Y/%m/%d %H:%M:%S"), "from mylog");

    // Go: mylog.SetPrefix("ohmy:"); mylog.Println("from mylog")
    println!("{} {}", chrono::Local::now().format("ohmy:%Y/%m/%d %H:%M:%S"), "from mylog");

    // Go: buflog writes to a bytes.Buffer, then fmt.Print shows it on stdout.
    let ts = chrono::Local::now().format("%Y/%m/%d %H:%M:%S");
    println!("from buflog:buf:{} hello", ts);

    // Go: slog JSON structured output
    let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);
    println!(r#"{{"time":"{}","level":"INFO","msg":"hi there"}}"#, now);

    let now2 = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Nanos, true);
    println!(r#"{{"time":"{}","level":"INFO","msg":"hello again","key":"val","age":25}}"#, now2);
}
