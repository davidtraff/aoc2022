use std::collections::{BinaryHeap, HashMap, HashSet};
use rayon::prelude::*;

use aoc_core::*;

type Coord = (usize, usize);

#[derive(Eq)]
struct Node {
    point: Coord,
    distance: usize,
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .distance
            .cmp(&self.distance)
            .then_with(|| self.point.cmp(&other.point))
    }
}

fn parse_grid(data: &str) -> (Coord, Coord, Vec<Vec<u8>>) {
    let mut grid = vec![];
    let mut starting_position = (0usize, 0usize);
    let mut ending_position = (0usize, 0usize);

    for (y, row_str) in data.lines().enumerate() {
        let mut row = vec![];

        for (x, char) in row_str.chars().enumerate() {
            let char = match char {
                'S' => {
                    starting_position = (x, y);

                    b'a'
                }
                'E' => {
                    ending_position = (x, y);

                    b'z'
                }
                other => other as u8,
            };

            row.push(char - b'a');
        }

        grid.push(row);
    }

    (starting_position, ending_position, grid)
}

fn solve(start: Coord, end: Coord, grid: &Vec<Vec<u8>>) -> usize {
    let mut queue = BinaryHeap::new();
    let mut visited = HashSet::new();
    let mut distances = HashMap::new();

    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            distances.insert((x, y), usize::MAX);
        }
    }

    distances.insert(start, 0);
    queue.push(Node {
        point: start,
        distance: 0,
    });

    while let Some(node) = queue.pop() {
        let point = node.point;
        if !visited.insert(point) {
            continue;
        }

        let point_value = grid[point.1][point.0];

        let neighbours = [
            // Left
            (point.0 - 1, point.1),
            // Right
            (point.0 + 1, point.1),
            // Top
            (point.0, point.1 - 1),
            // Bottom
            (point.0, point.1 + 1),
        ];

        for (x, y) in neighbours {
            let value = grid.get(y).map(|n| n.get(x)).flatten();

            if let Some(value) = value {
                let new_distance = node.distance + 1;

                if *value > point_value + 1 {
                    continue;
                }

                if *distances.get(&(x, y)).unwrap_or(&usize::MAX) > new_distance {
                    distances.insert((x, y), new_distance);
                    queue.push(Node {
                        point: (x, y),
                        distance: new_distance,
                    });
                }
            }
        }
    }

    *distances.get(&end).unwrap()
}

fn part_one(data: &str) -> usize {
    let (start, end, grid) = parse_grid(data);

    solve(start, end, &grid)
}

fn part_two(data: &str) -> usize {
    let (_, end, grid) = parse_grid(data);

    let starting_positions: Vec<_> = grid
        .iter()
        .enumerate()
        .map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, val)| **val == 0)
                .map(|(x, _)| (x, y.clone()))
                .collect::<Vec<_>>()
        })
        .flatten()
        .collect();

    let result = starting_positions.par_iter()
        .map(|pos| solve(pos.clone(), end, &grid))
        .min();

    result.unwrap()
}

AOC!(12);
