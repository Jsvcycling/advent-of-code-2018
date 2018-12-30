use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::IntoIterator;

const INPUT_FILE: &'static str = "./data/input.txt";

#[derive(Debug)]
struct Entry(u32, (u32, u32), (u32, u32));

fn read_file() -> Vec<String> {
    let f = File::open(INPUT_FILE).unwrap();
    BufReader::new(f).lines().collect::<Result<_, _>>().unwrap()
}

// Parse an individual line into an entry.
fn parse_line(line: String) -> Entry {
    let parts: Vec<&str> = line.split_whitespace().into_iter().collect();
    
    let id: u32 = parts[0].trim_matches('#').parse::<u32>().unwrap();
    let start: Vec<&str> = parts[2].trim_matches(':').split(',').collect();
    let size: Vec<&str> = parts[3].split('x').collect();

    Entry(
        id,
        ( start[0].parse::<u32>().unwrap(), start[1].parse::<u32>().unwrap() ),
        ( size[0].parse::<u32>().unwrap(), size[1].parse::<u32>().unwrap() ),
    )
}

fn part1(data: &Vec<Entry>) {
    let mut fabric: HashMap<(u32, u32), u32> = HashMap::new();

    for entry in data {
        let start_x = (entry.1).0;
        let start_y = (entry.1).1;
        let size_x = (entry.2).0;
        let size_y = (entry.2).1;

        // Populate the hash-map with the number of times a square is covered.
        for i in start_x..start_x+size_x {
            for j in start_y..start_y+size_y {
                if let Some(x) = fabric.get_mut(&(i, j)) {
                    *x += 1;
                } else {
                    fabric.insert((i, j), 1);
                }
            }
        }
    }

    // Count the number of squares that is claimed by at least 2 entries.
    let count = fabric.values().filter(|x| **x > 1).count();
    
    println!("{}", count);
}

fn part2(data: &Vec<Entry>) {
    // TODO
}

fn main() {
    let data: Vec<Entry> = read_file().into_iter().map(|x| parse_line(x)).collect();

    part1(&data);
    part2(&data);
}

