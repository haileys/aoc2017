extern crate aoc2017;
extern crate itertools;

use aoc2017::read_input;
use itertools::Itertools;

fn main() {
    let result: usize = read_input()
        .lines()
        .filter_map(|line|
            line.split_whitespace()
                .filter_map(|cell| cell.parse::<usize>().ok())
                .minmax()
                .into_option()
                .map(|(min, max)| max - min))
        .sum();

    println!("{}", result);
}
