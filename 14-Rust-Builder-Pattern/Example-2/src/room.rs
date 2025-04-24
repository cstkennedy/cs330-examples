use crate::error::*;
use crate::flooring::*;
use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display; //,Formatter,Result};

#[derive(Clone, Debug)]
pub struct DimensionSet {
    pub length: f64,
    pub width: f64,
}

impl DimensionSet {
    pub fn new(l: f64, w: f64) -> Self {
        DimensionSet {
            length: l,
            width: w,
        }
    }
}

impl Default for DimensionSet {
    fn default() -> Self {
        DimensionSet::new(1f64, 1f64)
    }
}

impl From<(f64, f64)> for DimensionSet {
    fn from(dims: (f64, f64)) -> Self {
        DimensionSet::new(dims.0, dims.1)
    }
}

#[derive(Clone, Debug)]
pub struct Room {
    pub name: String,
    pub dimensions: DimensionSet,
    pub flooring: Flooring,
}

impl Room {
    pub fn builder() -> RoomBuilder<NoName, NoDimensions, NoFlooring> {
        RoomBuilder::new()
    }

    /// Compute the area of flooring for a room.
    pub fn area(&self) -> f64 {
        self.dimensions.width * self.dimensions.length
    }

    /// Compute the flooring cost based on `self.area()` and unit cost.
    pub fn flooring_cost(&self) -> f64 {
        self.area() * self.flooring.unit_cost
    }
}

impl Default for Room {
    fn default() -> Self {
        Room {
            name: "Generic".to_string(),
            dimensions: Default::default(),
            flooring: Default::default(),
        }
    }
}

impl Display for Room {
    #[allow(unused_must_use)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Room ({})", self.name);
        writeln!(f, "  {:<6}: {:>8.1}", "Length", self.dimensions.length);
        writeln!(f, "  {:<6}: {:>8.1}", "Width", self.dimensions.width);
        writeln!(f, "  {:<6}: {:>8.1}", "Area", self.area());
        writeln!(f);
        writeln!(f, "  Flooring  : {}", self.flooring.type_name);
        writeln!(f, "  Unit Cost : $ {:>8.2}", self.flooring.unit_cost);
        writeln!(f, "  Total Cost: $ {:>8.2}", self.flooring_cost())
    }
}

impl PartialOrd for Room {
    fn partial_cmp(&self, rhs: &Self) -> Option<Ordering> {
        if self.name.eq(&rhs.name) {
            self.area().partial_cmp(&rhs.area())
        } else {
            self.name.partial_cmp(&rhs.name)
        }
    }
}

impl PartialEq for Room {
    fn eq(&self, rhs: &Self) -> bool {
        return self.name.eq(&rhs.name) && self.area().eq(&rhs.area());
    }
}

//------------------------------------------------------------------------------
#[derive(Clone, Copy, Default, PartialEq)]
pub struct NoName;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct NoDimensions;

#[derive(Clone, Copy, Default, PartialEq)]
pub struct NoFlooring;

/// The RoomBuilder's state is defined by three generics
///   - SN - (State Name) whether the name is set
///   - SD - (State Dimensions) whether the length and width are set
///   - SF - (State Flooring) whether the flooring is set
///
#[derive(Clone, Default, PartialEq)]
pub struct RoomBuilder<SN, SD, SF> {
    name: SN,
    length: SD,
    width: SD,
    flooring: SF,
}

impl RoomBuilder<NoName, NoDimensions, NoFlooring> {
    pub fn new() -> Self {
        RoomBuilder {
            name: NoName,
            length: NoDimensions,
            width: NoDimensions,
            flooring: NoFlooring,
        }
    }
}

impl RoomBuilder<NoName, NoDimensions, NoFlooring> {
    pub fn from_existing(self, room: &Room) -> RoomBuilder<String, f64, Flooring> {
        RoomBuilder {
            name: room.name.clone(),
            width: room.dimensions.width,
            length: room.dimensions.length,
            flooring: room.flooring.clone(),
        }
    }

    pub fn with_name(self, nme: &str) -> RoomBuilder<String, NoDimensions, NoFlooring> {
        RoomBuilder {
            name: nme.into(),
            length: self.length,
            width: self.width,
            flooring: self.flooring,
        }
    }
}

impl RoomBuilder<String, NoDimensions, NoFlooring> {
    pub fn with_dimensions(self, l: f64, w: f64) -> RoomBuilder<String, f64, NoFlooring> {
        RoomBuilder {
            name: self.name,
            length: l,
            width: w,
            flooring: self.flooring,
        }
    }
}

impl<SF> RoomBuilder<String, f64, SF> {
    pub fn with_flooring(self, flooring: Flooring) -> RoomBuilder<String, f64, Flooring> {
        RoomBuilder {
            name: self.name,
            length: self.length,
            width: self.width,
            flooring: flooring,
        }
    }
}

impl RoomBuilder<String, f64, Flooring> {
    pub fn with_dimensions(mut self, l: f64, w: f64) -> Self {
        self.length = l;
        self.width = w;

        self
    }

    pub fn build(self) -> Result<Room, BuildError> {
        let room = Room {
            name: self.name,
            dimensions: DimensionSet::new(self.length, self.width),
            flooring: self.flooring,
        };

        Ok(room)
    }
}
