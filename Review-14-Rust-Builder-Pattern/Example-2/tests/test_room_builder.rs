#[cfg(test)]
#[macro_use]

use room_renovation::flooring::*;
use room_renovation::room::*;
use room_renovation::house::*;

use hamcrest2::prelude::*;

#[test]
fn test_dimension_set()
{
    let dims = DimensionSet::default();

    assert_that!(dims.length, is(close_to(1.0, 0.01)));
    assert_that!(dims.width, is(close_to(1.0, 0.01)));

    let dims = DimensionSet::new(2.0, 3.0);

    assert_that!(dims.length, is(close_to(2.0, 0.01)));
    assert_that!(dims.width, is(close_to(3.0, 0.01)));

    let dims: DimensionSet = (4.0, 5.0).into();

    assert_that!(dims.length, is(close_to(4.0, 0.01)));
    assert_that!(dims.width, is(close_to(5.0, 0.01)));
}
