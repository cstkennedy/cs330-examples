use std::io::{stdin, stdout};
use std::io::Write;

// TODO: Add proper error handling
fn main() -> Result<(), Box<dyn std::error::Error>> {
    print!("Enter an Integer: ");
    let _ = stdout().flush();
    let trimmed_input = read_line_as_string();
    let input_int: i64 = trimmed_input.parse()?;

    print!("Enter a Double: ");
    let _ = stdout().flush();
    let trimmed_input = read_line_as_string();
    let input_double = trimmed_input.parse::<f64>()?;

    print!("Enter a Character: ");
    let _ = stdout().flush();
    let trimmed_input = read_line_as_string();
    let input_char = trimmed_input.chars().nth(0).unwrap_or('\0');

    print!("Enter a Boolean: ");
    let _ = stdout().flush();
    let trimmed_input = read_line_as_string().to_lowercase().to_string();
    let input_boolean: bool = trimmed_input.parse()?;

    print!("Enter a String (with spaces): ");
    let _ = stdout().flush();
    let input_string = read_line_as_string();

    println!(); // cout << "%n";
    println!("You Entered:"); // cout << "You Entered:" << "%n";
    println!("  Item {:2}: {}", 1, input_int);
    println!("  Item {:2}: {:4.2}", 2, input_double);
    println!("  Item {:2}: {}", 3, input_char);
    println!("  Item {:2}: {}", 4, input_boolean);
    println!("  Item {:2}: {}", 6, input_string);

    Ok(())
}

/// Read a line from std in and trim the trailing newline.
fn read_line_as_string() -> String {
    let mut str_buffer = String::new();
    let _ = stdin().read_line(&mut str_buffer).unwrap();
    let str_buffer = str_buffer.trim().to_string();

    str_buffer
}
