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

const KNOWN_SHAPES: [&'static str; 5] = [
    "Triangle",
    "Right Triangle",
    "Equilateral Triangle",
    "Square",
    "Circle",
];

const NUMBER_OF_SHAPES_KNOWN: usize = KNOWN_SHAPES.len();

/// Create a Shape
///
/// # Arguments
///
///   * `name` shape to be created
///
pub fn create(name: &str) -> Option<Box<dyn Shape>> {
    match name {
        "Triangle" => Triangle::new().into(),
        "Right Triangle" => RightTriangle::new().into(),
        "Equilateral Triangle" => EquilateralTriangle::new().into(),
        "Square" => Square::new().into(),
        "Circle" => Circle::new().into(),
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
    match name {
        "Triangle" => Triangle::from(dims).into(),
        "Right Triangle" => RightTriangle::from(dims).into(),
        "Equilateral Triangle" => EquilateralTriangle::from(dims).into(),
        "Square" => Square::from(dims).into(),
        "Circle" => Circle::from(dims).into(),
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
    KNOWN_SHAPES
        .iter()
        .find(|&shape_name| shape_name == &name)
        .is_some()
}

pub fn number_known() -> usize {
    NUMBER_OF_SHAPES_KNOWN
}

/// List the known shapes, one per line
///
pub fn list_known() -> String {
    KNOWN_SHAPES
        .iter()
        .map(|name| format!("  {}", name))
        .collect::<Vec<String>>()
        .join("\n")
        + "\n"
}

/// Create shapes based on names from an input buffer.
///
/// # Arguments
///
///  * `ins` - input source
///
pub fn read_shapes<B: BufRead>(ins: B) -> Vec<Box<dyn Shape>> {
    ins.lines()
        .map(|line| {
            let n = line.unwrap_or("unknown".into());
            let n = n.trim();

            create(n)
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

            create_with(&name, &dims)
        })
        .flatten()
        .collect()
}
