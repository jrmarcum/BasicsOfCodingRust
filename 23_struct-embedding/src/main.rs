// Rust does not have struct embedding. Instead, composition is used: the inner
// type becomes a named field of the outer struct. Methods of the inner type
// are not automatically promoted; they must be delegated explicitly.
// In Go, `co.num` works directly due to embedding; in Rust the path is
// `co.base.num`.

struct Base {
    num: i32,
}

impl Base {
    fn describe(&self) -> String {
        format!("base with num={}", self.num)
    }
}

// Container uses composition: base is a named field, not an embedded type.
struct Container {
    base: Base,
    str: String,
}

impl Container {
    // Delegate describe() to the inner Base, mirroring Go's promoted method.
    fn describe(&self) -> String {
        self.base.describe()
    }
}

// Trait analogous to Go's describer interface.
trait Describer {
    fn describe(&self) -> String;
}

impl Describer for Container {
    fn describe(&self) -> String {
        self.base.describe()
    }
}

fn main() {
    let co = Container {
        base: Base { num: 1 },
        str: "some name".to_string(),
    };

    // In Rust, inner fields are accessed via the field name: co.base.num.
    println!("co={{num: {}, str: {}}}", co.base.num, co.str);

    // Explicit full path to the inner field.
    println!("also num: {}", co.base.num);

    // Delegated method call.
    println!("describe: {}", co.describe());

    // Use via trait object, analogous to Go's describer interface.
    let d: &dyn Describer = &co;
    println!("describer: {}", d.describe());
}
