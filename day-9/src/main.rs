use std::collections::HashSet;

use aoc_core::*;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
struct Position(i32, i32);

impl Position {
    pub fn signum(&self) -> Self {
        Self(self.0.min(1).max(-1), self.1.min(1).max(-1))
    }

    pub fn len(&self) -> i32 {
        let len = (self.0 * self.0 + self.1 * self.1) as f32;

        len.sqrt() as i32
    }
}

impl std::ops::Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl std::ops::Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn move_tails(tails: &mut Vec<Position>) {
    let end = tails.len() - 1;

    for i in 0..end {
        let target = i + 1;
        let difference = tails[i] - tails[target];

        if difference.len() > 1 {
            tails[target] = tails[target] + difference.signum();
        }
    }
}

fn solve(data: &str, tail_count: usize) -> usize {
    let steps: Vec<(&str, i32)> = data
        .lines()
        .map(|line| line.split_once(" ").unwrap())
        .map(|(d, l)| (d, l.parse().unwrap()))
        .collect();

    let mut visited = HashSet::new();
    let mut tails = vec![Position(0, 0); tail_count + 1];

    for (direction, length) in steps {
        let direction = match direction {
            "L" => Position(-1, 0),
            "R" => Position(1, 0),
            "U" => Position(0, -1),
            "D" => Position(0, 1),
            _ => unreachable!(),
        };

        for _ in 0..length {
            tails[0] = tails[0] + direction;

            move_tails(&mut tails);

            visited.insert(tails.last().unwrap().clone());
        }
    }

    visited.len()
}

fn part_one(data: &str) -> usize {
    solve(data, 1)
}

fn part_two(data: &str) -> usize {
    solve(data, 9)
}

AOC!(9);
