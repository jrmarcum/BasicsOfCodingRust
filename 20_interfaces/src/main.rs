use std::f64::consts::PI;

trait Geometry {
    fn area(&self)    -> f64;
    fn perim(&self)   -> f64;
    fn display(&self) -> String;
}

struct Rect   { width: f64, height: f64 }
struct Circle { radius: f64 }

impl Geometry for Rect {
    fn area(&self)  -> f64 { self.width * self.height }
    fn perim(&self) -> f64 { 2.0 * self.width + 2.0 * self.height }
    fn display(&self) -> String {
        format!("{{{} {}}}", self.width as i64, self.height as i64)
    }
}

impl Geometry for Circle {
    fn area(&self)  -> f64 { PI * self.radius * self.radius }
    fn perim(&self) -> f64 { 2.0 * PI * self.radius }
    fn display(&self) -> String {
        format!("{{{}}}", self.radius as i64)
    }
}

fn measure(g: &dyn Geometry) {
    println!("{}", g.display());
    println!("{}", g.area());
    println!("{}", g.perim());
}

fn main() {
    let r = Rect   { width: 3.0, height: 4.0 };
    let c = Circle { radius: 5.0 };
    measure(&r);
    measure(&c);
}
