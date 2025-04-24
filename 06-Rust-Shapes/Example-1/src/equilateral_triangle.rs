use crate::shape::Shape;

use std::fmt;

/// Define a General EquilateralTriangle with 3 sides.
#[derive(Clone)]
pub struct EquilateralTriangle {
    pub side: f64,
}

impl EquilateralTriangle {
    pub fn new() -> Self {
        EquilateralTriangle { side: 1.0 }
    }

    pub fn with_side(s: f64) -> Self {
        EquilateralTriangle { side: s }
    }

    /// Compute the height using
    ///
    /// $height = \frac{5}{4}side$
    fn height(&self) -> f64 {
        (3_f64.sqrt() / 2.0) * self.side
    }
}

impl Shape for EquilateralTriangle {
    fn name(&self) -> &'static str {
        "Equilateral Triangle"
    }

    /// Compute perimeter by adding 3 sides together.
    fn perimeter(&self) -> f64 {
        3.0 * self.side
    }

    /// Compute the area using Heron's Formula. Use
    ///
    /// $s = \frac{1}{2}Perimeter$
    /// and
    /// $Area = \sqrt{ s(s-a)(s-b)(s-c) }$
    fn area(&self) -> f64 {
        3_f64.sqrt() / 4_f64 * self.side.powi(2)
    }
}

impl fmt::Display for EquilateralTriangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:12}:{:>24}", "Name", self.name())?;
        writeln!(f, "{:12}:{:>24.4}", "Side", self.side);
        writeln!(f, "{:12}:{:>24.4}", "Height", self.height())?;
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

    #[test]
    fn test_default_constructor() {
        let generic = EquilateralTriangle::new();

        assert_that!(generic.name(), equal_to("Equilateral Triangle"));
        assert_that!(generic.side, close_to(1.0, 0.01));
    }

    #[test]
    fn test_with_side() {
        let fancy = EquilateralTriangle::with_side(3.0);

        assert_that!(fancy.name(), equal_to("Equilateral Triangle"));
        assert_that!(fancy.side, close_to(3.0, 0.01));
    }

    #[test]
    fn test_height() {
        let generic = EquilateralTriangle::new();
        let fancy = EquilateralTriangle::with_side(3.0);

        let expected: f64 = 3_f64.sqrt() / 2.0;
        assert_that!(generic.height(), close_to(expected, 0.01));

        let expected: f64 = 3_f64.sqrt() * 3.0 / 2.0;
        assert_that!(fancy.height(), close_to(expected, 0.01));
    }

    #[test]
    fn test_area() {
        let generic = EquilateralTriangle::new();
        let fancy = EquilateralTriangle::with_side(3.0);

        // Based on 1/2 base * height (side=1)
        let expected_area: f64 = 3.0_f64.sqrt() / 4.0;
        assert_that!(generic.area(), close_to(expected_area, 1e-8));

        // Based on 1/2 side * height (side=3)
        let expected_area: f64 = 3.0 * (27.0_f64).sqrt() / 4.0;
        assert_that!(fancy.area(), close_to(expected_area, 1e-8));
    }

    #[test]
    fn test_perimeter() {
        let generic = EquilateralTriangle::new();
        let fancy = EquilateralTriangle::with_side(3.0);

        assert_that!(generic.perimeter(), close_to(3.0, 1e-8));
        assert_that!(fancy.perimeter(), close_to(9.0, 1e-8));
    }

    #[test]
    fn test_str() {
        let generic = EquilateralTriangle::new();
        let fancy = EquilateralTriangle::with_side(2.0);
        let fancy_str = fancy.to_string();

        assert!(fancy_str.starts_with("Name"));
        assert!(fancy_str.contains("Equilateral Triangle"));
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
