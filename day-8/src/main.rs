use std::collections::HashSet;

use aoc_core::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
struct Position(i32, i32);

impl Position {
    pub fn normalize(&self) -> Self {
        let len = self.len();

        Self(self.0 / len, self.1 / len)
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

fn shoot_ray(origin: Position, to: Position, grid: &Vec<Vec<i32>>) -> Option<Position> {
    let own_height = grid[origin.1 as usize][origin.0 as usize];
    let step = (to - origin).normalize();
    let mut current_position = origin.clone();

    loop {
        current_position = current_position + step;

        let position = grid
            .get(current_position.1 as usize)
            .map(|grid| grid.get(current_position.0 as usize))
            .flatten();

        if matches!(position, Some(height) if *height >= own_height) {
            return Some(current_position);
        }

        if position.is_none() {
            return None;
        }
    }
}

fn part_one(data: &str) -> i32 {
    let grid: Vec<Vec<_>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let height = grid.len() as i32;
    let width = grid[0].len() as i32;

    let mut visible = 0;

    for y in 1..(height - 1) {
        for x in 1..(width - 1) {
            let origin = Position(x, y);
            let rays = [
                // Left
                shoot_ray(origin, Position(0, y), &grid),
                // Right
                shoot_ray(origin, Position(width, y), &grid),
                // Top
                shoot_ray(origin, Position(x, 0), &grid),
                // Bottom
                shoot_ray(origin, Position(x, height), &grid),
            ];

            // Finns det någon som INTE träffade, lägg till.
            if rays.iter().any(|hit| hit.is_none()) {
                visible += 1;
            }
        }
    }

    let exterior = width * 2 + height * 2 - 4;

    visible + exterior
}

fn part_two(data: &str) -> i32 {
    let grid: Vec<Vec<_>> = data
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect();

    let height = (grid.len() - 1) as i32;
    let width = (grid[0].len() - 1) as i32;

    let mut max_score = 0;

    for y in 1..height {
        for x in 1..width {
            let origin = Position(x, y);
            let rays = [
                // Left
                shoot_ray(origin, Position(0, y), &grid).unwrap_or(Position(0, y)),
                // Right
                shoot_ray(origin, Position(width, y), &grid).unwrap_or(Position(width, y)),
                // Top
                shoot_ray(origin, Position(x, 0), &grid).unwrap_or(Position(x, 0)),
                // Bottom
                shoot_ray(origin, Position(x, height), &grid).unwrap_or(Position(x, height)),
            ];

            let score = rays
                .iter()
                .map(|ray| (*ray - origin).len())
                .fold(1, |acc, dst| acc * dst);

            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

AOC!(1);
