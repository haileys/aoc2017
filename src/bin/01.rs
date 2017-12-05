extern crate aoc2017;
extern crate itertools;

use aoc2017::read_input;
use itertools::Itertools;

fn main() {
    let digits = read_input()
        .split("")
        .filter_map(|c| c.parse().ok())
        .collect::<Vec<_>>();

    let answer: usize = digits
        .iter()
        .chain(digits.first())
        .tuple_windows()
        .filter(|&(a, b)| a == b)
        .map(|(a, _)| a)
        .sum();

    println!("{}", answer);
}
