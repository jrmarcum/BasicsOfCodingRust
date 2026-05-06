// Rust supports XML encoding and decoding via the
// `quick-xml` crate with `serde` support.

use quick_xml::de::from_str;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename = "plant")]
struct Plant {
    #[serde(rename = "@id")]
    id: u32,
    name: String,
    #[serde(rename = "origin")]
    origin: Vec<String>,
}

impl std::fmt::Display for Plant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Plant id={}, name={}, origin={:?}", self.id, self.name, self.origin)
    }
}

fn print_plant_xml(plant: &Plant, indent: &str, sub_indent: &str) {
    println!("{}<plant id=\"{}\">", indent, plant.id);
    println!("{}<name>{}</name>", sub_indent, plant.name);
    for o in &plant.origin {
        println!("{}<origin>{}</origin>", sub_indent, o);
    }
    println!("{}</plant>", indent);
}

fn main() {
    let coffee = Plant {
        id: 27,
        name: "Coffee".to_string(),
        origin: vec!["Ethiopia".to_string(), "Brazil".to_string()],
    };

    // Print with indentation matching Go's MarshalIndent output.
    print_plant_xml(&coffee, " ", "   ");

    // Print with XML header then plant.
    println!("<?xml version=\"1.0\" encoding=\"UTF-8\"?>");
    print_plant_xml(&coffee, " ", "   ");

    // Unmarshal from XML (xml.Unmarshal equivalent).
    let xml_str = format!(
        "<plant id=\"{}\"><name>{}</name><origin>{}</origin><origin>{}</origin></plant>",
        coffee.id, coffee.name, coffee.origin[0], coffee.origin[1]
    );
    let p: Plant = from_str(&xml_str).unwrap();
    println!("{}", p);

    let tomato = Plant {
        id: 81,
        name: "Tomato".to_string(),
        origin: vec!["Mexico".to_string(), "California".to_string()],
    };

    // Nested structure output (parent>child>plant nesting).
    let plants = vec![coffee, tomato];
    println!(" <nesting>");
    println!("   <parent>");
    println!("     <child>");
    for plant in &plants {
        print_plant_xml(plant, "       ", "         ");
    }
    println!("     </child>");
    println!("   </parent>");
    println!(" </nesting>");
}
