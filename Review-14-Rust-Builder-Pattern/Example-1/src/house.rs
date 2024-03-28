use crate::error::*;
use crate::room::*;
use std::fmt;
use std::fmt::Display;
use std::vec::Vec; // ,Formatter,Result};
                   // use std::cmp::Ordering;

type Collection = Vec<Room>;

#[derive(Clone, Default)]
pub struct House {
    name: String,
    rooms: Collection,
}

impl House {
    /// Set the name using a traditional (i.e., non-builder) mutator.
    ///
    /// # Arguments
    ///
    ///  * `nme` - new House name
    ///
    pub fn set_name(&mut self, nme: &str) {
        self.name = nme.to_string();
    }

    /// Get the name using a traditional accessor.
    ///
    pub fn get_name(&self) -> &String {
        &self.name
    }

    /// Add another room to this House.
    ///
    /// # Arguments
    ///
    ///  * `to_add` - new Room to add
    pub fn add_room(&mut self, to_add: Room) {
        self.rooms.push(to_add);
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--------{}--------\n", self.name)?;

        for room in self.rooms.iter() {
            let _ = writeln!(f, "{}", room)?;
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

pub struct HouseBuilder<'a> {
    name: Option<&'a str>,
    rooms: Vec<Room>,
}

impl<'a> HouseBuilder<'a> {
    pub fn new() -> Self {
        HouseBuilder {
            name: None,
            rooms: Default::default(),
        }
    }

    pub fn with_name(mut self, nme: &'a str) -> Self {
        self.name = Some(nme);

        self
    }

    pub fn with_room(mut self, another_room: Room) -> Self {
        self.rooms.push(another_room);

        self
    }

    pub fn with_rooms(mut self, mut more_rooms: Vec<Room>) -> Self {
        self.rooms.append(&mut more_rooms);

        self
    }

    pub fn build(self) -> Result<House, BuildError> {
        if self.rooms.len() < 1 {
            return Err(BuildError::GenericError(
                "A House must have at least one room.",
            ));
        }

        // let name = match self.name {
        // Some(name) => name,
        // None => "House",
        // };
        let name = self.name.unwrap_or("House");

        let house = House {
            name: name.to_owned(),
            rooms: self.rooms,
        };

        Ok(house)
    }
}
