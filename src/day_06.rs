use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter::IntoIterator;

const INPUT_FILE: &'static str = "./input/input_06.txt";

pub type Coord = (i32, i32);

fn read_file() -> Vec<String> {
    let f = File::open(INPUT_FILE).unwrap();
    BufReader::new(f).lines().collect::<Result<_, _>>().unwrap()
}

pub fn parse_line(line: String) -> Coord {
    let parts: Vec<&str> = line.split(", ").into_iter().collect();

    (parts[0].parse().unwrap(), parts[1].parse().unwrap())
}

fn manhattan_dist(p1: Coord, p2: Coord) -> u32 {
    return ((p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()) as u32
}

pub fn part1(data: &Vec<Coord>) -> Option<usize> {
    // The furthest points from the origin. Use these as our bounds.
    let grid_min_x = data.iter().min_by_key(|x| x.0).unwrap().0;
    let grid_min_y = data.iter().min_by_key(|x| x.1).unwrap().1;
    let grid_max_x = data.iter().max_by_key(|x| x.0).unwrap().0;
    let grid_max_y = data.iter().max_by_key(|x| x.1).unwrap().1;

    // This hash-map tracks which spaces have been visited and who is closest to them.
    let mut map: HashMap<Coord, Vec<(u32, u32)>> = HashMap::new();

    // Find the closest root(s) to each point on the grid.
    for x in grid_min_x..=grid_max_x {
        for y in grid_min_y..=grid_max_y {
            let mut j = 0;
            
            for root in data {
                let dist = manhattan_dist(*root, (x, y));

                if let Some(v) = map.get_mut(&(x, y)) {
                    let curr_min = v[0].1;

                    if dist < curr_min {
                        // The new value is less than the previous value.
                        v.clear();
                        v.push((j, dist));
                    } else if dist == curr_min {
                        // The new value is equal to the previous value.
                        v.push((j, dist));
                    }
                } else {
                    map.insert((x, y), vec![(j, dist)]);
                }

                j += 1;
            }
        }
    }

    // Remove any points that have more than 1 "closest" roots.
    map = map.into_iter().filter(|(_, v)| v.len() == 1).collect();

    let mut infinites: Vec<u32> = Vec::new();
    let mut closest_counts: HashMap<u32, u32> = HashMap::new();

    // Count how many times a root is the closest.
    for (key, val) in map.iter() {
        if key.0 == grid_min_x || key.0 == grid_max_x || key.1 == grid_min_y || key.1 == grid_max_y {
            if !infinites.contains(&val[0].0) {
                infinites.push(val[0].0);
            }

            continue;
        }
        
        let v = closest_counts.entry(val[0].0).or_insert(0);
        *v += 1;
    }

    // Remove roots that are infinite.
    closest_counts = closest_counts.into_iter().filter(|(k, _)| !infinites.contains(k)).collect();

    // Find the root that has the most closest points.
    let max = closest_counts.iter().max_by_key(|(_, v)| **v).unwrap().1;

    Some(*max as usize)
}

fn part2(data: &Vec<Coord>, limit: u32) -> Option<usize> {
    // The furthest points from the origin. Use these as our bounds.
    let grid_min_x = data.iter().min_by_key(|x| x.0).unwrap().0;
    let grid_min_y = data.iter().min_by_key(|x| x.1).unwrap().1;
    let grid_max_x = data.iter().max_by_key(|x| x.0).unwrap().0;
    let grid_max_y = data.iter().max_by_key(|x| x.1).unwrap().1;

    let mut map: HashMap<Coord, u32> = HashMap::new();

    // Compute the sum of distances from every root to a point.
    for x in grid_min_x..=grid_max_x {
        for y in grid_min_y..=grid_max_y {
            for root in data {
                let dist = manhattan_dist(*root, (x, y));

                let val = map.entry((x, y)).or_insert(0);
                *val += dist;
            }
        }
    }

    // Count the number of points where the sum is below a limit.
    let count = map.into_iter().filter(|(_, v)| v < &limit).count();
    
    Some(count)
}

fn main() {
    let data: Vec<Coord> = read_file().into_iter().map(|x| parse_line(x)).collect();
    
    println!("Part 1 Solution: {:?}", part1(&data).unwrap());
    println!("Part 2 Solution: {:?}", part2(&data, 10_000).unwrap());
}

// Test cases pulled from the instructions (learning unit-testing in Rust).
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_part1() {
        let test_data: Vec<Coord> = vec![
            (1, 1),
            (1, 6),
            (8, 3),
            (3, 4),
            (5, 5),
            (8, 9)
        ];
        
        assert_eq!(part1(&test_data), Some(17));
    }

    #[test]
    fn test_part2() {
        let test_limit = 32;
        let test_data: Vec<Coord> = vec![
            (1, 1),
            (1, 6),
            (8, 3),
            (3, 4),
            (5, 5),
            (8, 9)
        ];

        assert_eq!(part2(&test_data, test_limit), Some(16));
    }
}
