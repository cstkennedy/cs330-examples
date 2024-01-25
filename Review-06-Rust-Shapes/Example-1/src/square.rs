use crate::shape::Shape;

use std::fmt;

/// Define a Square (i.e., an ellipse with the same major and minor axis)
#[derive(Clone)]
pub struct Square {
    pub side: f64,
}

impl Square {
    /// Create a Square with a default side of 1.
    pub fn new() -> Self {
        Square { side: 1.0 }
    }

    /// Create a Square.
    ///
    /// # Args
    ///
    /// * `s` - desired side
    ///
    pub fn with_side(s: f64) -> Self {
        Square { side: s }
    }
}

impl Shape for Square {
    fn name(&self) -> &'static str {
        "Square"
    }

    /// Compute the area using $s^2$
    fn area(&self) -> f64 {
        self.side.powi(2)
    }

    /// Compute the perimeter using $4s$
    fn perimeter(&self) -> f64 {
        4.0 * self.side
    }
}

impl fmt::Display for Square {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:12}:{:>24}", "Name", self.name())?;
        writeln!(f, "{:12}:{:>24.4}", "Side", self.side);
        writeln!(f, "{:12}:{:>24.4}", "Perimeter", self.perimeter())?;
        writeln!(f, "{:12}:{:>24.4}", "Area", self.area())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr;
    use hamcrest2::prelude::*;

    #[test]
    fn test_default_constructor() {
        let generic = Square::new();

        assert_that!(generic.name(), equal_to("Square"));
        assert_that!(generic.side, close_to(1.0, 0.01));
    }

    #[test]
    fn test_with_side() {
        let fancy = Square::with_side(2.0);

        assert_that!(fancy.name(), equal_to("Square"));
        assert_that!(fancy.side, close_to(2.0, 0.01));
    }

    #[test]
    fn test_area() {
        let generic = Square::new();
        let fancy = Square::with_side(2 as f64);

        assert_that!(generic.area(), close_to(1.0, 1e-6));

        assert_that!(fancy.area(), close_to(4.0, 1e-6));
    }

    #[test]
    fn test_perimeter() {
        let generic = Square::new();
        let fancy = Square::with_side(2.0);
        assert_that!(generic.perimeter(), close_to(4.0, 1e-6));

        assert_that!(fancy.perimeter(), close_to(8.0, 1e-6));
    }

    #[test]
    fn test_deep_copy() {
        let generic = Square::new();
        let fancy = Square::with_side(2.0);
        let a_copy = fancy.clone();

        assert_that!(ptr::eq(&a_copy, &fancy), is(false));

        // I really should have defined __eq__
        assert_that!(a_copy.side, close_to(fancy.side, 0.001));
    }

    #[test]
    fn test_str() {
        let generic = Square::new();
        let fancy = Square::with_side(2.0);
        let fancy_str = fancy.to_string();

        assert!(fancy_str.starts_with("Name"));
        assert!(fancy_str.contains("Square"));
        assert!(fancy_str.ends_with("\n"));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Perimeter",
                                            fancy.perimeter())));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Area",
                                            fancy.area())));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Side",
                                            fancy.side)));

        assert!(fancy_str.ends_with("\n"));
    }
}
