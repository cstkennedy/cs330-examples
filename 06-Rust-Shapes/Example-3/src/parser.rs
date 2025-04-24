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
        /*
        let mut shapes: Vec<Box<dyn Shape>> = Vec::new();

        for line in ins.lines() {
            // let next_shape = Factory::create(n);
            // match next_shape {
                // Some(s) => shapes.push(s),
                // None => {},
            // }
            let n = line.unwrap();
            let n = n.trim();
            if let Some(s) = Factory::create(n) {
                shapes.push(s)
            }
        }
        */

        ins.lines()
            .flatten()
            .map(|line| {
                let name = line.trim();
                Factory::create(name)
            })
            .flatten()
            .collect()
    }
}
