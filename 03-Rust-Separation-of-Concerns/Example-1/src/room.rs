use std::cmp::Ordering;
use std::fmt;
use std::fmt::Display; //,Formatter,Result};
                       // use std::str::FromStr;
                       // use std::num::ParseFloatError;

#[derive(Clone)]
pub struct Flooring {
    pub type_name: String,
    pub unit_cost: f64,
}

impl Flooring {
    fn new() -> Self {
        Flooring {
            type_name: "Generic".to_string(),
            unit_cost: 1.0f64,
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

impl Default for Flooring {
    fn default() -> Self {
        Flooring::new()
    }
}

#[derive(Clone)]
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

#[derive(Clone)]
pub struct Room {
    pub name: String,
    pub dimensions: DimensionSet,
    pub flooring: Flooring,
}

impl Room {
    /// Set the name using the builder pattern.
    ///
    /// # Arguments
    ///  * `nme` - room name
    ///
    pub fn with_name(mut self, nme: &str) -> Self {
        self.name = nme.to_string();

        self
    }

    /// Set the Flooring using the builder pattern.
    ///
    /// # Arguments
    ///  * `nme` - flooring type name
    ///  * `unit_c` - unit cost
    ///
    pub fn with_flooring(mut self, nme: &str, unit_c: f64) -> Self {
        self.flooring.type_name = nme.to_string();
        self.flooring.unit_cost = unit_c;

        self
    }

    /// Set the Flooring using the builder pattern.
    ///
    /// # Arguments
    ///  * `l` - length
    ///  * `w` - width
    ///
    pub fn with_dimensions(mut self, l: f64, w: f64) -> Self {
        self.dimensions.length = l;
        self.dimensions.width = w;

        self
    }

    /// Set the flooring.
    ///
    /// # Arguments
    ///  * `nme` - flooring type name
    ///  * `unit_c` - unit cost
    ///
    pub fn set_flooring(&mut self, nme: &str, unit_c: f64) {
        self.flooring.type_name = nme.to_string();
        self.flooring.unit_cost = unit_c;
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
