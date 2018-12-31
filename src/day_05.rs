use std::fs::File;
use std::io::Read;
use std::iter::IntoIterator;

const INPUT_FILE: &'static str = "./input/input_05.txt";
const ALPHABET: &'static str = "abcdefghijklmnopqrstuvwxyz";

fn read_file() -> String {
    let mut f = File::open(INPUT_FILE).unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();
    
    // Make sure to remove any whitespace (e.g. '\n') characters.
    content = content.split_whitespace().into_iter().collect::<String>();
    content
}

fn part1(data: &Vec<char>) -> Option<usize> {
    let mut i = 0;
    let mut output = data.clone();

    while i < output.len()-1 {
        if let (Some(a), Some(b)) = (output.get(i), output.get(i+1)) {
            // Are the two characters the same letter but opposite capitalization?
            if a.eq_ignore_ascii_case(b) && ((a.is_lowercase() && b.is_uppercase()) ||
                                             (a.is_uppercase() && b.is_lowercase())) {
                // Remove the two values that have been tested.
                output.remove(i);
                output.remove(i);

                // We only need to re-check the character behind the current one.
                if i > 0 {
                    i -= 1;
                }

                continue;
            }
        }
        
        i += 1;
    }

    Some(output.len())
}

fn part2(data: &Vec<char>) -> Option<usize> {
    let mut smallest = data.len();

    // Iterate over every letter of the alphabet.
    for c in ALPHABET.chars() {
        let mut working_data = data.clone();
        let mut i = 0;

        // First, remove all characters of the selected letter.
        while i < working_data.len() {
            if working_data[i].eq_ignore_ascii_case(&c) {
                working_data.remove(i);
                continue;
            }

            i += 1;
        }

        // Run part1 on the string now.
        let size = part1(&working_data).unwrap();
        smallest = smallest.min(size);
    }

    Some(smallest)
}

fn main() {
    let data: Vec<char> = read_file().chars().into_iter().collect();

    println!("Part 1 Solution: {:?}", part1(&data).unwrap());
    println!("Part 2 Solution: {:?}", part2(&data).unwrap());
}
