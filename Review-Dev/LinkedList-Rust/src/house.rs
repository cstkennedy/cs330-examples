use crate::room::*;
use std::vec::*;
use std::fmt;
use std::fmt::{Display,Formatter,Result};
use std::cmp::Ordering;

type Collection = std::vec::Vec<Room>;

#[derive(Clone)]
pub struct House {
    name: String,
    pub rooms: Collection,
}

impl House {
    pub fn new() -> Self {
        House {
            name: "House".to_string(),
            rooms: Collection::new()
        }
    }

    pub fn with_name(&mut self, nme: &str) -> &mut Self {
        self.name = nme.to_string();

        self
    }

    pub fn set_name(&mut self, nme: &str) {
        self.name = nme.to_string();
    }

    pub fn add_room(&mut self, to_add: Room) {
        self.rooms.push(to_add);
    }

    pub fn len(&self) -> usize {
        self.rooms.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rooms.is_empty()
    }
}


impl Display for House {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "--------{}--------", self.name);

        let mut total = 0f64;

        for room in self.rooms.iter() {
            writeln!(f, "{}", room);

            total += room.flooring_cost();
        }

        let avg = total / (self.len() as f64);

        writeln!(f);
        writeln!(f, "------------------------------");
        writeln!(f);

        writeln!(f, "Total Cost   : $ {:.2}", total);
        writeln!(f, "Avg Room Cost: $ {:.2}", avg)
    }
}

