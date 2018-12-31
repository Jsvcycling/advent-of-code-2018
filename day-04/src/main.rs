extern crate chrono;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::IntoIterator;

use chrono::{Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

const INPUT_FILE: &'static str = "./data/input.txt";

#[derive(Debug)]
struct State {
    pub timestamp: NaiveDateTime,
    pub id: i32,
    pub start: bool,
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

    let (id, start, awake) = match parts[0] {
        "Guard" => (parts[1].trim_matches('#').parse::<i32>().unwrap(), true, true),
        "wakes" => (-1, false, true),
        "falls" => (-1, false, false),
        _ => panic!("Unknown action"),
    };

    State { timestamp: NaiveDateTime::new(date, time), id: id, start: start, awake: awake }
}

// This works but still feels pretty ugly...
fn part1(data: &Vec<&State>) -> Option<usize> {
    let guard_id = {
        let mut sleep_time: HashMap<i32, i64> = HashMap::new();

        let mut i = 0;
        
        while i < data.len()-1 {
            if let (Some(x), Some(y)) = (data.get(i), data.get(i+1)) {
                if !x.awake && y.awake && x.id == y.id {
                    let time_diff = (y.timestamp - x.timestamp).num_minutes();
                    let val = sleep_time.entry(x.id).or_insert(0);
                    *val += time_diff;
                } else {
                    panic!("Bad data: {:?} {:?}", x, y);
                }
            }

            i += 2;
        }

        let max = sleep_time.iter().max_by_key(|x| x.1).unwrap();
        max.0.clone() as u32
    };

    let guard_data: Vec<&&State> = data.iter().filter(|x| x.id == guard_id as i32).collect();

    let pop_time = {
        let mut sleepy_guard_data: HashMap<NaiveTime, i32> = HashMap::new();

        let mut i = 0;

        while i < guard_data.len()-1 {
            if let (Some(x), Some(y)) = (guard_data.get(i), guard_data.get(i+1)) {
                if !x.awake && y.awake {
                    let mut curr_time = x.timestamp.time();

                    while curr_time < y.timestamp.time() {
                        let val = sleepy_guard_data.entry(curr_time).or_insert(0);
                        *val += 1;

                        curr_time = curr_time + Duration::minutes(1);
                    }
                } else {
                    panic!("Bad data: {:?} {:?}", x, y);
                }
            }

            i += 2;
        }

        let max = sleepy_guard_data.iter().max_by_key(|x| x.1).unwrap();
        max.0.minute()
    };
    
    Some((guard_id * pop_time) as usize)
}

// This is pretty ugly too but it works pretty well...
fn part2(data: &Vec<&State>) -> Option<usize> {
    let mut sleep_data: HashMap<NaiveTime, HashMap<i32, i32>> = HashMap::new();

    let mut i = 0;

    // For each time, count how many times a guard is asleep at that time.
    while i < data.len()-1 {
        if let (Some(x), Some(y)) = (data.get(i), data.get(i+1)) {
            if !x.awake && y.awake && x.id == y.id {
                let mut curr_time = x.timestamp.time();

                while curr_time < y.timestamp.time() {
                    if let Some(a) = sleep_data.get_mut(&curr_time) {
                        if let Some(b) = a.get_mut(&x.id) {
                            *b += 1;
                        } else {
                            a.insert(x.id, 1);
                        }
                    } else {
                        let mut b: HashMap<i32, i32> = HashMap::new();
                        b.insert(x.id, 1);
                        sleep_data.insert(curr_time, b);
                    }

                    curr_time = curr_time + Duration::minutes(1);
                }
            } else {
                panic!("Bad data: {:?} {:?}", x, y);
            }
        }

        i += 2;
    }

    let mut maxes: HashMap<NaiveTime, (i32, i32)> = HashMap::new();

    // Find the max at each time.
    for (time, vals) in sleep_data.iter() {
        let max = vals.iter().max_by_key(|x| x.1).unwrap();
        maxes.insert(*time, (*max.0, *max.1));
    }

    // Find the max of the maxes.
    let max = maxes.iter().max_by_key(|x| (x.1).1).unwrap();
    let guard_id = (max.1).0 as u32;
    let time = max.0.minute();

    Some((guard_id * time) as usize)
}

fn main() {
    // Parse each line of the input file.
    let mut states: Vec<State> = read_file()
        .into_iter()
        .map(|x| parse_line(x))
        .collect();

    // Sort by timestamp.
    states.sort_unstable_by(|a, b| a.timestamp.cmp(&b.timestamp));

    // Copy ID's down the list until a new ID is encountered.
    {
        let mut last_id = -1;
        
        for item in states.iter_mut() {
            if item.id != -1 {
                last_id = item.id;
                continue;
            }

            item.id = last_id
        }
    }

    // We only care about wake and sleep information.
    let data: Vec<&State> = states.iter().filter(|x| !x.start).collect();

    // Compute the solutions.
    println!("Part 1 Solution: {:?}", part1(&data).unwrap());
    println!("Part 2 Solution: {:?}", part2(&data).unwrap());
}
