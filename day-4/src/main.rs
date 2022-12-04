use aoc_core::*;

struct AssignmentRange {
    low: u8,
    high: u8,
}

impl AssignmentRange {
    pub fn from_str(value: &str) -> Self {
        let (low, high) = value.split_once('-').unwrap();

        Self {
            low: low.parse().unwrap(),
            high: high.parse().unwrap(),
        }
    }

    pub fn includes(&self, other: &AssignmentRange) -> bool {
        self.low <= other.low && self.high >= other.high
    }

    pub fn overlaps(&self, other: &AssignmentRange) -> bool {
        self.low <= other.low && self.high >= other.low
    }
}

fn part_one(data: &str) -> u32 {
    let lines = data
        .split("\r\n")
        .map(|line| line.split_once(',').unwrap());

    let mut count = 0u32;

    for (a, b) in lines {
        let a = AssignmentRange::from_str(a);
        let b = AssignmentRange::from_str(b);

        if a.includes(&b) || b.includes(&a) {
            count += 1;
        }
    }

    count
}

fn part_two(data: &str) -> u32 {
    let lines = data
        .split("\r\n")
        .map(|line| line.split_once(',').unwrap());

    let mut count = 0u32;

    for (a, b) in lines {
        let a = AssignmentRange::from_str(a);
        let b = AssignmentRange::from_str(b);

        if a.overlaps(&b) || b.overlaps(&a) {
            count += 1;
        }
    }

    count
}

AOC!(4);
