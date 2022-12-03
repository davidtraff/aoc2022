use std::collections::HashSet;

use aoc_core::*;

fn transform_char(char: &char) -> u32 {
    let value = *char as u32;

    if char.is_lowercase() {
        value - 96
    } else {
        value - 38
    }
}

fn part_one(data: &str) -> u32 {
    let lines = data.split("\r\n").map(|line| line.split_at(line.len() / 2));

    let mut intersections = vec![];

    for (a, b) in lines {
        let a: HashSet<_> = a.chars().collect();
        let b: HashSet<_> = b.chars().collect();

        let intersection = a.intersection(&b).map(transform_char);

        intersections.extend(intersection);
    }

    intersections.iter().sum()
}

fn part_two(data: &str) -> u32 {
    let lines: Vec<_> = data.split("\r\n").collect();

    let mut sum = 0u32;
    for chunk in lines.chunks(3) {
        let mut lines = chunk
            .iter()
            .map(|line| line.chars().collect::<HashSet<_>>());

        let mut set = lines.next().unwrap();
        for line in lines {
            set = set.intersection(&line).copied().collect();
        }

        let value: u32 = set.iter().map(transform_char).sum();

        sum = sum + value;
    }

    sum
}

AOC!(1);
