#[cfg(test)]
#[macro_use]
extern crate lazy_static;

use std::fs::File;
use std::io::Read;
use std::iter::IntoIterator;

const INPUT_FILE: &'static str = "./input/input_08.txt";

#[derive(Debug, Default)]
struct Node {
    children: Vec<Node>,
    metadata: Vec<usize>,
}

fn read_file() -> Vec<usize> {
    let mut f = File::open(INPUT_FILE).unwrap();
    let mut content = String::new();
    f.read_to_string(&mut content).unwrap();

    content.split_whitespace().into_iter()
        .map(|x| x.parse::<usize>().unwrap())
        .collect::<Vec<usize>>()
}

fn gen_tree(it: &mut impl Iterator<Item = usize>) -> Option<Node> {
    // Is there more data to be read?
    let children = match it.next() {
        None => return None,
        Some(c) => c,
    };

    let mut node = Node::default();
    let metadata = it.next().unwrap();

    // Perform a DFS-style recursion on the current node's children.
    for _ in 0..children {
        node.children.extend(gen_tree(it));
    }

    // Access the metadata for the current node.
    for _ in 0..metadata {
        node.metadata.push(it.next().unwrap());
    }

    Some(node)
}

fn part1(root: &Node) -> usize {
    // Add the current node's metadata to the accumulated metadata of it's children.
    root.metadata.iter().sum::<usize>() +
        root.children.iter().map(|c| part1(c)).sum::<usize>()
}

fn part2(root: &Node) -> usize {
    if root.children.is_empty() {
        // If the node has no children, sum the metadata.
        root.metadata.iter().sum::<usize>()
    } else {
        let mut s = 0;

        // If the node does have children, perform recursion on its children.
        for m in root.metadata.iter() {
            s += root.children.get(m - 1).map(|c| part2(c)).unwrap_or_default();
        }

        s
    }
}

fn main() {
    let input = read_file();
    let root = gen_tree(&mut input.into_iter()).unwrap();

    println!("Part 1 Solution: {:?}", part1(&root));
    println!("Part 2 Solution: {:?}", part2(&root));
}


#[cfg(test)]
mod day_08 {
    use super::*;

    lazy_static! {
        static ref TEST_INPUT: Vec<usize> = vec![
            2, 3, 0, 3, 10, 11, 12, 1, 1, 0, 1, 99, 2, 1, 1, 2
        ];

        static ref TEST_TREE: Node = gen_tree(&mut TEST_INPUT.clone().into_iter()).unwrap();
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(&TEST_TREE), 138);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(&TEST_TREE), 66);
    }
}
