use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display; //,Formatter,Result};

use crate::builder_utils::WrappedType;
use crate::error::{DimensionError};
use crate::flooring::*;

#[derive(Clone, Debug)]
pub struct DimensionSet {
    pub length: f64,
    pub width: f64,
}

impl DimensionSet {
    fn new(l: f64, w: f64) -> Self {
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

impl TryFrom<(f64, f64)> for DimensionSet {
    type Error = DimensionError;

    fn try_from(dims: (f64, f64)) -> Result<Self, Self::Error> {
        let (length, width) = dims;

        match (length > 0.0, width > 0.0) {
            (false, false) => Err(DimensionError::InvalidDimensions(0.0)),
            (false, true) => Err(DimensionError::InvalidLength(0.0)),
            (true, false) => Err(DimensionError::InvalidWidth(0.0)),
            (true, true) => Ok(DimensionSet::new(length, width)),
        }
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
        Room::builder()
            .with_name("Generic")
            .with_checked_dimensions(DimensionSet::default())
            .with_flooring(Flooring::default())
            .build()
    }
}

impl Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Room ({})", self.name)?;
        writeln!(f, "  {:<6}: {:>8.1}", "Length", self.dimensions.length)?;
        writeln!(f, "  {:<6}: {:>8.1}", "Width", self.dimensions.width)?;
        writeln!(f, "  {:<6}: {:>8.1}", "Area", self.area())?;
        writeln!(f)?;
        writeln!(f, "  Flooring  : {}", self.flooring.type_name)?;
        writeln!(f, "  Unit Cost : $ {:>8.2}", self.flooring.unit_cost)?;
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
        self.name.eq(&rhs.name) && self.area().eq(&rhs.area())
    }
}

//------------------------------------------------------------------------------
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NoName;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NoDimensions;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct NoFlooring;

type WithName = WrappedType<String>;
type WithDimensions = WrappedType<DimensionSet>;
type WithFlooring = WrappedType<Flooring>;

/// The RoomBuilder's state is defined by three generics
///   - SN - (State Name) whether the name is set
///   - SD - (State Dimensions) whether the length and width are set
///   - SF - (State Flooring) whether the flooring is set
///
#[derive(Clone, Debug, Default, PartialEq)]
pub struct RoomBuilder<SN, SD, SF> {
    name: SN,
    dimensions: SD,
    flooring: SF,
}

impl RoomBuilder<NoName, NoDimensions, NoFlooring> {
    pub fn new() -> Self {
        RoomBuilder {
            name: NoName,
            dimensions: NoDimensions,
            flooring: NoFlooring,
        }
    }
}

impl RoomBuilder<NoName, NoDimensions, NoFlooring> {
    pub fn from_existing(self, room: &Room) -> RoomBuilder<WithName, WithDimensions, WithFlooring> {
        RoomBuilder {
            name: room.name.clone().into(),
            dimensions: room.dimensions.clone().into(),
            flooring: room.flooring.clone().into(),
        }
    }

    pub fn with_name(self, nme: &str) -> RoomBuilder<WithName, NoDimensions, NoFlooring> {
        RoomBuilder {
            name: nme.into(),
            dimensions: self.dimensions,
            flooring: self.flooring,
        }
    }
}

impl RoomBuilder<WithName, NoDimensions, NoFlooring> {
    pub fn with_checked_dimensions(
        self,
        dimensions: DimensionSet,
    ) -> RoomBuilder<WithName, WithDimensions, NoFlooring> {
        RoomBuilder {
            name: self.name,
            dimensions: dimensions.into(),
            flooring: self.flooring,
        }
    }
}

impl<SF> RoomBuilder<WithName, WithDimensions, SF> {
    pub fn with_flooring(
        self,
        flooring: Flooring,
    ) -> RoomBuilder<WithName, WithDimensions, WithFlooring> {
        RoomBuilder {
            name: self.name,
            dimensions: self.dimensions.into(),
            flooring: flooring.into(),
        }
    }
}

impl RoomBuilder<WithName, WithDimensions, WithFlooring> {
    pub fn with_checked_dimensions(mut self, dimensions: DimensionSet) -> Self {
        self.dimensions = dimensions.into();

        self
    }

    pub fn build(self) -> Room {
        Room {
            name: self.name.inner_value(),
            dimensions: self.dimensions.inner_value(),
            flooring: self.flooring.inner_value(),
        }
    }
}
