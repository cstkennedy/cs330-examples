use std::collections::HashSet;
use std::fmt;
use std::io::BufRead;

use crate::shape::Shape;
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
        match &*self {
            KnownShape::Triangle(s) => write!(f, "{}", s),
            KnownShape::RightTriangle(s) => write!(f, "{}", s),
            KnownShape::EquilateralTriangle(s) => write!(f, "{}", s),
            KnownShape::Square(s) => write!(f, "{}", s),
            KnownShape::Circle(s) => write!(f, "{}", s),
        }
    }
}

impl Shape for KnownShape {
    fn name(&self) -> &'static str {
        match &*self {
            KnownShape::Triangle(s) => s.name(),
            KnownShape::RightTriangle(s) => s.name(),
            KnownShape::EquilateralTriangle(s) => s.name(),
            KnownShape::Square(s) => s.name(),
            KnownShape::Circle(s) => s.name(),
        }
    }

    fn area(&self) -> f64 {
        match &*self {
            KnownShape::Triangle(s) => s.area(),
            KnownShape::RightTriangle(s) => s.area(),
            KnownShape::EquilateralTriangle(s) => s.area(),
            KnownShape::Square(s) => s.area(),
            KnownShape::Circle(s) => s.area(),
        }
    }

    fn perimeter(&self) -> f64 {
        match &*self {
            KnownShape::Triangle(s) => s.perimeter(),
            KnownShape::RightTriangle(s) => s.perimeter(),
            KnownShape::EquilateralTriangle(s) => s.perimeter(),
            KnownShape::Square(s) => s.perimeter(),
            KnownShape::Circle(s) => s.perimeter(),
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

    /// Create a Shape with specified dimensions.
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///   * `dims` input dimensions
    ///
    pub fn create_with(&self, name: &str, dims: &[f64]) -> Option<KnownShape> {
        match name  {
            "Triangle" => {
                Some(KnownShape::Triangle(Triangle::with_sides(dims[0], dims[1], dims[2])))
            },
            "Right Triangle" => {
                Some(KnownShape::RightTriangle(RightTriangle::with_base_height(dims[0], dims[1])))
            },
            "Equilateral Triangle" => {
                Some(KnownShape::EquilateralTriangle(EquilateralTriangle::with_side(dims[0])))
            },
            "Square" => {
                Some(KnownShape::Square(Square::with_side(dims[0])))
            },
            "Circle" => {
                Some(KnownShape::Circle(Circle::with_radius(dims[0])))
            },
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

/// Create shapes based on names from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes<B: BufRead>(ins: B, shape_factory: Factory)-> Vec<KnownShape> {

    let mut shapes: Vec<KnownShape> = Vec::new();

    for line in ins.lines() {
        let n = line.unwrap();
        let n = n.trim();
        if let Some(s) = shape_factory.create(n) {
            shapes.push(s)
        }
    }

    shapes
}

/// Create shapes based on names *and dimension data* from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes_with<B>(ins: B, shape_factory: Factory)-> Vec<KnownShape>
    where B: BufRead  {

    let mut shapes: Vec<KnownShape> = Vec::new();

    for line in ins.lines() {
        let raw_line = line.unwrap();

        // Skip empty line
        if raw_line.is_empty() {
            continue;
        }

        let split_line: Vec<String> = raw_line.trim().split(';')
            .map(|s| s.to_string())
            .collect();

        // There is no line data (i.e., no ';')
        if split_line.len() != 2 {
            continue;
        }

        let n = split_line[0].clone();
        let split_line = &split_line[1];

        // Mistake -> s.len() > 0 != s.is_empty() -> I forgot the leading '!'
        let dims: Vec<f64> = split_line.split(' ')
            .filter(|s| !s.is_empty())
            .map(|dim| match dim.trim().parse() {
                Ok(d) => d,
                Err(_e) => 0.0,
            }).collect();

        if let Some(s) = shape_factory.create_with(&n, &dims) {
            shapes.push(s);
        }
    }

    shapes
}
