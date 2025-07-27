use hamcrest2::prelude::*;
use rstest::*;

use html_color::HtmlColor;

#[fixture]
fn black() -> HtmlColor {
    HtmlColor::default()
}

#[fixture]
fn white() -> HtmlColor {
    HtmlColor::new(255, 255, 255)
}

#[fixture]
fn red() -> HtmlColor {
    HtmlColor::new(255, 0, 0)
}

#[fixture]
fn green() -> HtmlColor {
    HtmlColor::new(0, 255, 0)
}

#[fixture]
fn blue() -> HtmlColor {
    HtmlColor::new(0, 0, 255)
}

#[rstest]
pub fn test_default() {
    let color = HtmlColor::default();

    assert_that!(color.red, is(equal_to(0)));
    assert_that!(color.green, is(equal_to(0)));
    assert_that!(color.blue, is(equal_to(0)));

    assert_that!(color.to_string(), is(equal_to("#000000")));
}

#[rstest]
pub fn test_constructor() {
    let color = HtmlColor::new(11, 5, 255);

    assert_that!(color.red, is(equal_to(11)));
    assert_that!(color.green, is(equal_to(5)));
    assert_that!(color.blue, is(equal_to(255)));

    assert_that!(color.to_string(), is(equal_to("#0B05FF")));
}

#[rstest]
pub fn test_ord() {
    todo!()
}

#[rstest]
pub fn test_eq() {
    todo!()
}

#[rstest]
pub fn test_hash() {
    todo!()
}
