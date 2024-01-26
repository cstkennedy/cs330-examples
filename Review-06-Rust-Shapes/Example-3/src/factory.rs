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
            + "\n"
    }
}

/// Create shapes based on names from an input buffer
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes<B: BufRead>(ins: B, shape_factory: Factory) -> Vec<Box<dyn Shape>> {
    /*
    let mut shapes: Vec<Box<dyn Shape>> = Vec::new();

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
    */
    ins.lines()
        .map(|line| {
            let n = line.unwrap_or("unknown".into());
            let n = n.trim();

            shape_factory.create(n)
        })
        .flatten()
        .collect()
}
