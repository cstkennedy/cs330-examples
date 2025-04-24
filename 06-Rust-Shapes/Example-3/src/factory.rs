use crate::circle::Circle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::right_triangle::RightTriangle;
use crate::shape::Shape;
use crate::square::Square;
use crate::triangle::Triangle;
use std::io::BufRead;

impl From<Triangle> for Option<Box<dyn Shape>> {
    fn from(shape: Triangle) -> Self {
        Some(Box::new(shape))
    }
}

impl From<EquilateralTriangle> for Option<Box<dyn Shape>> {
    fn from(shape: EquilateralTriangle) -> Self {
        Some(Box::new(shape))
    }
}

impl From<RightTriangle> for Option<Box<dyn Shape>> {
    fn from(shape: RightTriangle) -> Self {
        Some(Box::new(shape))
    }
}

impl From<Circle> for Option<Box<dyn Shape>> {
    fn from(shape: Circle) -> Self {
        Some(Box::new(shape))
    }
}

impl From<Square> for Option<Box<dyn Shape>> {
    fn from(shape: Square) -> Self {
        Some(Box::new(shape))
    }
}

pub struct Factory;


impl Factory {
    const KNOWN_SHAPES: [&'static str; 5] = [
                "Triangle",
                "Right Triangle",
                "Equilateral Triangle",
                "Square",
                "Circle",
            ];


    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    pub fn create(name: &str) -> Option<Box<dyn Shape>> {
        // match name {
        //     "Triangle" => Some(Box::new(Triangle::new())),
        //     "Right Triangle" => Some(Box::new(RightTriangle::new())),
        //     "Equilateral Triangle" => Some(Box::new(EquilateralTriangle::new())),
        //     "Square" => Some(Box::new(Square::new())),
        //     "Circle" => Some(Box::new(Circle::new())),
        //     _ => None,
        // }

        match name {
            "Triangle" => Triangle::new().into(),
            "Right Triangle" => RightTriangle::new().into(),
            "Equilateral Triangle" => EquilateralTriangle::new().into(),
            "Square" => Square::new().into(),
            "Circle" => Circle::new().into(),
            _ => None,
        }
    }

    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    pub fn is_known(name: &str) -> bool {
        Self::KNOWN_SHAPES
            .iter()
            .find(|&shape_name| shape_name == &name)
            .is_some()
    }

    pub fn number_known() -> usize {
        Self::KNOWN_SHAPES.len()
    }

    pub fn list_known() -> String {
        Self::KNOWN_SHAPES
            .iter()
            .map(|name| format!("  {}", name))
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }
}
