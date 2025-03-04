use typed_builder::TypedBuilder;

#[derive(Clone, Debug, PartialEq, TypedBuilder)]
pub struct Flooring {
    #[builder(default="Generic".into())]
    pub type_name: String,

    #[builder(default = 1.0_f64)]
    pub unit_cost: f64,
}

impl Flooring {
    pub fn new() -> Self {
        Flooring::builder().build()
    }
}

impl Default for Flooring {
    fn default() -> Self {
        Flooring::builder().build()
    }
}
