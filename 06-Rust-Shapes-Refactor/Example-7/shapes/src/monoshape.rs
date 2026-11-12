use std::cell::LazyCell;
use std::str::FromStr;

use itertools::Itertools;

use crate::factory::{CreationFactory, FactoryDirectory};

use crate::circle::Circle;
use crate::equilateral_triangle::EquilateralTriangle;
use crate::error::CreationError;
use crate::right_triangle::RightTriangle;
use crate::shape::Shape;
use crate::square::Square;
use crate::triangle::Triangle;

//------------------------------------------------------------------------------
// Mono Shape Implementation
//------------------------------------------------------------------------------
/// A Monomorphized Wrapper for all Known Shapes.
///
/// Note: This is the first step in reducing duplicated FFI factory logic.
#[derive(Clone, Debug)]
pub enum MonoShape {
    Circle { inner: Circle },
    Square { inner: Square },
    Triangle { inner: Triangle },
    EquilateralTriangle { inner: EquilateralTriangle },
    RightTriangle { inner: RightTriangle },
}

// TODO: replace with macro
impl Shape for MonoShape {
    fn name(&self) -> &'static str {
        match *&self {
            MonoShape::Circle { inner } => inner.name(),
            MonoShape::Square { inner } => inner.name(),
            MonoShape::Triangle { inner } => inner.name(),
            MonoShape::EquilateralTriangle { inner } => inner.name(),
            MonoShape::RightTriangle { inner } => inner.name(),
        }
    }

    fn area(&self) -> f64 {
        match *&self {
            MonoShape::Circle { inner } => inner.area(),
            MonoShape::Square { inner } => inner.area(),
            MonoShape::Triangle { inner } => inner.area(),
            MonoShape::EquilateralTriangle { inner } => inner.area(),
            MonoShape::RightTriangle { inner } => inner.area(),
        }
    }

    fn perimeter(&self) -> f64 {
        match *&self {
            MonoShape::Circle { inner } => inner.perimeter(),
            MonoShape::Square { inner } => inner.perimeter(),
            MonoShape::Triangle { inner } => inner.perimeter(),
            MonoShape::EquilateralTriangle { inner } => inner.perimeter(),
            MonoShape::RightTriangle { inner } => inner.perimeter(),
        }
    }
}

impl std::fmt::Display for MonoShape {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *&self {
            MonoShape::Circle { inner } => {
                write!(f, "{}", &inner)
            }
            MonoShape::Square { inner } => {
                write!(f, "{}", &inner)
            }
            MonoShape::Triangle { inner } => {
                write!(f, "{}", &inner)
            }
            MonoShape::EquilateralTriangle { inner } => {
                write!(f, "{}", &inner)
            }
            MonoShape::RightTriangle { inner } => {
                write!(f, "{}", &inner)
            }
        }
    }
}

impl From<Circle> for MonoShape {
    fn from(shape: Circle) -> Self {
        MonoShape::Circle { inner: shape }
    }
}

impl From<Square> for MonoShape {
    fn from(shape: Square) -> Self {
        MonoShape::Square { inner: shape }
    }
}

impl From<Triangle> for MonoShape {
    fn from(shape: Triangle) -> Self {
        MonoShape::Triangle { inner: shape }
    }
}

impl From<EquilateralTriangle> for MonoShape {
    fn from(shape: EquilateralTriangle) -> Self {
        MonoShape::EquilateralTriangle { inner: shape }
    }
}

impl From<RightTriangle> for MonoShape {
    fn from(shape: RightTriangle) -> Self {
        MonoShape::RightTriangle { inner: shape }
    }
}

impl FromStr for MonoShape {
    type Err = CreationError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut tokens = line.trim().split(";");

        let Some(name) = tokens.next() else {
            return Err(CreationError::MalformedLineError(
                compact_str::format_compact!(
                    "Line '{line}' did not include a name"
                ),
            ));
        };

        let Some(the_rest) = tokens.next() else {
            return Err(CreationError::MalformedLineError(
                compact_str::format_compact!(
                    "Line '{line}' did not have exactly one (1) semicolon"
                ),
            ));
        };

        let values: Vec<f64> = the_rest
            .split_whitespace()
            .map(|token| token.parse())
            .try_collect()?;

        let shape = MonoFactory::create_with(name, &values)?;

        Ok(shape)
    }
}

//------------------------------------------------------------------------------
// Mono Factory Implementation
//------------------------------------------------------------------------------
type MonoDefaultFunction = dyn Fn() -> MonoShape;
type MonoDimFunction = dyn Fn(&[f64]) -> Result<MonoShape, CreationError>;
type MonoShapeTuple<'a> = (&'a str, Box<MonoDefaultFunction>, Box<MonoDimFunction>);

#[rustfmt::skip]
const CREATE_MONOSHAPE: LazyCell<[MonoShapeTuple; 5]> = LazyCell::new(|| {
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

pub struct MonoFactory;

impl CreationFactory for MonoFactory {
    type Item = MonoShape;
    type Error = CreationError;

    /// Create a Shape
    ///
    /// # Arguments
    ///
    ///   * `name` shape to be created
    ///
    fn create_default(name: &str) -> Result<Self::Item, Self::Error> {
        match CREATE_MONOSHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
        {
            Some((_, creation_op, _)) => Ok(creation_op()),
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
        match CREATE_MONOSHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
        {
            None => Err(CreationError::UnknownShapeError(name.into())),
            Some((_, _, creation_op)) => creation_op(&dims),
        }
    }
}

impl FactoryDirectory for MonoFactory {
    /// Determine whether a given shape is known
    ///
    /// # Arguments
    ///
    ///  * `name` the shape for which to query
    ///
    fn is_known(name: &str) -> bool {
        CREATE_MONOSHAPE
            .iter()
            .find(|(shape_name, _, _)| shape_name == &name)
            .is_some()
    }

    fn number_known() -> usize {
        CREATE_MONOSHAPE.len()
    }

    /// List the known shapes, one per line
    ///
    fn list_known() -> String {
        CREATE_MONOSHAPE
            .iter()
            .map(|(name, _, _)| format!("  {}", name))
            .join("\n")
            + "\n"
    }
}
