use aoc_core::*;

#[derive(Clone, Copy, PartialEq)]
enum MoveType {
    Rock = 1,
    Paper = 2,
    Scissors = 3,
}

impl From<&str> for MoveType {
    fn from(value: &str) -> Self {
        match value {
            "A" | "X" => MoveType::Rock,
            "B" | "Y" => MoveType::Paper,
            "C" | "Z" => MoveType::Scissors,
            _ => panic!("Invalid move."),
        }
    }
}

impl MoveType {
    pub fn win_over(&self) -> MoveType {
        match self {
            MoveType::Rock => MoveType::Paper,
            MoveType::Paper => MoveType::Scissors,
            MoveType::Scissors => MoveType::Rock,
        }
    }

    pub fn lose_to(&self) -> MoveType {
        match self {
            MoveType::Paper => MoveType::Rock,
            MoveType::Scissors => MoveType::Paper,
            MoveType::Rock => MoveType::Scissors,
        }
    }
}

fn part_one(data: &str) -> (u32, u32) {
    let rounds = data.split("\r\n");

    let mut score = (0u32, 0u32);

    for round in rounds {
        let (first, second) = round.split_once(" ").unwrap();

        let first = MoveType::from(first);
        let second = MoveType::from(second);

        score.0 += first as u32;
        score.1 += second as u32;

        if first.lose_to().eq(&second) {
            score.0 += 6;
        } else if second.lose_to().eq(&first) {
            score.1 += 6;
        } else {
            score.0 += 3;
            score.1 += 3;
        }
    }

    score
}

fn part_two(data: &str) -> (u32, u32) {
    let rounds = data.split("\r\n");

    let mut score = (0u32, 0u32);

    for round in rounds {
        let (first, second) = round.split_once(" ").unwrap();

        let first = MoveType::from(first);
        let second = match second {
            "X" => first.lose_to(),
            "Y" => first,
            "Z" => first.win_over(),
            _ => unreachable!(),
        };

        score.0 += first as u32;
        score.1 += second as u32;

        if first.lose_to().eq(&second) {
            score.0 += 6;
        } else if second.lose_to().eq(&first) {
            score.1 += 6;
        } else {
            score.0 += 3;
            score.1 += 3;
        }
    }

    score
}

AOC!(2);
