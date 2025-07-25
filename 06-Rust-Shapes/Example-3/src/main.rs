extern crate shapes;


use shapes::prelude::*;
use std::env;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

const PROGRAM_HEADING: [&'static str; 2] = ["Objects & Traits: 2-D Shapes", "Thomas J. Kennedy"];

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
#[cfg_attr(tarpaulin, skip)]
fn main() {
    // Print Program Heading
    println!("{}", "-".repeat(80));

    for &line in PROGRAM_HEADING.iter() {
        println!("{:^80}", line);
    }

    println!("{}", "-".repeat(80));

    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        println!("Usage: {} file_name", argv[0]);
        std::process::exit(1);
    }

    // Examine the ShapeFactory
    println!("{}", "*".repeat(38));
    println!("{:^38}", "Available Shapes");
    println!("{}", "*".repeat(38));

    // List the available shapes
    print!("{}", Factory::list_known());
    println!("{}", "-".repeat(38));
    println!("{:>2} shapes available.", Factory::number_known());
    println!();

    let file = File::open(&argv[1]).expect("Could not open file");
    let ins = BufReader::new(file);

    let shapes = Parser::read_shapes(ins);

    println!("{}", "*".repeat(38));
    println!("{:^38}", "Display All Shapes");
    println!("{}", "*".repeat(38));

    for shp in shapes.iter() {
        println!("{}\n", shp);
    }

    println!("{}", "*".repeat(38));
    println!("{:^38}", "Display All Shapes (Debug)");
    println!("{}", "*".repeat(38));

    for shp in shapes.iter() {
        println!("{:#?}", shp);
    }
}
