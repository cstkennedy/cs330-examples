use itertools::{Itertools, MinMaxResult};
use ordered_float::OrderedFloat;
use std::iter::Sum;
use std::path::Path;

use eyre::{self, WrapErr};
use simple_logger::SimpleLogger;

use room_renovation::io::{PlainTextParser, CsvParser};
use room_renovation::prelude::*;

///
/// Compute the area of a room and the cost of
/// flooring for the room
///
#[cfg(not(tarpaulin_include))]
fn main() -> eyre::Result<()> {
    SimpleLogger::new().with_colors(true).init()?;

    let argv: Vec<String> = std::env::args().collect();

    if argv.len() < 2 {
        eyre::bail!("Usage: {} room_filename", argv[0]);
    }

    let filename = Path::new(&argv[1]);
    let maybe_extension = filename.extension().and_then(|s| s.to_str());

    let house = match maybe_extension {
        Some("txt") => <HouseParser as PlainTextParser>::read_from_file(filename)
            .wrap_err("Input did not contain at least one valid room line")?,
        Some("csv") => <HouseParser as CsvParser>::read_from_file(filename)
            .wrap_err("Input did not contain at least one valid room line")?,
        Some(ext) => {
            eyre::bail!("'{ext}' is not supported")
        }
        _ => {
            todo!()
        }
    };

    let new_flooring = Flooring::builder()
        .with_name("Stone Bricks")
        .with_unit_cost(12.97.try_into()?)
        .build();

    /*
    let duplicate_house = upgrade_flooring(house.iter(), new_flooring)
        .wrap_err("Reference 'house' was invalid (THIS SHOULD NEVER HAPPEN)")?;
    */
    let duplicate_house = House::builder()
        .with_name("After Stone Bricks")
        .with_rooms(house.iter().map(|room| {
            Room::builder()
                .from_existing(room)
                .with_flooring(new_flooring.clone())
                .build()
        }))?
        .build();

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

/*
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
fn upgrade_flooring<'a, I>(original_rooms: I, new_flooring: Flooring) -> Result<House, HouseError>
    where I: Iterator<Item=&'a Room>
{
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
        )?
        .build();

    Ok(house)

}
*/
