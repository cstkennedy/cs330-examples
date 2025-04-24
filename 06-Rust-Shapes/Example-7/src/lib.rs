#[cfg(test)]
#[macro_use]
extern crate hamcrest2;

pub mod circle;
pub mod equilateral_triangle;
pub mod right_triangle;
pub mod shape;
pub mod square;
pub mod triangle;

pub mod factory;
pub mod parser;

pub mod prelude {
    pub use crate::factory::Factory;
    pub use crate::parser::Parser;
    pub use crate::shape::Shape;
}
