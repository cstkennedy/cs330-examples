extern crate itertools;
extern crate ordered_float;
extern crate room_renovation;

use itertools::{Itertools, MinMaxResult};
use ordered_float::OrderedFloat;

// use std::io::BufReader;
// use std::fs::File;
// use std::env;
use std::vec::Vec;

use room_renovation::house::House;
use room_renovation::room::{DimensionSet, Flooring, Room};

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
    // Construct, build, and print a house
    let mut house = House::new();

    // std::istringstream fakeInputFile(ROOM_DATA);
    build_house(&mut house);

    println!("{}", house);

    // Upgrade the flooring in a second duplicate house
    let duplicate_house = upgrade_flooring(&house);

    /*
    // cout.setf(ios::boolalpha);
    cout << "\n"
         << "house == duplicateHouse   -> "
         << (house == duplicateHouse)
         << "\n"
         << "&house == &duplicateHouse -> "
         << (&house == &duplicateHouse)
         << "\n"
         << "\n";
    */
    println!("house == duplicate_house -> {}", (house == duplicate_house));
    println!(
        "&house == &duplicate_house -> {}",
        std::ptr::eq(&house, &duplicate_house)
    );

    println!("{}", house);
    println!("{}", duplicate_house);

    /*
    // Get all the flooring costs with a 10% discount
    vector<double> costs(duplicateHouse.size());
    std::transform(duplicateHouse.begin(), duplicateHouse.end(), costs.begin(),
                   discountFlooring);
    */
    let costs: Vec<f64> = duplicate_house
        .iter()
        .map(|r| discount_flooring(r))
        .collect();

    /*
    std::copy(costs.begin(), costs.end(),
              std::ostream_iterator<double>(std::cout, "\n"));
    */
    for room_cost in costs.iter() {
        println!("{:.2}", room_cost)
    }

    /*
    costs.iter()
        .for_each(|room_cost| println!("{:.2}", room_cost));
    */

    /*
    // Print the sum, min, max -> D.R.Y!
    cout << "Total: "
         << std::accumulate(costs.begin(), costs.end(), 0.0, std::plus<double>())
         << "\n";
    cout << "Min: "
         << *std::min_element(costs.begin(), costs.end())
         << "\n"
         << "Max: "
         << *std::max_element(costs.begin(), costs.end())
         << "\n";
    */

    let total: f64 = costs.iter().sum();
    let min: f64 = *costs.iter().min_by_key(|c| OrderedFloat(**c)).unwrap();
    let max: f64 = *costs.iter().max_by_key(|c| OrderedFloat(**c)).unwrap();

    println!("Total: {:.2}", total);
    println!("Min  : {:.2}", min);
    println!("Max  : {:.2}", max);

    // I would probably use minmax_element and auto
    /*
    std::pair<std::vector<double>::const_iterator,
              std::vector<double>::const_iterator> extremes = std::minmax_element(costs.begin(), costs.end());
    */
    /*
    auto extremes = std::minmax_element(costs.begin(), costs.end());

    cout << "Min: " << *(extremes.first)  << "\n"
         << "Max: " << *(extremes.second) << "\n";
    */
    match costs.iter().minmax_by_key(|c| OrderedFloat(**c)) {
        MinMaxResult::MinMax(ex_min, ex_max) => {
            println!("Min  : {:.2}", ex_min);
            println!("Max  : {:.2}", ex_max);
        }
        _ => {}
    }

    println!();

    // Demo "if let" syntax
    if let MinMaxResult::MinMax(ex_min, ex_max) = costs.iter().minmax_by_key(|c| OrderedFloat(**c))
    {
        println!("Min  : {:.2}", ex_min);
        println!("Max  : {:.2}", ex_max);
    }
}

///
/// Build our example house
///
fn build_house(house: &mut House) {
    house.add_room(Room {
        name: "Laundry Room".to_string(),
        dimensions: DimensionSet::new(8f64, 4f64),
        flooring: Flooring {
            unit_cost: 1.95f64,
            type_name: "Laminate".to_string(),
        },
    });

    let kitchen = Room::default()
        .with_name("Kitchen")
        .with_dimensions(20f64, 12f64)
        .with_flooring("Tile", 3.87f64);
    // .to_owned();

    house.add_room(kitchen);

    house.add_room(
        Room::default()
            .with_name("Storage Room")
            .with_dimensions(16f64, 16f64)
            .with_flooring("Birch Wood", 4.39f64), // .to_owned()
    );
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
