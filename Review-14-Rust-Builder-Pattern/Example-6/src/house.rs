use std::fmt::{self, Display, Formatter};
use std::vec::Vec;

use crate::error::{HouseError, HouseErrorWithState};
use crate::room::Room;

type Collection = Vec<Room>;

#[derive(Clone, Debug, Default)]
pub struct House {
    name: String,
    rooms: Collection,
}

impl House {
    pub fn builder() -> HouseBuilder<NoRooms> {
        HouseBuilder::new()
    }

    /// Get the name using a traditional accessor.
    ///
    pub fn get_name(&self) -> &str {
        &self.name
    }

    /// Return the number of rooms in this House.
    ///
    pub fn len(&self) -> usize {
        self.rooms.len()
    }

    /// Determine whether this House is empty (i.e. `self.len() == 0).
    ///
    pub fn is_empty(&self) -> bool {
        self.rooms.is_empty()
    }

    /// Wrapper around `Collection<Room>.iter()`.
    ///
    pub fn iter(&self) -> impl Iterator<Item = &Room> {
        self.rooms.iter()
    }

    /// Wrapper around `Collection<Room>.iter_mut()`.
    ///
    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut Room> {
        self.rooms.iter_mut()
    }

    pub fn flooring_metrics(&self) -> (f64, f64) {
        let total = self.rooms.iter().map(|room| room.flooring_cost()).sum();

        let avg = total / (self.len() as f64);

        (total, avg)
    }
}

impl Display for House {
    /// This is the equivalent of overloading:
    ///   - `operator<<` in C++
    ///   - `toString` in Java
    ///   - `__str__` in Python
    ///
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        writeln!(f, "--------{}--------\n", self.name)?;

        for room in self.rooms.iter() {
            writeln!(f, "{}", room)?;
        }

        let (total, avg) = self.flooring_metrics();

        writeln!(f, "------------------------------")?;
        writeln!(f, "Total Cost   : $ {:.2}", total)?;
        writeln!(f, "Avg Room Cost: $ {:.2}", avg)
    }
}

impl PartialEq for House {
    /// This is the equivalent of overloading:
    ///   - `operator==` in C++
    ///   - `equals` in Java
    ///   - `__eq__` in Python
    fn eq(&self, rhs: &Self) -> bool {
        if self.name != rhs.name {
            return false;
        }

        if self.len() != rhs.len() {
            return false;
        }

        // We are guaranteed to have the same number of rooms
        for (lhs_room, rhs_room) in self.iter().zip(rhs.iter()) {
            if lhs_room != rhs_room {
                return false;
            }
        }

        return true;
    }
}

//------------------------------------------------------------------------------
#[derive(Default, Debug, PartialEq)]
pub struct NoRooms;

#[derive(Debug, PartialEq)]
pub struct HouseBuilder<SR> {
    name: String,
    rooms: SR,
}

/// The HouseBuilder's state is defined by one generics
///   - SR - (State Rooms) whether at least one room has been added
///
impl HouseBuilder<NoRooms> {
    pub fn new() -> Self {
        HouseBuilder {
            name: "House".to_owned(),
            rooms: Default::default(),
        }
    }
}

impl<SR> HouseBuilder<SR> {
    pub fn with_name(mut self, nme: &str) -> Self {
        self.name = nme.to_owned();

        self
    }
}

impl HouseBuilder<NoRooms> {
    pub fn with_room(self, first_room: Room) -> HouseBuilder<Vec<Room>> {
        HouseBuilder {
            name: self.name,
            rooms: vec![first_room],
        }
    }

    pub fn with_rooms(
        self,
        first_rooms: Vec<Room>,
    ) -> Result<HouseBuilder<Vec<Room>>, HouseErrorWithState<Self>> {
        match first_rooms.len() {
            0 => Err(HouseErrorWithState {
                the_error: HouseError::ZeroRooms,
                the_builder: self,
            }),
            _ => Ok(HouseBuilder {
                name: self.name,
                rooms: first_rooms,
            }),
        }
    }
}

impl HouseBuilder<Vec<Room>> {
    pub fn with_room(mut self, another_room: Room) -> Self {
        self.rooms.push(another_room);

        self
    }

    pub fn with_rooms(mut self, mut more_rooms: Vec<Room>) -> Self {
        self.rooms.append(&mut more_rooms);

        self
    }

    pub fn build(self) -> House {
        House {
            name: self.name.to_owned(),
            rooms: self.rooms,
        }
    }
}
