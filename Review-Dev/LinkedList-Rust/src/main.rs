extern crate room_renovation;
extern crate ordered_float;

use ordered_float::OrderedFloat;

use std::io::BufReader;
use std::fs::File;
use std::env;
use std::vec::Vec;

use room_renovation::room::{Room,Flooring,DimensionSet};
use room_renovation::house::{House};

/**
 * Compute the area of a room and the cost of
 * flooring for the room
 * <p>
 * Let us Review the use of reference and pointer variables.
 * <p>
 * We will used these when we implement the iterator interface.
 */
#[cfg_attr(tarpaulin, skip)]
fn main() {
    // Construct, build, and print a house
    let mut house = House::new();

    // std::istringstream fakeInputFile(ROOM_DATA);
    build_house(&mut house);

    println!("{}", house);

    // Upgrade the flooring in a
    // second duplicate house
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
    println!("{}", house);
    println!("{}", duplicate_house);

    /*
    // Get all the flooring costs with a 10% discount
    vector<double> costs(duplicateHouse.size());
    std::transform(duplicateHouse.begin(), duplicateHouse.end(), costs.begin(),
                   discountFlooring);

    std::copy(costs.begin(), costs.end(),
              std::ostream_iterator<double>(std::cout, "\n"));

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

    // I would probably use minmax_element and auto
    /*
    std::pair<std::vector<double>::const_iterator,
              std::vector<double>::const_iterator> extremes = std::minmax_element(costs.begin(), costs.end());
    */
    auto extremes = std::minmax_element(costs.begin(), costs.end());

    cout << "Min: " << *(extremes.first)  << "\n"
         << "Max: " << *(extremes.second) << "\n";
    */

}

/**
 * Build our example house
 */
fn build_house(house: &mut House) {

/*
    Kitchen; 20 12 3.87 Tile
    Storage Room; 16 16 4.39 Birch Wood)
*/

    house.add_room(
        Room {
            name: "Laundry Room".to_string(),
            dimensions: DimensionSet {
                length: 8f64,
                width: 4f64,
            },
            flooring: Flooring {
                unit_cost: 1.95f64,
                type_name: "Laminate".to_string()
            }
        }
    );
    house.add_room(
        Room {
            name: "Kitchen".to_string(),
            dimensions: DimensionSet {
                length: 20f64,
                width: 12f64,
            },
            flooring: Flooring {
                unit_cost: 3.87f64,
                type_name: "Tile".to_string()
            }
        }
    );
    house.add_room(
        Room {
            name: "Storage Room".to_string(),
            dimensions: DimensionSet {
                length: 16f64,
                width: 16f64,
            },
            flooring: Flooring {
                unit_cost: 4.39f64,
                type_name: "Birch Wood".to_string()
            }
        }
    );

}

/**
 * Take a room and change the flooring
 *
 * @param original House to change
 *
 * @return House with the updated flooring
 */
fn upgrade_flooring(original: &House) -> House {
    let mut modified = original.clone();

    for room in modified.rooms.iter_mut() {
        room.set_flooring("Stone Bricks", 12.97);
    }

    modified.set_name("After Stone Bricks");

    modified
}

fn discount_flooring(r: &Room) -> f64 {
    0.90 * r.flooring_cost()
}



