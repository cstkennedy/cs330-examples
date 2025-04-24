extern crate shapes;

use shapes::factory::Factory;

use shapes::shape::Shape;

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
fn main() {
    // Print Program Heading
    println!("{}", "-".repeat(80));

    for &line in PROGRAM_HEADING.iter() {
        println!("{:^80}", line);
    }

    println!("{}", "-".repeat(80));

    // Examine the ShapeFactory
    println!("{}", "*".repeat(38));
    println!("{:^38}", "Available Shapes");
    println!("{}", "*".repeat(38));

    // List the available shapes
    print!("{}", Factory::list_known());
    println!("{}", "-".repeat(38));
    println!("{:>2} shapes available.", Factory::number_known());
    println!();

    // Create 5 "Random" Shapes
    /*
    let mut shapes: Vec<Box<Shape>> = Vec::new();
    shapes.push(Factory::create("Triangle").unwrap());
    shapes.push(Factory::create("Right Triangle").unwrap());
    shapes.push(Factory::create("Equilateral Triangle").unwrap());
    shapes.push(Factory::create("Square").unwrap());
    shapes.push(Factory::create("Circle").unwrap());

    let next_shape = Factory::create("1337 Haxor");
    match next_shape {
        Some(s) => shapes.push(s),
        None => {},
    }
    */

    // Create 5 "Random" Shapes **with a loop**
    let shape_names = [
        "Triangle",
        "Right Triangle",
        "Equilateral Triangle",
        "Square",
        "Circle",
        "1337 Haxor",
    ];

    /*
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();

    for nme in shape_names.iter() {
        let next_shape = Factory::create(nme);
        match next_shape {
            Some(shp) => shapes.push(shp),
            None => {}
        }
    }
    */

    let shapes: Vec<Box<dyn Shape>> = shape_names
        .iter()
        .map(|nme| Factory::create(nme))
        .flatten()
        .collect();

    println!("{}", "*".repeat(38));
    println!("{:^38}", "Shapes That Exist");
    println!("{}", "*".repeat(38));
    println!("{:<24}: {:>4}", "Original Size", shapes.len());
    println!("{:<24}: {:>4}", "Invalid Shapes", 0);
    println!();

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
        println!("{:?}", shp);
    }

    println!();
    println!("{:?}", shapes);
}
