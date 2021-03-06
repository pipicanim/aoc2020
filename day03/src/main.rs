use std::fs::File;
use std::io::{prelude::*, BufReader};
#[derive(PartialEq, Debug)]
enum Space {
    Free,
    Tree,
}

fn main() {
    let reader = BufReader::new(File::open("input.txt").unwrap());
    let space = reader
        .lines()
        .map(|x| {
            x.unwrap()
                .chars()
                .map(|x| match x {
                    '.' => Space::Free,
                    '#' => Space::Tree,
                    _ => Space::Free,
                })
                .collect::<Vec<Space>>()
        })
        .collect::<Vec<Vec<Space>>>();

    println!("part01: {:?}", find_trees_on_slope(&space, &(3, 1)));
    println!(
        "part02: {:?}",
        [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]
            .iter()
            .map(|x| find_trees_on_slope(&space, x))
            .fold(1, |acc, x| x * acc)
    );
}

fn find_trees_on_slope(space: &Vec<Vec<Space>>, slope: &(usize, usize)) -> usize {
    space
        .iter()
        .step_by(slope.1)
        .enumerate()
        .filter(|(i, x)| *x.get((i * slope.0) % x.len()).unwrap() == Space::Tree)
        .count()
}
