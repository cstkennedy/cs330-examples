extern crate itertools;
extern crate ordered_float;
extern crate room_renovation;

use itertools::{Itertools, MinMaxResult};
use ordered_float::OrderedFloat;

// use std::io::BufReader;
// use std::fs::File;
// use std::env;
use std::vec::Vec;

use room_renovation::flooring::{Flooring, FlooringBuilder};
use room_renovation::house::House;
use room_renovation::room::{DimensionSet, Room, RoomBuilder};

///
/// Compute the area of a room and the cost of
/// flooring for the room
/// <p>
/// Let us Review the use of reference and pointer variables.
/// <p>
/// We will use these when we implement the iterator interface.
///
#[cfg_attr(tarpaulin, skip)]
fn main() {
    let house = build_house();

    println!("{}", house);

    // Upgrade the flooring in a second duplicate house
    let duplicate_house = upgrade_flooring(&house);

    println!("house == duplicate_house -> {}", (house == duplicate_house));
    println!(
        "&house == &duplicate_house -> {}",
        std::ptr::eq(&house, &duplicate_house)
    );

    println!("{}", house);
    println!("{}", duplicate_house);

    let costs: Vec<f64> = duplicate_house
        .iter()
        .map(|r| discount_flooring(r))
        .collect();

    for room_cost in costs.iter() {
        println!("{:.2}", room_cost)
    }

    let total: f64 = costs.iter().sum();

    println!("Total: {:.2}", total);

    match costs.iter().minmax_by_key(|c| OrderedFloat(**c)) {
        MinMaxResult::MinMax(ex_min, ex_max) => {
            println!("Min  : {:.2}", ex_min);
            println!("Max  : {:.2}", ex_max);
        }
        _ => {}
    }

    println!();
}

///
/// Build our example house
///
fn build_house() -> House {
    let mut house = House::new();

    house.add_room(Room {
        name: "Laundry Room".to_string(),
        dimensions: DimensionSet::new(8f64, 4f64),
        flooring: Flooring {
            unit_cost: 1.95f64,
            type_name: "Laminate".to_string(),
        },
    });

    let kitchen = RoomBuilder::new()
        .with_name("Kitchen")
        .with_dimensions(20f64, 12f64)
        .with_flooring(
            FlooringBuilder::new()
                .with_specific_name("Tile")
                .with_unit_cost(3.87f64)
                .build()
                .unwrap(),
        )
        .build()
        .unwrap();

    house.add_room(kitchen);

    house.add_room(
        RoomBuilder::new()
            .with_name("Storage Room")
            .with_dimensions(16f64, 16f64)
            .with_flooring(
                FlooringBuilder::new()
                    .with_specific_name("Birch Wood")
                    .with_unit_cost(4.39f64)
                    .build()
                    .unwrap(),
            )
            .build()
            .unwrap(),
    );

    house
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
fn upgrade_flooring(original: &House) -> House {
    let mut modified = original.clone();

    /*
    for room in modified.iter_mut() {
        room.set_flooring("Stone Bricks", 12.97);
    }
    */

    modified
        .iter_mut()
        .for_each(|room| room.set_flooring("Stone Bricks", 12.97));

    modified.set_name("After Stone Bricks");

    modified
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
