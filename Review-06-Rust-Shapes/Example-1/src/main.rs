extern crate shapes;

use shapes::shape::Shape;
use shapes::circle::Circle;
use shapes::equilateral_triangle::EquilateralTriangle;
use shapes::right_triangle::RightTriangle;

use std::fmt;

const PROGRAM_HEADING: [&'static str; 2] = ["Objects & Inheritance: 2-D Shapes",
                                            "Thomas J. Kennedy"];

fn main() {
    // Print Program Heading
    println!("{}", "-".repeat(80));

    for &line in PROGRAM_HEADING.iter() {
        println!("{:^80}", line);
    }

    println!("{}", "-".repeat(80));

    // Create one RightTriangle
    let rht_tri = RightTriangle::with_base_height(1.0, 2.0);

    let tri = &rht_tri;  // Point tri to rht_tri
    let shape = &rht_tri;  // Point shape to rht_tri

    // Is this a valid assignment?
    // let eql_tri = &rht_tri;
    // Yes, because how Rust variable bindings work

    println!("{:^38}", "Display a Right Triangle (rht_tri)");
    println!("{}", "-".repeat(38));
    println!("{}", rht_tri);

    println!("{:^38}", "Display a Right Triangle (tri)");
    println!("{}", "-".repeat(38));
    println!("{}", tri);

    println!("{:^38}", "Display a Right Triangle (shape)");
    println!("{}", "-".repeat(38));
    println!("{}", shape);

    println!("{}", "~".repeat(80));

    // Create one Equilateral Triangle
    let eql_tri = EquilateralTriangle::with_side(8.0);

    let tri = &eql_tri;  // Point tri to rht_tri
    let shape = &eql_tri;  // Point shape to rht_tri

    println!("{:^38}", "Display an Eql. Triangle (eql_tri)");
    println!("{}", "-".repeat(38));
    println!("{}", eql_tri);

    println!("{:^38}", "Display an Eql. Triangle (tri)");
    println!("{}", "-".repeat(38));
    println!("{}", tri);

    println!("{:^38}" ,"Display an Eql. Triangle (shape)");
    println!("{}", "-".repeat(38));
    println!("{}", shape);
}
