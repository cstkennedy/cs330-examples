use std::collections::HashSet;
use std::fmt;
use std::io::BufRead;

//use crate::shape::Shape;
use crate::square::Square;
use crate::circle::Circle;
use crate::triangle::Triangle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::right_triangle::RightTriangle;

#[derive(Debug)]
pub enum KnownShape {
    Triangle(Triangle),
    RightTriangle(RightTriangle),
    EquilateralTriangle(EquilateralTriangle),
    Square(Square),
    Circle(Circle),
}

impl fmt::Display for KnownShape {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // match &*self {
            // KnownShape::Triangle(s) => writeln!(f, "{}", s),
            // KnownShape::RightTriangle(s) => writeln!(f, "{}", s),
            // KnownShape::EquilateralTriangle(s) => writeln!(f, "{}", s),
            // KnownShape::Square(s) => writeln!(f, "{}", s),
            // KnownShape::Circle(s) => writeln!(f, "{}", s),
        // }
        // Change to "write! based on factory test"
        match &*self {
            KnownShape::Triangle(s) => write!(f, "{}", s),
            KnownShape::RightTriangle(s) => write!(f, "{}", s),
            KnownShape::EquilateralTriangle(s) => write!(f, "{}", s),
            KnownShape::Square(s) => write!(f, "{}", s),
            KnownShape::Circle(s) => write!(f, "{}", s),
        }
    }
}

pub struct Factory {
    known_shapes: HashSet<&'static str>,
}

impl Factory {
    pub fn new() -> Self {
        let mut factory = Factory{ known_shapes: HashSet::new() };

        factory.known_shapes.insert("Triangle");
        factory.known_shapes.insert("Right Triangle");
        factory.known_shapes.insert("Equilateral Triangle");
        factory.known_shapes.insert("Square");
        factory.known_shapes.insert("Circle");

        factory
    }

    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    pub fn create(&self, name: &str) -> Option<KnownShape> {
        match name  {
            "Triangle" => Some(KnownShape::Triangle(Triangle::new())),
            "Right Triangle" => Some(KnownShape::RightTriangle(RightTriangle::new())),
            "Equilateral Triangle" => Some(KnownShape::EquilateralTriangle(EquilateralTriangle::new())),
            "Square" => Some(KnownShape::Square(Square::new())),
            "Circle" => Some(KnownShape::Circle(Circle::new())),
            _ =>  None
        }
    }

    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    pub fn is_known(&self, name: &str) -> bool {
        self.known_shapes.contains(name)
    }

    pub fn number_known(&self) -> usize {
        self.known_shapes.len()
    }
}

impl fmt::Display for Factory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for name in &self.known_shapes {
            writeln!(f, "  {}", name)?;
        }

        Ok(())
    }
}

/// Create shapes based on names from an input buffer
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes<B: BufRead>(ins: B, shape_factory: Factory)-> Vec<KnownShape> {

    let mut shapes: Vec<KnownShape> = Vec::new();

    for line in ins.lines() {
        // let next_shape = shape_factory.create(n);
        // match next_shape {
            // Some(s) => shapes.push(s),
            // None => {},
        // }
        let n = line.unwrap();
        let n = n.trim();
        if let Some(s) = shape_factory.create(n) {
            shapes.push(s)
        }
    }

    shapes
}
