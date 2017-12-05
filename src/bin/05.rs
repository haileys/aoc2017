extern crate aoc2017;

use aoc2017::read_input;

fn main() {
    let mut jumps = read_input()
        .lines()
        .map(|j| j.parse().expect("to parse"))
        .collect::<Vec<isize>>();

    let mut pc: isize = 0;

    for i in 0.. {
        if pc < 0 || pc as usize >= jumps.len() {
            println!("{}", i);
            return;
        }

        let offset = jumps[pc as usize];
        jumps[pc as usize] += 1;
        pc += offset;
    }
}
