#[cfg(test)]
#[macro_use]
use hamcrest2::prelude::*;

use shapes::prelude::Parser;

use std::io::BufReader;

#[test]
fn test_read_shapes() {
    // The read function should handle (i.e., ignore) leading whitespace
    let raw_str = r#"
        Triangle
        Right Triangle
        Equilateral Triangle
        Square
        Circle
        1337 Haxor"#;

    let str_reader = BufReader::new(raw_str.as_bytes());

    let some_shapes = Parser::read_shapes(str_reader);

    assert_that!(some_shapes.len(), is(equal_to(5)));

    assert!(some_shapes[0].to_string().contains("Triangle"));
    assert!(some_shapes[1].to_string().contains("Right Triangle"));
    assert!(some_shapes[2].to_string().contains("Equilateral Triangle"));
    assert!(some_shapes[3].to_string().contains("Square"));
    assert!(some_shapes[4].to_string().contains("Circle"));
}
