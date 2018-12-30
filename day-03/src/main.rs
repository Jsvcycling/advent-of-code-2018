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
    let fabric: HashMap<(u32, u32), u32> = HashMap::new();

    for entry in entries {
        println!("{:?}", entry);
    }
}

fn part2(data: &Vec<Entry>) {
    // TODO
}

fn main() {
    let data: Vec<Entry> = read_file().into_iter().map(|x| parse_line(x)).collect();

    part1(&data);
    part2(&data);
}

