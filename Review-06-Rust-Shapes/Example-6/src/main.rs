extern crate shapes;
extern crate ordered_float;

use ordered_float::OrderedFloat;

use shapes::shape::Shape;
use shapes::factory::Factory;
use shapes::factory::KnownShape;

use std::io::BufReader;
use std::fs::File;
use std::env;
use std::vec::Vec;

const PROGRAM_HEADING: [&str; 2] = ["Objects & Traits: 2-D Shapes",
                                    "Thomas J. Kennedy"];

/// Utility function to print the program heading to Standard Out
#[cfg_attr(tarpaulin, skip)]
fn print_heading() {

    println!("{}", "-".repeat(80));

    for &line in PROGRAM_HEADING.iter() {
        println!("{:^80}", line);
    }

    println!("{}", "-".repeat(80));
}

/// Utility function to print a one-line heading with borders
fn print_section_heading(title: &str) {

    println!("{}", "-".repeat(38));
    println!("{:^38}", title);
    println!("{}", "-".repeat(38));
}



/// What happens when the number of shapes is non-trivial?
///
/// Suppose we were to expand our Shape hierarchy to include the following
/// shapes:
///   - Isosceles Triangle
///   - Circle
///   - Ellipse
///   - Rectangle
///   - Square
///   - Rhombus
///   - Parallelogram
///   - Kite
///   - Generalized Polygon
///
/// How would we manage the addition of new Shapes?
///
/// A common approach is to make use of the Factory Model.This Model exists in a
/// number of languages--e.g.:
///   - C++
///   - Java
///   - Python
///   - Rust
///   - PHP
///   - C#
///
/// A class that contains static members is created.  As new classes are
/// created, the Factory Class is updated.
///
/// In this example, our factory class is called ShapeFactory. The ShapeFactory
/// could be designed as a singleton class. Our ShapeFactory is simply a
/// tracker--i.e., records are fixed and will be updated manually at compile
/// time.
#[cfg_attr(tarpaulin, skip)]
fn main() {

    print_heading();

    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        println!("Usage: {} file_name", argv[0]);
        std::process::exit(1);
    }

    let shape_factory = Factory::new();

    // Examine the ShapeFactory
    print_section_heading("Available Shapes");
    print!("{}", shape_factory);
    println!("{}", "-".repeat(38));
    println!("{:>2} shapes available.", shape_factory.number_known());
    println!();

    let f = File::open(&argv[1]).expect("Could not open file");
    let ins = BufReader::new(f);

    let mut shapes = shapes::factory::read_shapes_with(ins, shape_factory);

    print_section_heading("Display All Shapes");
    for s in shapes.iter() {
        println!("{}", s);
    }

    print_section_heading("Display Shape Names");
    for s in shapes.iter() {
        println!("{}", s.name());
    }
    println!();

    print_section_heading("Largest Shape by Area");
    let largest = &*shapes.iter()
                          .max_by_key(|s| OrderedFloat(s.area())).unwrap();
    println!("{}", largest);


    print_section_heading("Smallest Shape by Perimeter");
    let smallest = &*shapes.iter()
                          .min_by_key(|s| OrderedFloat(s.perimeter())).unwrap();
    println!("{}", smallest);
}
