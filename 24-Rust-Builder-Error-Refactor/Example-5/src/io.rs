use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;
use log;

use crate::error::{ParseRoomError, PlainTextParserError};
use crate::flooring::{Cost, Flooring};
use crate::house::House;
use crate::room::{DimensionSet, Room};

impl FromStr for Room {
    type Err = ParseRoomError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, the_rest) = line
            .split_once(";")
            .ok_or_else(|| PlainTextParserError::missing_semicolon(line))?;
        let mut the_rest = the_rest.split_whitespace();

        // 3. Dimensions...
        let length = the_rest
            .next()
            .ok_or_else(|| PlainTextParserError::too_few_tokens(line))?
            .parse()?;

        let width = the_rest
            .next()
            .ok_or_else(|| PlainTextParserError::too_few_tokens(line))?
            .parse()?;

        let dimensions = DimensionSet::try_from((length, width))?;

        // 4. Unit cost...
        let unit_cost: Cost = the_rest
            .next()
            .ok_or_else(|| PlainTextParserError::too_few_tokens(line))?
            .parse::<f64>()?
            .try_into()?;

        // The flooring name might contain spaces.
        // Combine the remainder of the line.
        let flooring_name = {
            let flooring_name = the_rest.join(" ").trim().to_owned();

            if flooring_name.is_empty() {
                return Err(PlainTextParserError::too_few_tokens(line).into());
            }

            flooring_name
        };

        let room = Room::builder()
            .with_name(name)
            .with_checked_dimensions(dimensions)
            .with_flooring(
                Flooring::builder()
                    .with_name(flooring_name)
                    .with_unit_cost(unit_cost)
                    .build(),
            )
            .build();

        Ok(room)
    }
}

pub trait Parser {
    type Item;
    type Error;
}

pub trait PlainTextParser: Parser {
    fn read_from_file<P>(filename: P) -> Result<Self::Item, Self::Error>
    where
        P: AsRef<Path>;

    fn read_item(buffer: impl BufRead) -> Result<Self::Item, Self::Error>;
}

pub struct HouseParser;

impl Parser for HouseParser {
    type Item = House;
    type Error = ParseRoomError;
}

impl PlainTextParser for HouseParser {
    fn read_from_file<P>(filename: P) -> Result<Self::Item, Self::Error>
    where
        P: AsRef<Path>,
    {
        let filename = filename.as_ref();
        let file = File::open(filename)?;
        let ins = BufReader::new(file);

        Self::read_item(ins)
    }

    fn read_item(buffer: impl BufRead) -> Result<Self::Item, Self::Error> {
        let house = House::builder()
            .with_rooms(
                buffer
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
                    .flat_map(|(_, result)| result),
            )?
            .build();

        Ok(house)
    }
}
