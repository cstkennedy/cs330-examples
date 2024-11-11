use itertools::{Itertools, MinMaxResult};
use ordered_float::OrderedFloat;
use std::iter::Sum;

// use std::io::BufReader;
// use std::fs::File;
// use std::env;
use std::vec::Vec;

use room_renovation::flooring::Flooring;
use room_renovation::house::House;
use room_renovation::io::read_house_from_str;
use room_renovation::room::Room;

const ROOM_DATA: &str = r#"
Laundry Room; 8 4 1.95 Laminate
Kitchen; 20 12 3.87 Tile
Storage Room; 16 16 4.39 Birch Wood
"#;

///
/// Compute the area of a room and the cost of
/// flooring for the room
/// <p>
/// Let us Review the use of reference and pointer variables.
/// <p>
/// We will use these when we implement the iterator interface.
///
#[cfg(not(tarpaulin_include))]
fn main() {
    let house =
        read_house_from_str(ROOM_DATA).expect("Input did not contain at least one valid room line");
    let duplicate_house =
        upgrade_flooring(&house).expect("Reference 'house' was invalid (THIS SHOULD NEVER HAPPEN)");

    println!("{house}");
    println!("{duplicate_house}");

    let costs: Vec<_> = duplicate_house
        .iter()
        .map(discount_flooring)
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
}

///
/// Take a room and change the flooring
///
/// # Arguments
///
///   * `original` - House to change
///
/// # Returns
///
/// House with the updated flooring
///
fn upgrade_flooring(original: &House) -> Option<House> {
    let new_flooring = Flooring::builder()
        .type_name("Stone Bricks".into())
        .unit_cost(12.97)
        .build();

    let bldr = House::builder()
        .with_name("After Stone Bricks")
        .with_rooms(
            original
                .iter()
                .map(|room| {
                    Room::builder()
                        .from_existing(room)
                        .with_flooring(new_flooring.clone())
                        .build()
                })
                .collect::<Vec<Room>>(),
        );

    match bldr {
        Ok(bldr) => Some(bldr.build()),
        Err(_) => None,
    }
}

///
/// Take a room, discount the flooring cost by 90%.
///
/// # Returns
///
/// Discounted flooring cost
///
fn discount_flooring(r: &Room) -> f64 {
    0.90 * r.flooring_cost()
}
