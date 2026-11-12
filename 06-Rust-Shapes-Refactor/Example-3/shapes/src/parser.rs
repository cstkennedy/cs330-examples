use crate::factory::CreationFactory;

use std::io::BufRead;
use std::str::FromStr;

pub struct Parser<F> {
    factory: std::marker::PhantomData<F>,
}

impl<F> Parser<F>
where
    F: CreationFactory,
{
    /// Create shapes based on names from an input buffer.
    ///
    /// # Arguments
    ///
    ///  * `ins` - input source
    ///
    pub fn read_shapes<B: BufRead>(ins: B) -> Vec<F::Item> {
        ins.lines()
            .flatten()
            .flat_map(|line| {
                let name = line.trim();
                F::create_default(name)
            })
            .collect()
    }

    /// Create shapes based on names *and dimension data* from an input buffer.
    ///
    /// # Arguments
    ///
    ///  * `ins` - input source
    ///
    pub fn read_shapes_with<B>(ins: B) -> Vec<F::Item>
    where
        B: BufRead,
        F::Item: FromStr,
    {
        ins.lines()
            .flatten()
            .filter(|line| line.len() > 0)
            .flat_map(|line| line.parse::<F::Item>())
            .collect()
    }
}
