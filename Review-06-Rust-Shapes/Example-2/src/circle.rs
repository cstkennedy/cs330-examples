use crate::shape::Shape;

use std::fmt;

/// Define a Circle (i.e., an ellipse with the same major and minor axis)
#[derive(Clone)]
pub struct Circle {
    pub radius: f64,
}

impl Circle {
    /// Create a Circle with a default radius of 1.
    pub fn new() -> Self {
        Circle { radius: 1.0 }
    }

    /// Create a Circle.
    ///
    /// # Args
    ///
    /// * `r` - desired radius
    ///
    pub fn with_radius(r: f64) -> Self {
        Circle { radius: r }
    }

    /// Compute diameter.
    pub fn diameter(&self) -> f64 {
        2f64 * self.radius
    }
}

impl Shape for Circle {
    fn name(&self) -> &'static str {
        "Circle"
    }

    /// Compute the area using $\pi r^2$
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }

    /// Compute the perimeter using $2\pi r$
    fn perimeter(&self) -> f64 {
        2.0 * std::f64::consts::PI * self.radius
    }
}

impl fmt::Display for Circle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:12}:{:>24}", "Name", self.name())?;
        writeln!(f, "{:12}:{:>24.4}", "Radius", self.radius)?;
        writeln!(f, "{:12}:{:>24.4}", "Diameter", self.diameter())?;
        writeln!(f, "{:12}:{:>24.4}", "Perimeter", self.perimeter())?;
        writeln!(f, "{:12}:{:>24.4}", "Area", self.area())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64;
    use std::ptr;
    use hamcrest2::prelude::*;

    const TAU: f64 = 2.0 * f64::consts::PI;


    #[test]
    fn test_default_constructor() {
        let generic = Circle::new();

        assert_that!(generic.name(), equal_to("Circle"));
        assert_that!(generic.radius, close_to(1.0, 0.01));
    }

    #[test]
    fn test_with_radius() {
        let fancy = Circle::with_radius(2.0);

        assert_that!(fancy.name(), equal_to("Circle"));
        assert_that!(fancy.radius, close_to(2.0, 0.01));
    }

    #[test]
    fn test_diameter() {
        let generic = Circle::new();
        let fancy = Circle::with_radius(2 as f64);

        assert_that!(generic.diameter(), close_to(2 as f64, 1e-6));
        assert_that!(fancy.diameter(), close_to(4 as f64, 1e-6));
    }

    #[test]
    fn test_area() {
        let generic = Circle::new();
        let fancy = Circle::with_radius(2 as f64);

        assert_that!(generic.area(),
                     close_to(f64::consts::PI * generic.radius.powi(2), 0.05));

        assert_that!(fancy.area(),
                     close_to(f64::consts::PI * fancy.radius.powi(2), 0.05));
    }

    #[test]
    fn test_perimeter() {
        let generic = Circle::new();
        let fancy = Circle::with_radius(2.0);
        assert_that!(generic.perimeter(),
                     close_to(TAU * generic.radius, 0.05));

        assert_that!(fancy.perimeter(),
                     close_to(TAU * fancy.radius, 0.05));
    }

    #[test]
    fn test_deep_copy() {
        let generic = Circle::new();
        let fancy = Circle::with_radius(2.0);
        let a_copy = fancy.clone();

        assert_that!(ptr::eq(&a_copy, &fancy), is(false));

        // I really should have defined __eq__
        assert_that!(a_copy.radius, close_to(fancy.radius, 0.001));
    }

    #[test]
    fn test_str() {
        let generic = Circle::new();
        let fancy = Circle::with_radius(2.0);
        let fancy_str = fancy.to_string();

        assert!(fancy_str.starts_with("Name"));
        assert!(fancy_str.contains("Circle"));
        assert!(fancy_str.ends_with("\n"));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Perimeter",
                                            fancy.perimeter())));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Area",
                                            fancy.area())));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Radius",
                                            fancy.radius)));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Diameter",
                                            fancy.diameter())));

        assert!(fancy_str.ends_with("\n"));
    }
}
