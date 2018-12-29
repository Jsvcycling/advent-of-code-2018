use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &'static str = "./data/input.txt";

fn main() -> Result<()> {
    // The frequency shift value.
    let mut val: i32 = 0;

    // A map of all the values we've seen before.
    let mut past_vals: HashMap<i32, bool> = HashMap::new();

    // Has a duplicate been found?
    let mut found = false;

    // Until a duplicate is found...
    while !found {
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

            // Have we seen this value before?
            if past_vals.get(&val) == None {
                // Nope, add it to the hash-map.
                past_vals.insert(val, true);
            } else {
                // Yep! We're done.
                found = true;
                break
            }
        }
    }

    // Output the final frequency shift and return.
    println!("{}", val);
    Ok(())
}
