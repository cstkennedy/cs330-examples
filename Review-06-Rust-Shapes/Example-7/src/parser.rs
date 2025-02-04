use crate::factory::Factory;
use crate::shape::Shape;

use std::io::BufRead;

pub struct Parser;

impl Parser {
    /// Create shapes based on names from an input buffer.
    ///
    /// # Arguments
    ///
    ///  * `ins` - input source
    ///
    pub fn read_shapes<B: BufRead>(ins: B) -> Vec<Box<dyn Shape>> {
        ins.lines()
            .flatten()
            .flat_map(|line| {
                let name = line.trim();
                Factory::create(name)
            })
            .collect()
    }

    /// Create shapes based on names *and dimension data* from an input buffer.
    ///
    /// # Arguments
    ///
    ///  * `ins` - input source
    ///
    pub fn read_shapes_with<B>(ins: B) -> Vec<Box<dyn Shape>>
    where
        B: BufRead,
    {
        ins.lines()
            .flatten()
            .filter(|line| line.len() > 0)
            .map(|line| {
                line.trim()
                    .split(";")
                    .map(String::from)
                    .collect::<Vec<String>>()
            })
            .filter(|split_line| split_line.len() == 2)
            .flat_map(|split_line| {
                let name = &split_line[0];

                let dims = &split_line[1]
                    .split_whitespace()
                    .filter(|s| !s.is_empty())
                    .flat_map(|dim| dim.trim().parse())
                    .collect::<Vec<f64>>();

                Factory::create_with(&name, &dims)
            })
            .collect()
    }
}
