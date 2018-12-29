use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &'static str = "./data/input.txt";

fn main() -> Result<()> {
    // The frequency shift value.
    let mut val: i32 = 0;

    // Open up the file.
    let file = File::open(INPUT_FILE).unwrap();

    // Read each line.
    for curr_line in BufReader::new(file).lines() {
        let line = curr_line?;
        let mut chars = line.chars();

        // Parse and execute the shift.
        match chars.next().unwrap() {
            '+' => val += chars.as_str().parse::<i32>().unwrap(),
            '-' => val -= chars.as_str().parse::<i32>().unwrap(),
            _ => panic!("Unknown character.")
        }
    }

    // Output the final frequency shift and return.
    println!("{}", val);
    Ok(())
}
