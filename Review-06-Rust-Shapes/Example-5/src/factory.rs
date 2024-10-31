use crate::circle::Circle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::right_triangle::RightTriangle;
use crate::shape::Shape;
use crate::square::Square;
use crate::triangle::Triangle;

use std::cell::LazyCell;
use std::io::BufRead;

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

#[rustfmt::skip]
const CREATE_SHAPE_FROM_DEFAULTS: LazyCell<Vec<(&str, Box<dyn Fn() -> Box<dyn Shape>>)>> = LazyCell::new(|| {
    vec![
        (
            "Triangle",
            Box::new(|| Triangle::new().into())
        ),
        (
            "Right Triangle",
            Box::new(|| RightTriangle::new().into()),
        ),
        (
            "Equilateral Triangle",
            Box::new(|| EquilateralTriangle::new().into()),
        ),
        (
            "Square",
            Box::new(|| Square::new().into())
        ),
        (
            "Circle",
            Box::new(|| Circle::new().into())
        ),
    ]
});

#[rustfmt::skip]
const CREATE_SHAPE_FROM_DIMS: LazyCell<Vec<(&str, Box<dyn Fn(&[f64]) -> Box<dyn Shape>>)>> = LazyCell::new(|| {
    vec![
        (
            "Triangle",
            Box::new(|dims| Triangle::from(dims).into())
        ),
        (
            "Right Triangle",
            Box::new(|dims| RightTriangle::from(dims).into()),
        ),
        (
            "Equilateral Triangle",
            Box::new(|dims| EquilateralTriangle::from(dims).into()),
        ),
        (
            "Square",
            Box::new(|dims| Square::from(dims).into())
        ),
        (
            "Circle",
            Box::new(|dims| Circle::from(dims).into())
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
        match CREATE_SHAPE_FROM_DEFAULTS
            .iter()
            .find(|(shape_name, _)| shape_name == &name)
        {
            Some((_, creation_op)) => creation_op().into(),
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
        match CREATE_SHAPE_FROM_DIMS
            .iter()
            .find(|(shape_name, _)| shape_name == &name)
        {
            Some((_, creation_op)) => creation_op(&dims).into(),
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
        CREATE_SHAPE_FROM_DEFAULTS
            .iter()
            .find(|(shape_name, _)| shape_name == &name)
            .is_some()
    }

    pub fn number_known() -> usize {
        CREATE_SHAPE_FROM_DEFAULTS.len()
    }

    /// List the known shapes, one per line
    ///
    pub fn list_known() -> String {
        CREATE_SHAPE_FROM_DEFAULTS
            .iter()
            .map(|(name, _)| format!("  {}", name))
            .collect::<Vec<String>>()
            .join("\n")
            + "\n"
    }
}
/// Create shapes based on names from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes<B: BufRead>(ins: B) -> Vec<Box<dyn Shape>> {
    ins.lines()
        .flatten()
        .map(|line| {
            let name = line.trim();
            Factory::create(name)
        })
        .flatten()
        .collect()
}

/// Create shapes based on names *and dimension data* from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes_with<B>(ins: B) -> Vec<Box<dyn Shape>>
where
    B: BufRead,
{
    ins.lines()
        .flatten()
        .filter(|line| line.len() > 0)
        .map(|line| {
            line.trim()
                .split(";")
                .map(|s| s.to_string())
                .collect::<Vec<String>>()
        })
        .filter(|split_line| split_line.len() == 2)
        .map(|split_line| {
            let name = &split_line[0];

            let dims = &split_line[1]
                .split(' ')
                .filter(|s| !s.is_empty())
                .map(|dim| dim.trim().parse().unwrap_or(0_f64))
                .collect::<Vec<f64>>();

            Factory::create_with(&name, &dims)
        })
        .flatten()
        .collect()
}
