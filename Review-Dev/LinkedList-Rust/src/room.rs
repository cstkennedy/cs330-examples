use std::fmt;
use std::fmt::{Display,Formatter,Result};
use std::cmp::Ordering;
use std::str::FromStr;
use std::num::ParseFloatError;

#[derive(Clone)]
pub struct Flooring {
    pub type_name: String,
    pub unit_cost: f64,
}

impl Flooring {
    fn new() -> Self {
        Flooring{
            type_name: "Generic".to_string(),
            unit_cost: 1.0f64
        }
    }

    fn with_specific_name(&mut self, a_name: &str) -> &mut Self {
        self.type_name = a_name.to_string();

        self
    }

    fn with_unit_cost(&mut self, a_cost: f64) -> &mut Self {
        self.unit_cost = a_cost;

        self
    }
}

#[derive(Clone)]
pub struct DimensionSet {
    pub length: f64,
    pub width: f64,
}

impl DimensionSet {
    fn new(l: f64, w: f64) -> Self {
        DimensionSet {
            length: l,
            width: w
        }
    }
}

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub dimensions: DimensionSet,
    pub flooring: Flooring
}

impl Room {
    pub fn set_flooring(&mut self, nme: &str, unit_c: f64) {
        self.flooring.type_name = nme.to_string();
        self.flooring.unit_cost = unit_c;
    }

    pub fn area(&self) -> f64 {
        self.dimensions.width * self.dimensions.length
    }

    pub fn flooring_cost(&self) -> f64 {
        self.area() * self.flooring.unit_cost
    }
}

/*
Room::Room()
    :name("Generic")
{
}

//------------------------------------------------------------------------------
Room::Room(Dimension l, Dimension w, Cost c)
    :Room("Generic", l, w, c)
{
}


//------------------------------------------------------------------------------
Room::Room(std::string n, Dimension l, Dimension w, Cost c)
    :name(n),
     flooring("Generic", c),
     dimensions(l, w)
{
    // Wreorder as a topic for discussion
}

//------------------------------------------------------------------------------
Room::Room(std::string n, DimensionSet d, Cost c, std::string fn)
    :dimensions(d),
     flooring(fn, c),
     name(n)
{
}

//------------------------------------------------------------------------------
void Room::setDimensions(Dimension l, Dimension w)
{
    dimensions.setLength(l);
    dimensions.setWidth(w);
}

*/

impl Display for Room {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        writeln!(f, "Room ({})", self.name);
        writeln!(f, "  {:<8}: {:>8.1}", "Length", self.dimensions.length);
        writeln!(f, "  {:<8}: {:>8.1}", "Width", self.dimensions.width);
        writeln!(f, "  {:<8}: {:>8.1}", "Area", self.area());
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
        }
        else {
            self.name.partial_cmp(&rhs.name)
        }
    }
}

impl PartialEq for Room {
    fn eq(&self, rhs: &Self) -> bool {
        return self.name.eq(&rhs.name)
            && self.area().eq(&rhs.area());
    }
}

/*
impl FromStr for Room {
    type Err = ParseFloatError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let tokens: Vec<String> = s.to_string().split().iter().trim().collect();

        let room = Room {
            name: tokens[0].trim(),
            dimensions: DimensionSet {
                length: tokens[1].parse::<f64>().unwrap(),
                width: tokens[2].parse::<f64>().unwrap()
            },
            flooring: Flooring {
                type_name: tokens[3],
                unit_cost: tokens[4].parse::<f64>().unwrap(),
            }
        };

        Ok(room)
    }
}
*/
