use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::str::FromStr;

use itertools::Itertools;
use log;

use crate::error::{CostError, DimensionError, HouseError, ParseRoomError};
use crate::flooring::{Cost, Flooring};
use crate::house::House;
use crate::room::{DimensionSet, Room};

impl FromStr for Cost {
    type Err = CostError;

    fn from_str(token: &str) -> Result<Self, Self::Err> {
        let raw_f64: f64 = token.parse()?;

        raw_f64.try_into()
    }
}

const MIN_NUM_ROOM_TOKENS: usize = 4;

impl FromStr for Room {
    type Err = ParseRoomError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let mut tokens = line.split(";");

        // 1. Grab the name
        let Some(name) = tokens.next() else {
            unreachable!()
        };

        let Some(the_rest) = tokens.next() else {
            return Err(ParseRoomError::MissingDelimiter {
                delim: ";".into(),
                line: line.to_owned(),
            }
            .into());
        };

        // 2. Split everything else by whitespace
        let mut the_rest = the_rest.split_whitespace();

        // 3. Dimensions...
        let Some(length_str) = the_rest.next() else {
            return Err(ParseRoomError::TooFewTokens {
                line: line.to_owned(),
            }
            .into());
        };
        let length: f64 = length_str.parse()?;

        let Some(width_str) = the_rest.next() else {
            return Err(ParseRoomError::TooFewTokens {
                line: line.to_owned(),
            }
            .into());
        };
        let width: f64 = width_str.parse()?;
        let dimensions = DimensionSet::try_from((length, width))?;

        // 4. Unit cost...
        let Some(unit_cost) = the_rest.next() else {
            return Err(ParseRoomError::TooFewTokens {
                line: line.to_owned(),
            }
            .into());
        };
        let unit_cost: Cost = unit_cost.parse()?;

        // The flooring name might contain spaces.
        // Combine the remainder of the line.
        let flooring_name = the_rest.join(" ");

        if flooring_name.is_empty() {
            return Err(ParseRoomError::TooFewTokens {
                line: line.to_owned(),
            }
            .into());
        }

        let room = Room::builder()
            .with_name(name)
            .with_checked_dimensions(dimensions)
            .with_flooring(
                Flooring::builder()
                    .with_name(&flooring_name)
                    .with_unit_cost(unit_cost)
                    .build(),
            )
            .build();

        Ok(room)
    }
}

pub struct HouseParser;

impl HouseParser {
    /// Open a file and read in data based on a supplied closure
    ///
    /// # Arguments
    ///
    ///   * `filename` - file from which to read
    ///   * `parse_fn` - parsing function to use
    pub fn read_from_file<T, F>(filename: &str, parse_fn: F) -> Result<T, ParseRoomError>
    where
        F: Fn(BufReader<File>) -> T,
    {
        let file = File::open(filename)?;
        let ins = BufReader::new(file);
        let all_things = parse_fn(ins);

        Ok(all_things)
    }

    pub fn read_house(room_data: impl BufRead) -> Result<House, HouseError> {
        let house = House::builder()
            .with_rooms(
                room_data
                    .lines()
                    .flatten()
                    .filter(|line| !line.is_empty())
                    .map(|line| Room::from_str(&line))
                    .enumerate()
                    .inspect(|(idx, room_result)| {
                        if let Err(error) = room_result {
                            log::warn!("Line #{idx} - {}", error);
                        }
                    })
                    .flat_map(|(_, result)| result)
            )?
            .build();

        Ok(house)
    }
}
