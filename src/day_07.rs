#![feature(vec_remove_item)]

#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::IntoIterator;

const INPUT_FILE: &'static str = "./input/input_07.txt";
const ALPHABET: &'static str = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";

type Entry = (char, char);

fn read_file() -> Vec<String> {
    let f = File::open(INPUT_FILE).unwrap();
    BufReader::new(f).lines().collect::<Result<_, _>>().unwrap()
}

fn parse_line(line: &String) -> Entry {
    let parts: Vec<&str> = line.split_whitespace().into_iter().collect();

    (parts[1].parse::<char>().unwrap(), parts[7].parse::<char>().unwrap())
}

fn part1(data: &mut HashMap<char, Vec<char>>) -> Option<String> {
    let mut output: Vec<char> = Vec::new();

    loop {
        // Select the nodes that have no parent(s).
        let mut choices: Vec<char> = data
            .iter()
            .filter(|(_, v)| v.len() == 0)
            .map(|(k, _)| k.clone())
            .collect();

        // If the working list is empty, we're done.
        if choices.len() == 0 {
            break
        }

        // Sort the parent-less nodes into lexographical order.
        choices.sort();

        // Append the first node to the output and remove it as a parent.
        let node = choices[0];

        // Remove the selected node as a dependency for all other nodes.
        data.iter_mut().for_each(|(_, v)| { v.remove_item(&node); });

        // Remove the node from contention and append it to the output string.
        data.remove(&node);
        output.push(node);
    }

    assert_eq!(data.len(), 0);

    Some(output.into_iter().collect::<String>())
}

fn part2(data: &mut HashMap<char, Vec<char>>, count: usize, base_time: usize) -> Option<usize> {
    let mut workers: HashMap<usize, (char, usize)> = {
        let mut m = HashMap::with_capacity(count);
        (0..count).for_each(|x| { m.insert(x, ('0', 0)); });
        m
    };

    let mut curr_second: usize = 0;

    loop {
        for (_, worker) in workers.iter_mut() {
            // This worker is still working, move on.
            if worker.1 > 0 {
                continue;
            }

            // The worker is done, the node is no longer a pending dependency for others.
            data.iter_mut().for_each(|(_, v)| { v.remove_item(&worker.0); });

            // Select the nodes that have no parent(s).
            let mut choices: Vec<char> = data
                .iter()
                .filter(|(_, v)| v.len() == 0)
                .map(|(k, _)| k.clone())
                .collect();

            // There are no more available options for the worker to work on at the moment.
            if choices.len() == 0 {
                worker.0 = '0';
                worker.1 = 0;
                continue;
            }

            // Sort the parent-less nodes into lexographical order.
            choices.sort();

            // Start the node.
            let node = choices[0];
            data.remove(&node);
            worker.0 = node;
            worker.1 = base_time + ALPHABET.chars().position(|x| x == node).unwrap() + 1;
        }

        // Are we all done?
        let done = workers
            .iter()
            .filter(|(_, (n, _))| n != &'0')
            .count() == 0;

        // We're done. Quit.
        if done {
            break;
        }

        // Decrement the time left value for each worker.
        workers.iter_mut()
            .filter(|(_, (n, _))| n != &'0')
            .for_each(|(_, (_, t))| *t -= 1);

        // Increment the current second.
        curr_second += 1;
    }

    assert_eq!(data.len(), 0);

    Some(curr_second)
}

fn main() {
    let input: Vec<Entry> = read_file().iter().map(|x| parse_line(x)).collect();
    let mut data: HashMap<char, Vec<char>> = HashMap::new();

    for entry in input {
        let parent_id = entry.0;
        let child_id = entry.1;

        // Add the parent entry (if not exists).
        data.entry(parent_id).or_insert(vec![]);

        // Add the child entry (if not exists). Append the parent to the list.
        let child = data.entry(child_id).or_insert(vec![]);
        child.push(parent_id);
    }

    println!("Part 1 Solution: {:?}", part1(&mut data.clone()).unwrap());
    println!("Part 2 Solution: {:?}", part2(&mut data.clone(), 5, 60).unwrap());
}

#[cfg(test)]
mod day_07 {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: Vec<String> = vec![
            "Step C must be finished before step A can begin.".to_string(),
            "Step C must be finished before step F can begin.".to_string(),
            "Step A must be finished before step B can begin.".to_string(),
            "Step A must be finished before step D can begin.".to_string(),
            "Step B must be finished before step E can begin.".to_string(),
            "Step D must be finished before step E can begin.".to_string(),
            "Step F must be finished before step E can begin.".to_string(),
        ];

        static ref TEST_DATA: HashMap<char, Vec<char>> = {
            let mut m = HashMap::new();
            m.insert('C', vec![]);
            m.insert('A', vec!['C']);
            m.insert('F', vec!['C']);
            m.insert('B', vec!['A']);
            m.insert('D', vec!['A']);
            m.insert('E', vec!['B', 'D', 'F']);
            m
        };
    }

    #[test]
    fn test_parse_line() {
        assert_eq!(parse_line(&TEST_INPUT[0]), ('C', 'A'));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&mut TEST_DATA.clone()), Some("CABDFE".to_string()));
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&mut TEST_DATA.clone(), 2, 0), Some(15));
    }
}
