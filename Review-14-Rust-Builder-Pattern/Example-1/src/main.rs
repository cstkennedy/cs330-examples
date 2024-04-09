use itertools::{Itertools, MinMaxResult};
use ordered_float::OrderedFloat;

// use std::io::BufReader;
// use std::fs::File;
// use std::env;
use std::vec::Vec;

use room_renovation::flooring::FlooringBuilder;
use room_renovation::house::{House, HouseBuilder};
use room_renovation::io::read_house_from_str;
use room_renovation::room::{Room, RoomBuilder};

const ROOM_DATA: &'static str = r#"
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
    let house = read_house_from_str(ROOM_DATA);

    println!("{house}");

    // Upgrade the flooring in a second duplicate house
    let duplicate_house = upgrade_flooring(&house);

    println!("house == duplicate_house -> {}", (house == duplicate_house));
    println!(
        "&house == &duplicate_house -> {}",
        std::ptr::eq(&house, &duplicate_house)
    );

    println!("{house}");
    println!("{duplicate_house}");

    let costs: Vec<f64> = duplicate_house
        .iter()
        .map(|r| discount_flooring(r))
        .collect();

    for room_cost in costs.iter() {
        println!("{room_cost:.2}")
    }

    let total: f64 = costs.iter().sum();

    println!("Total: {total:.2}");

    /*
    match costs.iter().map(|c| OrderedFloat(*c)).minmax() {
        MinMaxResult::MinMax(ex_min, ex_max) => {
            println!("Min  : {:.2}", ex_min);
            println!("Max  : {:.2}", ex_max);
        }
        _ => {}
    }
    */

    let result = costs.iter().map(|c| OrderedFloat(*c)).minmax();
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
fn upgrade_flooring(original: &House) -> House {
    /*
    let mut bldr = HouseBuilder::new()
        .with_name("After Stone Bricks")

    for room in original.iter() {
        let mut updated_room = room.clone();
        updated_room.set_flooring("Stone Bricks", 12.97);

        bldr = bldr.with_room(updated_room);
    }
    */
    /*
    let house = HouseBuilder::new()
        .with_name("After Stone Bricks")
        .with_rooms(
            &mut original.iter()
            .map(|room| {
                let mut updated_room = room.clone();
                updated_room.set_flooring("Stone Bricks", 12.97);

                updated_room
            })
            .collect::<Vec<Room>>()
        )
        .build()
        .unwrap();

    house
    */

    /*
    HouseBuilder::new()
        .with_name("After Stone Bricks")
        .with_rooms(
            original
                .iter()
                .map(|room| {
                    let mut updated_room = room.clone();
                    updated_room.set_flooring("Stone Bricks", 12.97);

                    updated_room
                })
                .collect::<Vec<Room>>(),
        )
        .build()
        .unwrap()
    */

    let new_flooring = FlooringBuilder::new()
        .with_specific_name("Stone Bricks")
        .with_unit_cost(12.97)
        .build()
        .unwrap();

    HouseBuilder::new()
        .with_name("After Stone Bricks")
        .with_rooms(
            original
                .iter()
                .map(|room| {
                    RoomBuilder::new()
                        .from_existing(room)
                        .with_flooring(new_flooring.clone())
                        .build()
                        .unwrap()
                })
                .collect::<Vec<Room>>(),
        )
        .build()
        .unwrap()
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
