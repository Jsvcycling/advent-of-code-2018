use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::FromIterator;

const INPUT_FILE: &'static str = "./input/input_02.txt";

fn read_file() -> Vec<String> {
    let f = File::open(INPUT_FILE).unwrap();
    BufReader::new(f).lines().collect::<Result<_, _>>().unwrap()
}

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

fn part1(data: &Vec<String>) -> Option<usize> {
    // How many strings have a character that appears 2 and 3 times.
    let mut count2: u32 = 0;
    let mut count3: u32 = 0;

    // For each line in the file...
    for l in data {
        let chars = l.chars();

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

    Some((count2 * count3) as usize)
}

fn part2(data: &Vec<String>) -> Option<String> {
    let mut strings: Vec<Vec<char>> = Vec::new();
    
    for l in data {
        let chars = l.chars();
        strings.push(chars.collect());
    }

    // Iterate over every string and compare it to every other string (in O(n^2) time).
    for i in 0..strings.len()-1 {
        for j in i+1..strings.len()-1 {
            if let (Some(a), Some(b)) = (strings.get(i), strings.get(j)) {
                if let Some(matches) = get_matches(a, b) {
                    // If the strings only differ by one character...
                    if matches.len() == a.len()-1 {
                        // We found it!
                        return Some(String::from_iter(matches));
                    }
                }
            }
        }
    }

    None
}

fn main() {
    let data = read_file();

    println!("Part 1 Solution: {:?}", part1(&data).unwrap());
    println!("Part 2 Solution: {:?}", part2(&data).unwrap());
}
