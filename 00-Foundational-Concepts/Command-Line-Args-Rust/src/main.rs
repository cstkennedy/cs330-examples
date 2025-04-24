use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    for (idx, arg) in args.iter().enumerate() {
        println!("{:>2}: {}", idx, arg);
    }
}
