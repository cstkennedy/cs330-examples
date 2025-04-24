use crate::room::*;
use std::fmt;
use std::fmt::Display;
use std::vec::Vec; // ,Formatter,Result};
                   // use std::cmp::Ordering;

type Collection = Vec<Room>;

#[derive(Clone)]
pub struct House {
    name: String,
    rooms: Collection,
}

impl House {
    /// This is the Default constructor. Start with the name set to "House" and
    /// an empty set of rooms (i.e., zero rooms).
    pub fn new() -> Self {
        House {
            name: "House".to_string(),
            rooms: Collection::new(),
        }
    }

    /// Demonstrate the builder pattern. Take a mutable reference to a House,
    /// change its name, and return a reference.
    ///
    /// # Arguments
    ///
    ///  * `nme` - new House name
    ///
    pub fn with_name(mut self, nme: &str) -> Self {
        self.name = nme.to_string();
        self
    }

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
    pub fn iter(&self) -> std::slice::Iter<Room> {
        self.rooms.iter()
    }

    /// Wrapper around `Collection<Room>.iter_mut()`.
    ///
    pub fn iter_mut(&mut self) -> std::slice::IterMut<Room> {
        self.rooms.iter_mut()
    }
}

impl Default for House {
    /// Construct a house with the name "Generic" and zero rooms.
    ///
    fn default() -> Self {
        House::new()
    }
}

impl Display for House {
    /// This is the equivalent of overloading:
    ///   - `operator<<` in C++
    ///   - `toString` in Java
    ///   - `__str__` in Python
    ///
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--------{}--------\n", self.name);

        let mut total = 0f64;

        for room in self.rooms.iter() {
            let _ = writeln!(f, "{}", room);

            total += room.flooring_cost();
        }

        let avg = total / (self.len() as f64);

        #[allow(unused_must_use)]
        {
            writeln!(f, "------------------------------");
            writeln!(f, "Total Cost   : $ {:.2}", total);
            writeln!(f, "Avg Room Cost: $ {:.2}", avg)
        }
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
        let mut lhs_it = self.iter();
        let mut rhs_it = rhs.iter();

        let mut lhs_room = lhs_it.next();
        let mut rhs_room = rhs_it.next();

        while lhs_room != None && rhs_room != None {
            match (lhs_room, rhs_room) {
                (Some(lhs), Some(rhs)) => {
                    if lhs != rhs {
                        return false;
                    }

                    lhs_room = lhs_it.next();
                    rhs_room = rhs_it.next();
                }
                (Some(_lhs), None) => {
                    return false;
                }
                (None, Some(_rhs)) => {
                    return false;
                }
                _ => {}
            }
        }

        /*
        if lhs_room == None && rhs_room == None {
            return true;
        }

        false
        */

        return lhs_room == None && rhs_room == None;
    }
}
