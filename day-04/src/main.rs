extern crate chrono;

// use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::IntoIterator;

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};

const INPUT_FILE: &'static str = "./data/input.txt";

#[derive(Debug)]
struct State {
    pub timestamp: NaiveDateTime,
    pub id: i32,
    pub awake: bool,
}

fn read_file() -> Vec<String> {
    let f = File::open(INPUT_FILE).unwrap();
    BufReader::new(f).lines().collect::<Result<_, _>>().unwrap()
}

fn parse_line(line: String) -> State {
    let mut parts: Vec<&str> = line.split_whitespace().into_iter().collect();

    let date = NaiveDate::parse_from_str(parts.remove(0).trim_matches('['), "%Y-%m-%d").unwrap();
    let time = NaiveTime::parse_from_str(parts.remove(0).trim_matches(']'), "%H:%M").unwrap();

    let (id, awake) = match parts[0] {
        "Guard" => (parts[1].trim_matches('#').parse::<i32>().unwrap(), true),
        "wakes" => (-1, true),
        "falls" => (-1, false),
        _ => panic!("Unknown action"),
    };

    State { timestamp: NaiveDateTime::new(date, time), id: id, awake: awake } }

fn part1(data: &Vec<State>) -> Option<usize> {
    None
}

fn part2(data: &Vec<State>) -> Option<usize> {
    None
}

fn main() {
    // Parse each line of the input file.
    let mut data: Vec<State> = read_file()
        .into_iter()
        .map(|x| parse_line(x))
        .collect();

    data.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));

    {
        let mut last_id = -1;
        
        for item in data.iter_mut() {
            if item.id != -1 {
                last_id = item.id;
                continue;
            }

            item.id = last_id
        }
    }

    // for item in data.iter() {
    //     println!("{:?}", item);
    // }

    println!("Part 1 Solution: {:?}", part1(&data));
    println!("Part 2 Solution: {:?}", part2(&data));
}
