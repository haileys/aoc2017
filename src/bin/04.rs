extern crate aoc2017;
extern crate itertools;

use aoc2017::read_input;
use itertools::Itertools;

fn main() {
    let answer = read_input()
        .lines()
        .map(|line| line.split_whitespace().collect::<Vec<_>>())
        .filter(|phrases| phrases.len() == phrases.iter().unique().count())
        .count();

    println!("{}", answer);
}
