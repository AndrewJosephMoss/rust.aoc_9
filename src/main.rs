use aoc_9;
use std::fs;

fn main() {
    part_1();
    part_2();
}

fn part_1() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = aoc_9::process_part_1(&input);
    println!("part1: {}", result);
}

fn part_2() {
    let input = fs::read_to_string("input.txt").unwrap();
    let result = aoc_9::process_part_2(&input);
    println!("part2: {}", result);
}
