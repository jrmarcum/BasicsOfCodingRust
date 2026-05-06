fn fmt_slice<T: std::fmt::Display>(s: &[T]) -> String {
    format!("[{}]", s.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" "))
}

fn main() {
    let a: [i32; 5] = [0; 5];
    println!("emp: {}", fmt_slice(&a));

    let mut a = a;
    a[4] = 100;
    println!("set: {}", fmt_slice(&a));
    println!("get: {}", a[4]);

    println!("len: {}", a.len());

    let b: [i32; 5] = [1, 2, 3, 4, 5];
    println!("dcl: {}", fmt_slice(&b));

    let mut two_d: [[i32; 3]; 2] = [[0; 3]; 2];
    for i in 0..2 {
        for j in 0..3 {
            two_d[i][j] = (i + j) as i32;
        }
    }
    let rows: Vec<String> = two_d.iter().map(|row| fmt_slice(&row[..])).collect();
    println!("2d:  [{}]", rows.join(" "));
}
