use crate::circle::Circle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::right_triangle::RightTriangle;
use crate::shape::Shape;
use crate::square::Square;
use crate::triangle::Triangle;

use std::cell::LazyCell;

impl From<Circle> for Box<dyn Shape> {
    fn from(shape: Circle) -> Self {
        Box::new(shape)
    }
}

impl From<Square> for Box<dyn Shape> {
    fn from(shape: Square) -> Self {
        Box::new(shape)
    }
}

impl From<Triangle> for Box<dyn Shape> {
    fn from(shape: Triangle) -> Self {
        Box::new(shape)
    }
}

impl From<EquilateralTriangle> for Box<dyn Shape> {
    fn from(shape: EquilateralTriangle) -> Self {
        Box::new(shape)
    }
}

impl From<RightTriangle> for Box<dyn Shape> {
    fn from(shape: RightTriangle) -> Self {
        Box::new(shape)
    }
}

impl From<&[f64]> for Triangle {
    fn from(dims: &[f64]) -> Self {
        Triangle {
            side_a: dims[0],
            side_b: dims[1],
            side_c: dims[2],
        }
    }
}

impl From<&[f64]> for EquilateralTriangle {
    fn from(dims: &[f64]) -> Self {
        EquilateralTriangle { side: dims[0] }
    }
}

impl From<&[f64]> for RightTriangle {
    fn from(dims: &[f64]) -> Self {
        RightTriangle {
            base: dims[0],
            height: dims[1],
        }
    }
}

impl From<&[f64]> for Circle {
    fn from(dims: &[f64]) -> Self {
        Circle { radius: dims[0] }
    }
}

impl From<&[f64]> for Square {
    fn from(dims: &[f64]) -> Self {
        Square { side: dims[0] }
    }
}

type DefaultFunction = dyn Fn() -> Box<dyn Shape>;
type DimFunction = dyn Fn(&[f64]) -> Box<dyn Shape>;
type ShapeTuple<'a> = (&'a str, Box<DefaultFunction>, Box<DimFunction>, usize);

#[rustfmt::skip]
const CREATE_SHAPE: LazyCell<Vec<ShapeTuple>> = LazyCell::new(|| {
    vec![
        (
            "Triangle",
            Box::new(|| Triangle::new().into()),
            Box::new(|dims| Triangle::from(dims).into()),
            3
        ),
        (
            "Right Triangle",
            Box::new(|| RightTriangle::new().into()),
            Box::new(|dims| RightTriangle::from(dims).into()),
            2,
        ),
        (
            "Equilateral Triangle",
            Box::new(|| EquilateralTriangle::new().into()),
            Box::new(|dims| EquilateralTriangle::from(dims).into()),
            1
        ),
        (
            "Square",
            Box::new(|| Square::new().into()),
            Box::new(|dims| Square::from(dims).into()),
            1
        ),
        (
            "Circle",
            Box::new(|| Circle::new().into()),
            Box::new(|dims| Circle::from(dims).into()),
            1
        ),
    ]
});

pub struct Factory;

impl Factory {
    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    pub fn create(name: &str) -> Option<Box<dyn Shape>> {
        match CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _, _)| shape_name == &name)
        {
            Some((_, creation_op, _, _)) => creation_op().into(),
            _ => None,
        }
    }

    /// Create a Shape with specified dimensions.
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///   * `dims` input dimensions
    ///
    pub fn create_with(name: &str, dims: &[f64]) -> Option<Box<dyn Shape>> {
        if let Some((_, _, creation_op, required_len)) = CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _, _)| shape_name == &name)
        {
            if *required_len == dims.len() {
                return creation_op(&dims).into();
            }
        }
        None
    }

    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    pub fn is_known(name: &str) -> bool {
        CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _, _)| shape_name == &name)
            .is_some()
    }

    pub fn number_known() -> usize {
        CREATE_SHAPE.len()
    }

    /// List the known shapes, one per line
    ///
    pub fn list_known() -> String {
        CREATE_SHAPE
            .iter()
            .map(|(name, _, _, _)| format!("  {}", name))
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }
}
