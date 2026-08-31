use clap::Parser;
use rand::prelude::*;

use linked_list::prelude::*;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Args {
    /// Seed to the RNG
    seed: Option<u64>,

    /// Number of integers to generate and store in the linked list
    #[arg(default_value_t = 10)]
    num_ints: usize,
}

fn generate_list<R>(rng: &mut R, num_ints: usize) -> LinkedList
where
    R: Rng,
{
    let mut ll = LinkedList::default();

    for _ in 0..num_ints {
        let rand_int: i64 = rng.random_range(-10..=10);
        ll.push(rand_int)
    }

    ll
}

fn main() {
    let args = Args::parse();

    let mut rng = match args.seed {
        Some(seed) => SmallRng::seed_from_u64(seed),
        None => rand::make_rng(),
    };

    let ll = generate_list(&mut rng, args.num_ints);

    println!("{:#?}", ll);
    println!();
    println!("{:}", ll);
}
