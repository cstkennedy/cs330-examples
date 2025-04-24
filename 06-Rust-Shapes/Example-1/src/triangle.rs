use crate::shape::Shape;

use std::fmt;

/// Define a General Triangle with 3 sides.
#[derive(Clone)]
pub struct Triangle {
    pub side_a: f64,
    pub side_b: f64,
    pub side_c: f64,
}

impl Triangle {
    pub fn new() -> Self {
        Triangle { side_a: 1.0, side_b: 1.0, side_c: 1.0 }
    }

    pub fn with_sides(a: f64, b: f64, c:f64) -> Self {
        Triangle { side_a: a, side_b: b, side_c: c }
    }
}


impl Shape for Triangle {
    fn name(&self) -> &'static str {
        "Triangle"
    }

    /// Compute perimeter by adding 3 sides together.
    fn perimeter(&self) -> f64 {
        self.side_a + self.side_b + self.side_c
    }

    /// Compute the area using Heron's Formula. Use
    ///
    /// $s = \frac{1}{2}Perimeter$
    /// and
    /// $Area = \sqrt{ s(s-a)(s-b)(s-c) }$
    fn area(&self) -> f64 {

        let s = self.perimeter() / 2.0;

        (s * (s - self.side_a) * (s - self.side_b) * (s - self.side_c)).sqrt()
    }
}

impl fmt::Display for Triangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "{:12}:{:>24}", "Name", self.name())?;
        writeln!(f, "{:12}:{:>24.4}", "Side A", self.side_a);
        writeln!(f, "{:12}:{:>24.4}", "Side B", self.side_b);
        writeln!(f, "{:12}:{:>24.4}", "Side C", self.side_c);
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
        let generic = Triangle::new();

        assert_that!(generic.name(), equal_to("Triangle"));
        assert_that!(generic.side_a, close_to(1.0, 0.01));
        assert_that!(generic.side_b, close_to(1.0, 0.01));
        assert_that!(generic.side_c, close_to(1.0, 0.01));
    }

    #[test]
    fn test_with_sides() {
        let generic = Triangle::with_sides(3.0, 4.0, 5.0);

        assert_that!(generic.name(), equal_to("Triangle"));
        assert_that!(generic.side_a, close_to(3.0, 0.01));
        assert_that!(generic.side_b, close_to(4.0, 0.01));
        assert_that!(generic.side_c, close_to(5.0, 0.01));
    }

    #[test]
    fn test_area() {
        let generic = Triangle::new();
        let fancy = Triangle::with_sides(3.0, 4.0, 5.0);

        // Based on 1/2 base * height (side=1)
        let expected_area: f64 = 3.0_f64.sqrt() / 4.0;
        assert_that!(generic.area(), close_to(expected_area, 1e-8));

        // Based on 1/2 side * height (side=3)
        let expected_area: f64 = 0.5 * 3.0 * 4.0;
        assert_that!(fancy.area(), close_to(expected_area, 1e-8));
    }

    #[test]
    fn test_perimeter() {
        let generic = Triangle::new();
        let fancy = Triangle::with_sides(3.0, 4.0, 5.0);

        assert_that!(generic.perimeter(), close_to(3.0, 1e-8));
        assert_that!(fancy.perimeter(), close_to(12.0, 1e-8));
    }

    #[test]
    fn test_str() {
        let fancy = Triangle::with_sides(3.0, 4.0, 5.0);
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
                                            "Side A",
                                            fancy.side_a)));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Side B",
                                            fancy.side_b)));

        assert!(fancy_str.contains(&format!("{:12}:{:>24.4}",
                                            "Side C",
                                            fancy.side_c)));

        assert!(fancy_str.ends_with("\n"));
    }


}
