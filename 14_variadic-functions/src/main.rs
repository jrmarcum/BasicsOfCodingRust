fn sum(nums: &[i32]) {
    let display: Vec<String> = nums.iter().map(|n| n.to_string()).collect();
    print!("[{}] ", display.join(" "));
    let total: i32 = nums.iter().sum();
    println!("{}", total);
}

fn main() {
    sum(&[1, 2]);
    sum(&[1, 2, 3]);

    let nums = vec![1, 2, 3, 4];
    sum(&nums);
}
