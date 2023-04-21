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

    fn with_specific_name(mut self, a_name: &str) -> Self {
        self.type_name = a_name.to_string();

        self
    }

    fn with_unit_cost(mut self, a_cost: f64) -> Self {
        self.unit_cost = a_cost;

        self
    }
}

impl Default for Flooring {
    fn default() -> Self {
        Flooring::new()
    }
}
