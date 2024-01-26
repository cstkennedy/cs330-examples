#[cfg(test)]
#[macro_use]
extern crate hamcrest2;
extern crate shapes;

use hamcrest2::prelude::*;

use shapes::factory;
use shapes::factory::Factory;

use std::io::BufReader;

use stringreader::StringReader;

#[test]
fn test_is_known() {
    let factory = Factory::new();

    assert!(factory.is_known("Circle"));
    assert!(factory.is_known("Square"));
    assert!(factory.is_known("Triangle"));
    assert!(factory.is_known("Right Triangle"));
    assert!(factory.is_known("Equilateral Triangle"));
}

#[test]
fn test_number_known() {
    let factory = Factory::new();

    assert_that!(factory.number_known(), is(equal_to(5)));
}

#[test]
fn test_str() {
    let factory = Factory::new();
    let f_str = factory.list_known();

    assert!(f_str.contains("  Circle"));
    assert!(f_str.contains("  Square"));
    assert!(f_str.contains("  Triangle"));
    assert!(f_str.contains("  Right Triangle"));
    assert!(f_str.contains("  Equilateral Triangle"));
}

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

    let str_reader = StringReader::new(raw_str);
    let str_reader = BufReader::new(str_reader);

    let factory = Factory::new();

    let some_shapes = factory::read_shapes(str_reader, factory);

    assert_that!(some_shapes.len(), is(equal_to(5)));

    assert!(some_shapes[0].to_string().contains("Triangle"));
    assert!(some_shapes[1].to_string().contains("Right Triangle"));
    assert!(some_shapes[2].to_string().contains("Equilateral Triangle"));
    assert!(some_shapes[3].to_string().contains("Square"));
    assert!(some_shapes[4].to_string().contains("Circle"));
}
