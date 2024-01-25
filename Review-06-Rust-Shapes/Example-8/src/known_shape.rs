use std::fmt;

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

impl From<Triangle> for KnownShape {
    fn from(item: Triangle) -> Self {
        KnownShape::Triangle(item)
    }
}

impl From<RightTriangle> for KnownShape {
    fn from(item: RightTriangle) -> Self {
        KnownShape::RightTriangle(item)
    }
}

impl From<EquilateralTriangle> for KnownShape {
    fn from(item: EquilateralTriangle) -> Self {
        KnownShape::EquilateralTriangle(item)
    }
}

impl From<Square> for KnownShape {
    fn from(item: Square) -> Self {
        KnownShape::Square(item)
    }
}

impl From<Circle> for KnownShape {
    fn from(item: Circle) -> Self {
        KnownShape::Circle(item)
    }
}

impl From<&[f64]> for Triangle {
    fn from(dims: &[f64]) -> Self {
        Triangle::with_sides(dims[0], dims[1], dims[2])
    }
}

impl From<&[f64]> for RightTriangle  {
    fn from(dims: &[f64]) -> Self {
        RightTriangle::with_base_height(dims[0], dims[1])
    }
}

impl From<&[f64]> for EquilateralTriangle {
    fn from(dims: &[f64]) -> Self {
        EquilateralTriangle::with_side(dims[0])
    }
}

impl From<&[f64]> for Square {
    fn from(dims: &[f64]) -> Self {
        Square::with_side(dims[0])
    }
}

impl From<&[f64]> for Circle {
    fn from(dims: &[f64]) -> Self {
        Circle::with_radius(dims[0])
    }
}

