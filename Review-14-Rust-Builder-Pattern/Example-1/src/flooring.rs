use crate::error::*;

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
}

impl Default for Flooring {
    fn default() -> Self {
        Flooring::new()
    }
}

pub struct FlooringBuilder<'a> {
    type_name: Option<&'a str>,
    unit_cost: Option<f64>,
}

impl<'a> FlooringBuilder<'a> {
    pub fn new() -> Self {
        FlooringBuilder {
            type_name: None,
            unit_cost: None,
        }
    }

    pub fn with_specific_name(mut self, a_name: &'a str) -> Self {
        self.type_name = Some(a_name);

        self
    }

    pub fn with_unit_cost(mut self, a_cost: f64) -> Self {
        self.unit_cost = Some(a_cost);

        self
    }

    pub fn build(self) -> Result<Flooring, BuildError> {
        Ok(Flooring {
            type_name: self.type_name.unwrap().to_owned(),
            unit_cost: self.unit_cost.unwrap(),
        })
    }
}
