use std::fmt;

/// Define the interface for a 2D shape
pub trait Shape {
    /// Return the name of the shape as a fixed string
    fn name(&self) ->&'static str;

    /// Compute the area of a 2D shape
    fn area(&self) -> f64;

    /// Compute the perimeter of a 2D shape
    fn perimeter(&self) -> f64;
}


