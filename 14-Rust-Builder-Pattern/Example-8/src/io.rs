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

impl TryFrom<(&str, &str)> for DimensionSet {
    type Error = DimensionError;

    fn try_from(dims: (&str, &str)) -> Result<Self, Self::Error> {
        let (length, width): (f64, f64) = (dims.0.parse()?, dims.1.parse()?);

        DimensionSet::try_from((length, width))
    }
}

const MIN_NUM_ROOM_TOKENS: usize = 4;

fn separate_name_from_rest(line: &str) -> Result<(&str, Vec<&str>), ParseRoomError> {
    let tokens = line.split(";").collect::<Vec<&str>>();

    if tokens.len() < 2 {
        return Err(ParseRoomError::MissingDelimiter {
            delim: ";".into(),
            line: line.to_owned(),
        }
        .into());
    }

    // 1. Grab the name
    // 2. Split everything else by whitespace
    let name = tokens[0];
    let the_rest: Vec<&str> = tokens[1].split_whitespace().collect();

    if the_rest.len() < MIN_NUM_ROOM_TOKENS {
        return Err(ParseRoomError::TooFewTokens {
            num_tokens: the_rest.len(),
            line: line.to_owned(),
        }
        .into());
    }

    Ok((name, the_rest))
}

impl FromStr for Room {
    type Err = ParseRoomError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, the_rest) = separate_name_from_rest(&line)?;

        let dimensions = DimensionSet::try_from((the_rest[0], the_rest[1]))?;
        let unit_cost: Cost = the_rest[2].parse()?;

        // The flooring name might contain spaces.
        // Combine the remainder of the line.
        let flooring_name = the_rest.into_iter().skip(3).join(" ");

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

// TODO: Rewrite to use nom parser
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
                    .collect(),
            )?
            .build();

        Ok(house)
    }
}
