use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

use eyre::OptionExt;


fn main() -> eyre::Result<()> {
    let word_fname = std::env::args()
        .nth(1)
        .ok_or_eyre("Word filename was not provided")?;

    let word_file = {
        let file = File::open(word_fname)?;
        BufReader::new(file)
    };

    let words: Vec<String> = word_file
        .lines()
        .flatten()
        .filter(|line| !line.is_empty())
        .map(|line| line.trim().to_owned())
        .filter(|word| word.len() < 5)
        .filter(|word| !['.', '+'].iter().any(|symbol| word.contains(*symbol)))
        .collect();

    for word in words.iter() {
        println!("|{word}|");
    }

    Ok(())
}
