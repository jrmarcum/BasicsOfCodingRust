// Rust offers dynamic content rendering with the `tera` crate, which provides
// a Jinja2-like template language. This mirrors Go's `text/template` package.
//
// Template syntax mapping:
//   Go `{{.}}`              -> Tera `{{ value }}`
//   Go `{{.FieldName}}`     -> Tera `{{ field_name }}`
//   Go `{{if .}}...{{end}}` -> Tera `{% if value %}...{% endif %}`
//   Go `{{range .}}...{{end}}` -> Tera `{% for item in items %}...{% endfor %}`

use tera::{Context, Tera};

fn main() {
    // Create a Tera instance and render templates inline (no files needed).
    let mut tera = Tera::default();

    // t1: render a simple scalar value via `{{ value }}`.
    tera.add_raw_template("t1", "Value: {{ value }}\n").unwrap();

    let mut ctx = Context::new();
    ctx.insert("value", "some text");
    print!("{}", tera.render("t1", &ctx).unwrap());

    let mut ctx = Context::new();
    ctx.insert("value", &5);
    print!("{}", tera.render("t1", &ctx).unwrap());

    let mut ctx = Context::new();
    ctx.insert("value", &["Go", "Rust", "C++", "C#"]);
    print!("{}", tera.render("t1", &ctx).unwrap());

    // t2: access a struct field by name.
    tera.add_raw_template("t2", "Name: {{ name }}\n").unwrap();

    let mut ctx = Context::new();
    ctx.insert("name", "Jane Doe");
    print!("{}", tera.render("t2", &ctx).unwrap());

    let mut ctx = Context::new();
    ctx.insert("name", "Mickey Mouse");
    print!("{}", tera.render("t2", &ctx).unwrap());

    // t3: if/else conditional.
    tera.add_raw_template("t3", "{% if value %}yes {% else %}no {% endif %}\n")
        .unwrap();

    let mut ctx = Context::new();
    ctx.insert("value", "not empty");
    print!("{}", tera.render("t3", &ctx).unwrap());

    let mut ctx = Context::new();
    ctx.insert("value", "");
    print!("{}", tera.render("t3", &ctx).unwrap());

    // t4: range/for loop over a list.
    tera.add_raw_template("t4", "Range: {% for item in items %}{{ item }} {% endfor %}\n")
        .unwrap();

    let mut ctx = Context::new();
    ctx.insert("items", &["Go", "Rust", "C++", "C#"]);
    print!("{}", tera.render("t4", &ctx).unwrap());
}
