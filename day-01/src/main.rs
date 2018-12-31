use std::collections::HashMap;

use std::fs::File;
use std::io::{BufRead, BufReader};

const INPUT_FILE: &'static str = "./data/input.txt";

fn read_file() -> Vec<String> {
    let f = File::open(INPUT_FILE).unwrap();
    BufReader::new(f).lines().collect::<Result<_, _>>().unwrap()
}

fn part1(data: &Vec<String>) -> Option<usize> {
    // The frequency shift value.
    let mut val: i32 = 0;

    for l in data {
        let mut chars = l.chars();

        // Parse and execute the shift.
        match chars.next().unwrap() {
            '+' => val += chars.as_str().parse::<i32>().unwrap(),
            '-' => val -= chars.as_str().parse::<i32>().unwrap(),
            _ => panic!("Unknown character."),
        }
    }

    Some(val as usize)
}

fn part2(data: &Vec<String>) -> Option<usize> {
        // The frequency shift value.
    let mut val: i32 = 0;

    // A map of all the values we've seen before.
    let mut past_vals: HashMap<i32, bool> = HashMap::new();

    // Has a duplicate been found?
    let mut found = false;

    // Until a duplicate is found...
    while !found {
        for l in data {
            let mut chars = l.chars();

            // Parse and execute the shift.
            match chars.next().unwrap() {
                '+' => val += chars.as_str().parse::<i32>().unwrap(),
                '-' => val -= chars.as_str().parse::<i32>().unwrap(),
                _ => panic!("Unknown character.")
            }

            let val = past_vals.entry(val).or_insert(false);
            
            if *val {
                found = true;
                break
            } else {
                *val = true;
            }
        }
    }

    Some(val as usize)
}

fn main() {
    let data = read_file();

    println!("Part 1 Solution: {:?}", part1(&data).unwrap());
    println!("Part 2 Solution: {:?}", part2(&data).unwrap());
}
