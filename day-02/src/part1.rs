use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Result};

const INPUT_FILE: &'static str = "./data/input.txt";

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE).unwrap();

    // How many strings have a character that appears 2 and 3 times.
    let mut count2: u32 = 0;
    let mut count3: u32 = 0;

    // For each line in the file...
    for curr_line in BufReader::new(file).lines() {
        let line = curr_line?;
        let chars = line.chars();

        let mut counts: HashMap<char, u32> = HashMap::new();

        // Count the number of times each character appears in the string.
        for c in chars {
            if counts.get(&c) == None {
                counts.insert(c, 1);
            } else {
                if let Some(x) = counts.get_mut(&c) {
                    *x += 1;
                } else {
                    panic!("Something went wrong. :(");
                }
            }
        }

        // Check if a character appears exactly twice.
        for val in counts.values() {
            if *val == 2 {
                count2 += 1;
                break
            }
        }

        // Check if a character appears exactly three times.
        for val in counts.values() {
            if *val == 3 {
                count3 += 1;
                break
            }
        }
    }

    println!("Entries with character pairs: {}", count2);
    println!("Entries with character triples: {}", count3);
    println!("Hash: {}", count2 * count3);
    
    Ok(())
}
