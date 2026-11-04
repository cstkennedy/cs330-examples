use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;
use log;

use crate::error::{ParseRoomError, PlainTextParserError};
use crate::flooring::Flooring;
use crate::house::House;
use crate::room::{DimensionSet, Room};

struct FlooringRecord {
    name: String,
    unit_cost: f64,
}

struct RoomRecord {
    name: String,
    length: f64,
    width: f64,
    flooring: FlooringRecord,
}

impl FromStr for RoomRecord {
    type Err = ParseRoomError;

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (name, the_rest) = line
            .split_once(";")
            .ok_or_else(|| PlainTextParserError::missing_semicolon(line))?;
        let mut the_rest = the_rest.split_whitespace();

        let length = the_rest
            .next()
            .ok_or_else(|| PlainTextParserError::too_few_tokens(line))?
            .parse()?;

        let width = the_rest
            .next()
            .ok_or_else(|| PlainTextParserError::too_few_tokens(line))?
            .parse()?;

        let unit_cost = the_rest
            .next()
            .ok_or_else(|| PlainTextParserError::too_few_tokens(line))?
            .parse()?;

        // The flooring name might contain spaces.
        // Combine the remainder of the line.
        let flooring_name = the_rest.join(" ").trim().to_owned();

        if flooring_name.is_empty() {
            return Err(PlainTextParserError::too_few_tokens(line).into());
        }

        Ok(RoomRecord {
            name: name.into(),
            length: length,
            width: width,
            flooring: FlooringRecord {
                name: flooring_name,
                unit_cost: unit_cost,
            },
        })
    }
}

impl TryFrom<RoomRecord> for Room {
    type Error = ParseRoomError;

    fn try_from(record: RoomRecord) -> Result<Self, Self::Error> {
        let dimensions = DimensionSet::try_from((record.length, record.width))?;

        let room = Room::builder()
            .with_name(&record.name)
            .with_checked_dimensions(dimensions)
            .with_flooring(
                Flooring::builder()
                    .with_name(record.flooring.name)
                    .with_unit_cost(record.flooring.unit_cost.try_into()?)
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
                    .map(|line| {
                        RoomRecord::from_str(&line).and_then(|record| Room::try_from(record))
                    })
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
