struct Rect {
    width:  i32,
    height: i32,
}

impl Rect {
    fn area(&self) -> i32 {
        self.width * self.height
    }

    fn perim(&self) -> i32 {
        2 * self.width + 2 * self.height
    }
}

fn main() {
    let r = Rect { width: 10, height: 5 };
    println!("area:  {}", r.area());
    println!("perim: {}", r.perim());

    let rp = &r;
    println!("area:  {}", rp.area());
    println!("perim: {}", rp.perim());
}
