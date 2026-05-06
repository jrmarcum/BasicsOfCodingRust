// URLs provide a uniform way to locate resources.
// Here's how to parse URLs in Rust using the `url` crate.

use url::Url;

fn main() {
    // We'll parse this example URL, which includes a scheme,
    // authentication info, host, port, path, query params,
    // and query fragment.
    let s = "postgres://user:pass@host.com:5432/path?k=v#f";

    // Parse the URL.
    let u = Url::parse(s).expect("failed to parse URL");

    // Accessing the scheme.
    println!("{}", u.scheme());

    // Authentication info: username and password.
    // Go's u.User prints "user:pass"; we replicate that.
    let username = u.username();
    let password = u.password().unwrap_or("");
    println!("{}:{}", username, password);
    println!("{}", username);
    println!("{}", password);

    // Host contains both hostname and port.
    println!("{}", u.host_str().unwrap_or(""));
    // Separate host and port (net.SplitHostPort equivalent).
    println!("{}", u.host_str().unwrap_or(""));
    println!("{}", u.port().map(|p| p.to_string()).unwrap_or_default());

    // Path and fragment.
    println!("{}", u.path());
    println!("{}", u.fragment().unwrap_or(""));

    // Raw query string.
    println!("{}", u.query().unwrap_or(""));

    // Parse query params into a map.
    // Go prints map[k:[v]]; we match that structure.
    let pairs: Vec<(String, String)> = u.query_pairs().into_owned().collect();
    print!("map[");
    for (i, (k, v)) in pairs.iter().enumerate() {
        if i > 0 { print!(" "); }
        print!("{}:[{}]", k, v);
    }
    println!("]");

    // Access first value for key "k".
    let k_val = u.query_pairs()
        .find(|(k, _)| k == "k")
        .map(|(_, v)| v.into_owned())
        .unwrap_or_default();
    println!("{}", k_val);
}
