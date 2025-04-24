
fn main() {
    let mut names = vec!("Thomas", "Jay", "Steve", "Janet", "Ravi");

    println!("---------While Loop Example----------");

    let mut it = names.iter();
    while let Some(val) = it.next() {
        println!("  - {}", val);
    }

    println!("--------For-each Loop Example--------");
    for val in names.iter() {
        println!("  - {}", val);
    }

    //--------------------------------------------------------------------------
    // Add a name - push_back demo
    //--------------------------------------------------------------------------
    println!("-------------Add a Name--------------");
    let new_name = "Hill";
    names.push(new_name);

    for val in names.iter() {
        println!("  - {}", val);
    }

    //--------------------------------------------------------------------------
    // Search for a name
    //--------------------------------------------------------------------------
    println!("----------Search for a Name----------");
    let thing_to_find = "Thomas";

    // Original non-function search
    let mut it = names.iter();
    while let Some(val) = it.next() {
        if val == &thing_to_find {
            println!("Found a Match");
        }
    }

    match names.iter().find(|&name| name == &"Steve") {
        Some(_) => println!("Found a Match"),
        None => println!("No match was found")
    }
}
