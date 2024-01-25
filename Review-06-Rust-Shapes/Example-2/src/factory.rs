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
    known_shapes: HashSet<&'static str>,
}

impl Factory {
    pub fn new() -> Self {
        let factory = Factory {
            known_shapes: HashSet::from([
                "Triangle",
                "Right Triangle",
                "Equilateral Triangle",
                "Square",
                "Circle",
            ]),
        };

        factory
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

    // Rust Refactoring required....
    // <https://stackoverflow.com/questions/30353462/how-to-clone-a-struct-storing-a-boxed-trait-object/30353928#30353928>
    /*
    pub fn create(&self, name: &str) -> Option<Box<Shape>>{
        match self.known_shapes.get(name) {
            Some(&shape_box) => Some(Box::new((*shape_box).clone())),
            None => None
        }
    }
    */

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
