use std::path::Path;
use std::str::FromStr;

use itertools::Itertools;
use log;

use serde::{Deserialize, Serialize};

use crate::error::{ParseRoomError, PlainTextParserError};
use crate::flooring::Flooring;
use crate::house::House;
use crate::room::{DimensionSet, Room};

pub mod formats {
    pub struct PlainText;
    pub struct Csv;
    pub struct Toml;
    pub struct Yaml;
}

#[derive(Debug, Deserialize, Serialize)]
struct FlooringRecord {
    #[serde(rename = "flooring_type")]
    name: String,
    unit_cost: f64,
}

#[derive(Debug, Deserialize, Serialize)]
struct RoomRecord {
    name: String,
    length: f64,
    width: f64,

    #[serde(flatten)]
    flooring: FlooringRecord,
}

#[derive(Debug, Deserialize, Serialize)]
struct TomlRoomRecords {
    #[serde(rename = "room")]
    pub rooms: Vec<RoomRecord>,
}

#[derive(Debug, Deserialize, Serialize)]
struct YamlRoomRecords {
    pub rooms: Vec<RoomRecord>,
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

pub trait Parser<Format> {
    type Item;
    type Error;

    fn read_from_file<P>(filename: P) -> Result<Self::Item, Self::Error>
    where
        P: AsRef<Path>,
        <Self as Parser<Format>>::Error: From<std::io::Error>,
    {
        let filename = filename.as_ref();
        let content = std::fs::read_to_string(filename)?;

        // Self::read_item(ins)
        <Self as Parser<Format>>::read_item(content)
    }

    fn read_item(buffer: impl AsRef<str>) -> Result<Self::Item, Self::Error>;
}

pub struct HouseParser;

impl Parser<formats::PlainText> for HouseParser {
    type Item = House;
    type Error = ParseRoomError;

    fn read_item(buffer: impl AsRef<str>) -> Result<Self::Item, Self::Error> {
        let house = House::builder()
            .with_rooms(
                buffer
                    .as_ref()
                    .lines()
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

impl Parser<formats::Csv> for HouseParser {
    type Item = House;
    type Error = ParseRoomError;

    fn read_item(buffer: impl AsRef<str>) -> Result<Self::Item, Self::Error> {
        let mut reader = csv::Reader::from_reader(buffer.as_ref().as_bytes());

        let house = House::builder()
            .with_rooms(
                reader
                    .deserialize::<RoomRecord>()
                    .into_iter()
                    .map(|record| record?.try_into())
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

fn log_warning<T, E>(result: &Result<T, E>)
where
    E: std::error::Error,
{
    if let Err(error) = result {
        log::warn!("{}", error);
    }
}

impl Parser<formats::Toml> for HouseParser {
    type Item = House;
    type Error = ParseRoomError;

    fn read_item(buffer: impl AsRef<str>) -> Result<Self::Item, Self::Error> {
        let records: TomlRoomRecords = toml::from_str(buffer.as_ref())?;
        let rooms = records.rooms;

        let house = House::builder()
            .with_rooms(
                rooms
                    .into_iter()
                    .map(Room::try_from)
                    .inspect(log_warning)
                    .flatten(),
            )?
            .build();

        Ok(house)
    }
}

impl Parser<formats::Yaml> for HouseParser {
    type Item = House;
    type Error = ParseRoomError;

    fn read_item(buffer: impl AsRef<str>) -> Result<Self::Item, Self::Error> {
        let records: YamlRoomRecords = serde_saphyr::from_str(buffer.as_ref())?;
        let rooms = records.rooms;

        let house = House::builder()
            .with_rooms(
                rooms
                    .into_iter()
                    .map(Room::try_from)
                    .inspect(log_warning)
                    .flatten(),
            )?
            .build();

        Ok(house)
    }
}
