use std::time::{SystemTime, UNIX_EPOCH};

fn what_am_i<T: std::any::Any>(i: &T) {
    let type_id = std::any::TypeId::of::<T>();
    if type_id == std::any::TypeId::of::<bool>() {
        println!("I'm a bool");
    } else if type_id == std::any::TypeId::of::<i32>() {
        println!("I'm an i32");
    } else {
        println!("Don't know type {}", std::any::type_name::<T>());
    }
}

fn main() {
    let i = 2;
    print!("Write {} as ", i);
    match i {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => {}
    }

    let secs = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let days = secs / 86400;
    let weekday = (days + 4) % 7;
    match weekday {
        0 | 6 => println!("It's the weekend"),
        _ => println!("It's a weekday"),
    }

    let hour = (secs % 86400) / 3600;
    if hour < 12 {
        println!("It's before noon");
    } else {
        println!("It's after noon");
    }

    what_am_i(&true);
    what_am_i(&1_i32);
    what_am_i(&"hey");
}
