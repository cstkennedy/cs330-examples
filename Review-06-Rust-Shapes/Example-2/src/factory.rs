use std::collections::HashSet;
use std::error::Error;
use std::fmt;

use crate::circle::Circle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::right_triangle::RightTriangle;
use crate::shape::Shape;
use crate::square::Square;
use crate::triangle::Triangle;

pub struct Factory {
    known_shapes: [&'static str; 5],
}

impl Factory {
    pub fn new() -> Self {
        Factory {
            known_shapes: [
                "Triangle",
                "Right Triangle",
                "Equilateral Triangle",
                "Square",
                "Circle",
            ],
        }
    }

    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    pub fn create(&self, name: &str) -> Option<Box<dyn Shape>> {
        match name {
            "Triangle" => Some(Box::new(Triangle::new())),
            "Right Triangle" => Some(Box::new(RightTriangle::new())),
            "Equilateral Triangle" => Some(Box::new(EquilateralTriangle::new())),
            "Square" => Some(Box::new(Square::new())),
            "Circle" => Some(Box::new(Circle::new())),
            _ => None,
        }
    }

    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    pub fn is_known(&self, name: &str) -> bool {
        self.known_shapes
            .iter()
            .find(|&shape_name| shape_name == &name)
            .is_some()
    }

    pub fn number_known(&self) -> usize {
        self.known_shapes.len()
    }

    pub fn list_known(&self) -> String {
        self.known_shapes
            .iter()
            .map(|name| format!("  {}", name))
            .collect::<Vec<String>>()
            .join("\n")
    }
}

impl fmt::Display for Factory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{}", self.list_known())
    }
}
