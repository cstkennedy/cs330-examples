use itertools::{Itertools, MinMaxResult};
use ordered_float::OrderedFloat;
use std::iter::Sum;

// use std::io::BufReader;
// use std::fs::File;
// use std::env;
use std::vec::Vec;

use eyre::{self, Result, WrapErr};

use room_renovation::error::HouseError;
use room_renovation::prelude::*;

///
/// Compute the area of a room and the cost of
/// flooring for the room
///
#[cfg(not(tarpaulin_include))]
fn main() -> eyre::Result<()> {
    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 2 {
        eyre::bail!("Usage: {} room_filename", argv[0]);
    }

    let house = HouseParser::read_from_file(&argv[1], |ins| HouseParser::read_house(ins))?
        .wrap_err("Input did not contain at least one valid room line")?;

    let duplicate_house = upgrade_flooring(house.iter())
        .wrap_err("Reference 'house' was invalid (THIS SHOULD NEVER HAPPEN)")?;

    println!("{house}");
    println!("{duplicate_house}");

    let costs: Vec<_> = duplicate_house
        .iter()
        .map(|room| 0.90 * room.flooring_cost())
        .map(OrderedFloat)
        .collect();

    for room_cost in costs.iter() {
        println!("{room_cost:.2}")
    }

    let total: f64 = *OrderedFloat::sum(costs.iter());

    println!("Total: {total:.2}");

    let result = costs.iter().minmax();
    if let MinMaxResult::MinMax(ex_min, ex_max) = result {
        println!("Min  : {ex_min:.2}");
        println!("Max  : {ex_max:.2}");
    }

    println!();

    Ok(())
}

///
/// Take a room and change the flooring
///
/// # Arguments
///
///   * `original_rooms` - Rooms from the orignal house
///
/// # Returns
///
/// House with updated flooring
///
#[rustfmt::skip]
fn upgrade_flooring<'a, I>(original_rooms: I) -> Result<House, HouseError>
    where I: Iterator<Item=&'a Room>
{
    let new_flooring = Flooring::builder()
        .type_name("Stone Bricks".into())
        .unit_cost(12.97)
        .build();

    let house = House::builder()
        .with_name("After Stone Bricks")
        .with_rooms(
            original_rooms
                .map(|room| {
                    Room::builder()
                        .from_existing(room)
                        .with_flooring(new_flooring.clone())
                        .build()
                })
                .collect(),
        )?
        .build();

    Ok(house)

}
