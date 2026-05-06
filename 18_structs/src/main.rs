use std::fmt;

struct Person {
    name: String,
    age:  i32,
}

impl fmt::Display for Person {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{{{} {}}}", self.name, self.age)
    }
}

impl Person {
    fn new(name: &str) -> Person {
        Person { name: name.to_string(), age: 42 }
    }
}

fn main() {
    println!("{}", Person { name: "Bob".to_string(),   age: 20 });
    println!("{}", Person { name: "Alice".to_string(), age: 30 });
    println!("{}", Person { name: "Fred".to_string(),  age: 0  });
    // Go's `&person{...}` prints with an `&` prefix because it's a pointer.
    println!("&{}", Person { name: "Ann".to_string(), age: 40 });
    println!("&{}", Person::new("Jon"));

    let s = Person { name: "Sean".to_string(), age: 50 };
    println!("{}", s.name);

    let sp = &s;
    println!("{}", sp.age);

    let mut s = s;
    s.age = 51;
    println!("{}", s.age);
}
