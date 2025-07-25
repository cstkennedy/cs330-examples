extern crate ordered_float;
extern crate shapes;

use eyre::WrapErr;
use ordered_float::OrderedFloat;

use shapes::prelude::Factory;
use shapes::prelude::Parser as ShapeParser;

use std::cell::LazyCell;
use std::fs::File;
use std::io::BufReader;
use std::vec::Vec;

use clap::Parser;

#[derive(clap::Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    shapes_filename: String,
}

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
        Factory::list_known().to_string(),
        STAR_DIVIDER.clone(),
        format!("{:>2} shapes available.", Factory::number_known()),
        "".to_owned(),
    ]
    .into_iter()
    .collect::<Vec<String>>()
    .join("\n")
});

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
/// A common approach is to make use of the Factory Model.  This Model exists
/// in a number of languages--e.g.:
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
/// In this example, our factory class is called ShapeFactory The
/// ShapeFactory could be designed as a singleton class.  Our ShapeFactory is
/// simply a tracker--i.e., records are static and will be updated manually
/// at compile time.
///
fn main() -> eyre::Result<()> {
    let cli = Args::parse();

    println!("{}", *PROGRAM_HEADING);
    println!("{}", *FACTORY_INFO);

    let shapes = {
        let file = File::open(&cli.shapes_filename)
            .wrap_err_with(|| format!("Could not open '{}", cli.shapes_filename))?;

        let ins = BufReader::new(file);
        ShapeParser::read_shapes_with(ins)
    };

    if shapes.len() == 0 {
        eyre::bail!("'{}' did not contain any valid shapes", cli.shapes_filename);
    }

    println!("{}", *STAR_DIVIDER);
    println!("{:^38}", "Display All Shapes");
    println!("{}", *STAR_DIVIDER);

    for shp in shapes.iter() {
        println!("{}", shp);
        println!();
    }

    println!("{}", *STAR_DIVIDER);
    println!("{:^38}", "Display Shape Names");
    println!("{}", *STAR_DIVIDER);
    for s in shapes.iter() {
        println!("{}", s.name());
    }
    println!();

    if let Some(largest) = shapes.iter().max_by_key(|s| OrderedFloat(s.area())) {
        println!("{}", *STAR_DIVIDER);
        println!("{:^38}", "Largest Shape by Area");
        println!("{}", *STAR_DIVIDER);

        println!("{}", largest);
        println!();
    }

    if let Some(smallest) = shapes.iter().min_by_key(|s| OrderedFloat(s.perimeter())) {
        println!("{}", *STAR_DIVIDER);
        println!("{:^38}", "Smallest Shape by Perimeter");
        println!("{}", *STAR_DIVIDER);

        println!("{}", smallest);
    }

    Ok(())
}
