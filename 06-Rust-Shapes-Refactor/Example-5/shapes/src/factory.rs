use crate::circle::Circle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::error::CreationError;
use crate::right_triangle::RightTriangle;
use crate::shape::Shape;
use crate::square::Square;
use crate::triangle::Triangle;

use std::cell::LazyCell;

use itertools::Itertools;

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

impl TryFrom<&[f64]> for Triangle {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 3 {
            return Err(CreationError::DimensionCountError {
                name: "Triangle",
                num_required: 3,
            });
        }
        Ok(Triangle {
            side_a: dims[0],
            side_b: dims[1],
            side_c: dims[2],
        })
    }
}

impl TryFrom<&[f64]> for EquilateralTriangle {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 1 {
            return Err(CreationError::DimensionCountError {
                name: "Equilateral Triangle",
                num_required: 1,
            });
        }
        Ok(EquilateralTriangle { side: dims[0] })
    }
}

impl TryFrom<&[f64]> for RightTriangle {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 2 {
            return Err(CreationError::DimensionCountError {
                name: "Right Triangle",
                num_required: 2,
            });
        }
        Ok(RightTriangle {
            base: dims[0],
            height: dims[1],
        })
    }
}

impl TryFrom<&[f64]> for Circle {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 1 {
            return Err(CreationError::DimensionCountError {
                name: "Circle",
                num_required: 1,
            });
        }
        Ok(Circle { radius: dims[0] })
    }
}

impl TryFrom<&[f64]> for Square {
    type Error = CreationError;

    fn try_from(dims: &[f64]) -> Result<Self, Self::Error> {
        if dims.len() != 1 {
            return Err(CreationError::DimensionCountError {
                name: "Square",
                num_required: 1,
            });
        }

        Ok(Square { side: dims[0] })
    }
}

impl std::str::FromStr for Box<dyn Shape> {
    type Err = CreationError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let split_line: Vec<&str> = line.trim().split(";").collect();

        if split_line.len() != 2 {
            return Err(CreationError::MalformedLineError(
                compact_str::format_compact!(
                    "Line '{line}' did not have exactly one (1) semicolon"
                ),
            ));
        }

        let name = split_line[0];
        let values: Vec<f64> = split_line[1]
            .split_whitespace()
            .flat_map(|token| token.parse())
            .collect();

        let shape = Factory::create_with(name, &values)?;

        Ok(shape)
    }
}

//------------------------------------------------------------------------------
// Factory Traits
//------------------------------------------------------------------------------
pub trait CreationFactory {
    type Item;
    type Error;

    fn create_default(name: &str) -> Result<Self::Item, Self::Error>;

    fn create_with(name: &str, dims: &[f64]) -> Result<Self::Item, Self::Error>;
}

pub trait FactoryDirectory {
    fn is_known(name: &str) -> bool;

    fn number_known() -> usize;

    fn list_known() -> String;
}

//------------------------------------------------------------------------------
// Factory Implementation
//------------------------------------------------------------------------------
type DefaultFunction = dyn Fn() -> Box<dyn Shape>;
type DimFunction = dyn Fn(&[f64]) -> Result<Box<dyn Shape>, CreationError>;
type ShapeTuple<'a> = (&'a str, Box<DefaultFunction>, Box<DimFunction>);

#[rustfmt::skip]
const CREATE_SHAPE: LazyCell<[ShapeTuple; 5]> = LazyCell::new(|| {
    [
        (
            "Triangle",
            Box::new(|| Triangle::default().into()),
            Box::new(|dims| Ok(Triangle::try_from(dims)?.into())),
        ),
        (
            "Right Triangle",
            Box::new(|| RightTriangle::default().into()),
            Box::new(|dims| Ok(RightTriangle::try_from(dims)?.into())),
        ),
        (
            "Equilateral Triangle",
            Box::new(|| EquilateralTriangle::default().into()),
            Box::new(|dims| Ok(EquilateralTriangle::try_from(dims)?.into())),
        ),
        (
            "Square",
            Box::new(|| Square::default().into()),
            Box::new(|dims| Ok(Square::try_from(dims)?.into())),
        ),
        (
            "Circle",
            Box::new(|| Circle::default().into()),
            Box::new(|dims| Ok(Circle::try_from(dims)?.into())),
        ),
    ]
});

pub struct Factory;

impl CreationFactory for Factory {
    type Item = Box<dyn Shape>;
    type Error = CreationError;

    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    fn create_default(name: &str) -> Result<Self::Item, Self::Error> {
        match CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
        {
            Some((_, creation_op, _)) => Ok(creation_op().into()),
            None => Err(CreationError::UnknownShapeError(name.into())),
        }
    }

    /// Create a Shape with specified dimensions.
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///   * `dims` input dimensions
    ///
    fn create_with(name: &str, dims: &[f64]) -> Result<Self::Item, Self::Error> {
        match CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
        {
            None => Err(CreationError::UnknownShapeError(name.into())),
            Some((_, _, creation_op)) => creation_op(&dims),
        }
    }
}

impl FactoryDirectory for Factory {
    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    fn is_known(name: &str) -> bool {
        CREATE_SHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
            .is_some()
    }

    fn number_known() -> usize {
        CREATE_SHAPE.len()
    }

    /// List the known shapes, one per line
    ///
    fn list_known() -> String {
        CREATE_SHAPE
            .iter()
            .map(|(name, _, _)| format!("  {}", name))
            .join("\n")
            + "\n"
    }
}
