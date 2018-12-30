use std::fs::File;
use std::io::{BufRead, BufReader, Result};
use std::iter::FromIterator;

const INPUT_FILE: &'static str = "./data/input.txt";

fn get_matches(x: &Vec<char>, y: &Vec<char>) -> Option<Vec<char>> {
    if x.len() != y.len() {
        return None;
    }
    
    let mut out: Vec<char> = Vec::new();

    // At position i, are the characters identical?
    for i in 0..x.len() {
        if let (Some(a), Some(b)) = (x.get(i), y.get(i)) {
            if a == b {
                out.push(*a)
            }
        }
    }

    Some(out)
}

fn main() -> Result<()> {
    let file = File::open(INPUT_FILE)?;

    let mut strings: Vec<Vec<char>> = Vec::new();

    // Create a vector of all the strings.
    for curr_line in BufReader::new(file).lines() {
        let line = curr_line?.clone();
        let chars = line.chars();

        strings.push(chars.collect());
    }

    // Iterate over every string and compare it to every other string (careful, this is O(n) time).
    'outer: for i in 0..strings.len()-1 {
        for j in i+1..strings.len()-1 {
            if let (Some(a), Some(b)) = (strings.get(i), strings.get(j)) {
                if let Some(matches) = get_matches(a, b) {
                    // If the strings only differ by one character...
                    if matches.len() == a.len()-1 {
                        // We found it!
                        println!("String A: {}", String::from_iter(a));
                        println!("String B: {}", String::from_iter(b));
                        println!("Matching Characters: {}", String::from_iter(matches));
                        break 'outer;
                    }
                }
            }
        }
    }

    Ok(())
}
