use crate::builder_utils::WrappedType;
use crate::error::CostError;

#[derive(Clone, Debug, PartialEq)]
pub struct Flooring {
    pub type_name: String,
    pub unit_cost: f64,
}

impl Flooring {
    pub const MIN_COST: f64 = 0_f64;

    pub fn builder() -> FlooringBuilder {
        FlooringBuilder::new()
    }
}

impl Default for Flooring {
    fn default() -> Self {
        Flooring::builder().build()
    }
}

#[derive(Default, Debug, Clone, Copy)]
pub struct Cost {
    wrapped: f64,
}

impl WrappedType<f64> for Cost {
    fn inner_value(self) -> f64 {
        self.wrapped
    }
}

impl std::ops::Deref for Cost {
    type Target = f64;

    fn deref(&self) -> &Self::Target {
        &self.wrapped
    }
}

impl TryFrom<f64> for Cost {
    type Error = CostError;

    fn try_from(value: f64) -> Result<Self, Self::Error> {
        if value < Flooring::MIN_COST {
            return Err(CostError::InvalidCost(Flooring::MIN_COST).into());
        }

        Ok(Cost { wrapped: value })
    }
}

pub struct FlooringBuilder {
    type_name: String,
    unit_cost: f64,
}

impl FlooringBuilder {
    pub fn new() -> Self {
        Self {
            type_name: "Generic".to_owned(),
            unit_cost: 1.0,
        }
    }

    pub fn with_name<S>(mut self, name: S) -> Self
    where
        S: AsRef<str>,
    {
        self.type_name = name.as_ref().to_owned();

        self
    }

    pub fn with_unit_cost(mut self, cost: Cost) -> Self {
        self.unit_cost = *cost;

        self
    }

    pub fn build(self) -> Flooring {
        Flooring {
            type_name: self.type_name,
            unit_cost: self.unit_cost,
        }
    }
}
