extern crate ordered_float;
extern crate shapes;

use ordered_float::OrderedFloat;
use eyre;
use eyre::WrapErr;

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

const STAR_DIVIDER: LazyCell<String> = LazyCell::new(|| "*".repeat(38));

const FACTORY_INFO: LazyCell<String> = LazyCell::new(|| {
    [
        STAR_DIVIDER.clone(),
        format!("{:^38}", "Available Shapes"),
        STAR_DIVIDER.clone(),
        format!("{}", Factory::list_known()),
        STAR_DIVIDER.clone(),
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
fn main() -> eyre::Result<()> {
    println!("{}", *PROGRAM_HEADING);

    let argv: Vec<String> = env::args().collect();

    if argv.len() < 2 {
        println!("Usage: {} file_name", argv[0]);
        std::process::exit(1);
    }

    println!("{}", *FACTORY_INFO);

    let filename: &str = &argv[1];
    let file = File::open(filename).wrap_err_with(|| format!("Could not open '{}", filename))?;
    let ins = BufReader::new(file);

    let shapes = Parser::read_shapes_with(ins);

    println!("{}", *STAR_DIVIDER);
    println!("{:^38}", "Display All Shapes");
    println!("{}", *STAR_DIVIDER);

    for shp in shapes.iter() {
        println!("{}\n", shp);
    }
    println!();

    println!("{}", *STAR_DIVIDER);
    println!("{:^38}", "Display Shape Names");
    println!("{}", *STAR_DIVIDER);
    for s in shapes.iter() {
        println!("{}", s.name());
    }
    println!();

    println!("{}", *STAR_DIVIDER);
    println!("{:^38}", "Largest Shape by Area");
    println!("{}", *STAR_DIVIDER);

    let largest = &*shapes
        .iter()
        .max_by_key(|s| OrderedFloat(s.area()))
        .unwrap();
    println!("{}", largest);

    println!("{}", *STAR_DIVIDER);
    println!("{:^38}", "Smallest Shape by Perimeter");
    println!("{}", *STAR_DIVIDER);

    let smallest = &*shapes
        .iter()
        .min_by_key(|s| OrderedFloat(s.perimeter()))
        .unwrap();
    println!("{}", smallest);

    Ok(())
}
