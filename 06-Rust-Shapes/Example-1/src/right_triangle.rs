use crate::shape::Shape;

use std::fmt;

/// Define a General RightTriangle with 3 sides.
#[derive(Clone)]
pub struct RightTriangle {
    pub base: f64,
    pub height: f64,
}

impl RightTriangle {
    pub fn new() -> Self {
        RightTriangle { base: 1.0, height: 1.0 }
    }

    pub fn with_base_height(b: f64, h: f64) -> Self {
        RightTriangle { base: b, height: h }
    }

    /// Compute the hypotenuse using:
    /// $hypotenuse = \sqrt{base^2 + height^2}$
    fn hypotenuse(&self) -> f64 {
        (self.base.powi(2) + self.height.powi(2)).sqrt()
    }
}

impl Shape for RightTriangle {
    fn name(&self) -> &'static str {
        "Right Triangle"
    }

    /// Compute perimeter by adding 3 sides together.
    fn perimeter(&self) -> f64 {
        self.base + self.height + self.hypotenuse()
    }

    /// Compute the area using Heron's Formula. Use
    ///
    /// $s = \frac{1}{2}Perimeter$
    /// and
    /// $Area = \sqrt{ s(s-a)(s-b)(s-c) }$
    fn area(&self) -> f64 {
        0.5 * self.base * self.height
    }
}

impl fmt::Display for RightTriangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:12}:{:>24}", "Name", self.name())?;
        writeln!(f, "{:12}:{:>24.4}", "Base", self.base);
        writeln!(f, "{:12}:{:>24.4}", "Height", self.height);
        writeln!(f, "{:12}:{:>24.4}", "Hypotenuse", self.hypotenuse())?;
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
        let generic = RightTriangle::new();

        assert_that!(generic.name(), equal_to("Right Triangle"));
        assert_that!(generic.base, close_to(1.0, 0.01));
        assert_that!(generic.height, close_to(1.0, 0.01));
        assert_that!(generic.hypotenuse(), close_to(2.0_f64.sqrt(), 0.0001));
    }

    #[test]
    fn test_with_base_height() {
        let fancy = RightTriangle::with_base_height(2.0, 3.0);

        assert_that!(fancy.name(), equal_to("Right Triangle"));
        assert_that!(fancy.base, close_to(2.0, 0.01));
        assert_that!(fancy.height, close_to(3.0, 0.01));
        assert_that!(fancy.hypotenuse(), close_to(13_f64.sqrt(), 0.0001));
    }

    #[test]
    fn test_area() {
        let generic = RightTriangle::new();
        let fancy = RightTriangle::with_base_height(2.0, 3.0);

        // Based on 1/2 base * height (side=1)
        let expected_area: f64 = 0.5;
        assert_that!(generic.area(), close_to(expected_area, 1e-8));

        // Based on 1/2 side * height (side=3)
        let expected_area: f64 = 3.0;
        assert_that!(fancy.area(), close_to(expected_area, 1e-8));
    }

    #[test]
    fn test_perimeter() {
        let generic = RightTriangle::new();
        let fancy = RightTriangle::with_base_height(2.0, 3.0);

        let expected = generic.base + generic.height + generic.hypotenuse();
        assert_that!(generic.perimeter(), close_to(expected, 1e-8));

        let expected = fancy.base + fancy.height + fancy.hypotenuse();
        assert_that!(fancy.perimeter(), close_to(expected, 1e-8));
    }
    
    #[test]
    fn test_str() {
        let fancy = RightTriangle::with_base_height(3.0, 4.0);
        let fancy_str = fancy.to_string();

        assert!(fancy_str.starts_with("Name"));
        assert!(fancy_str.contains("Triangle"));
        assert!(fancy_str.ends_with("\n"));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Perimeter",
                                            fancy.perimeter())));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Area",
                                            fancy.area())));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Base",
                                            fancy.base)));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Height",
                                            fancy.height)));

        assert!(fancy_str.ends_with("\n"));
    }
}
