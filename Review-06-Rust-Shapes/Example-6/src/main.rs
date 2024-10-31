extern crate ordered_float;
extern crate shapes;

use ordered_float::OrderedFloat;

use shapes::prelude::*;

use std::cell::LazyCell;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

const PROGRAM_HEADING: LazyCell<String> = LazyCell::new(|| {
    let heading: String = ["Objects & Traits: 2-D Shapes", "Thomas J. Kennedy"]
        .iter()
        .map(|line| format!("{:^80}\n", line))
        .collect();

    format!("{}\n{}{}", "-".repeat(80), heading, "-".repeat(80))
});

const FACTORY_INFO: LazyCell<String> = LazyCell::new(|| {
    [
        "*".repeat(38),
        format!("{:^38}", "Available Shapes"),
        "*".repeat(38),
        format!("{}", Factory::list_known()),
        "-".repeat(38),
        format!("{:>2} shapes available.", Factory::number_known()),
        "".to_owned(),
    ]
    .into_iter()
    .collect::<Vec<String>>()
    .join("\n")
});

// What happens when the number of shapes is non-trivial?
//
// Suppose we were to expand our Shape hierarchy to include the following
// shapes:
//   - Isosceles Triangle
//   - Circle
//   - Ellipse
//   - Rectangle
//   - Square
//   - Rhombus
//   - Parallelogram
//   - Kite
//   - Generalized Polygon
//
// How would we manage the addition of new Shapes?
//
// A common approach is to make use of the Factory Model.  This Model exists
// in a number of languages--e.g.:
//   - C++
//   - Java
//   - Python
//   - Rust
//   - PHP
//   - C#
//
// A class that contains static members is created.  As new classes are
// created, the Factory Class is updated.
//
// In this example, our factory class is called ShapeFactory The
// ShapeFactory could be designed as a singleton class.  Our ShapeFactory is
// simply a tracker--i.e., records are static and will be updated manually
// at compile time.
fn main() {
    // Print Program Heading
    println!("{}", *PROGRAM_HEADING);

    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        println!("Usage: {} file_name", argv[0]);
        std::process::exit(1);
    }

    println!("{}", *FACTORY_INFO);

    let argv: Vec<String> = env::args().collect();
    let file = File::open(&argv[1]).expect("Could not open file");
    let ins = BufReader::new(file);

    let shapes = Parser::read_shapes_with(ins);

    println!("{}", "*".repeat(38));
    println!("{:^38}", "Display All Shapes");
    println!("{}", "*".repeat(38));

    for shp in shapes.iter() {
        println!("{}\n", shp);
    }
    println!();

    println!("{}", "*".repeat(38));
    println!("{:^38}", "Display Shape Names");
    println!("{}", "*".repeat(38));
    for s in shapes.iter() {
        println!("{}", s.name());
    }
    println!();

    println!("{}", "*".repeat(38));
    println!("{:^38}", "Largest Shape by Area");
    println!("{}", "*".repeat(38));

    let largest = &*shapes
        .iter()
        .max_by_key(|s| OrderedFloat(s.area()))
        .unwrap();
    println!("{}", largest);

    println!("{}", "*".repeat(38));
    println!("{:^38}", "Smallest Shape by Perimeter");
    println!("{}", "*".repeat(38));

    let smallest = &*shapes
        .iter()
        .min_by_key(|s| OrderedFloat(s.perimeter()))
        .unwrap();
    println!("{}", smallest);
}
