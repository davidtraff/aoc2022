use aoc_core::*;

fn part_one(data: &str) -> u32 {
    let max = data
        .split("\r\n\r\n")
        .map(|lines| lines.split("\r\n"))
        .map(|lines| {
            lines
                .map(|line| line.parse::<u32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .max()
        .unwrap();

    max
}

fn part_two(data: &str) -> u32 {
    let mut elves: Vec<_> = data
        .split("\r\n\r\n")
        .map(|lines| lines.split("\r\n"))
        .map(|lines| {
            lines
                .map(|line| line.parse::<u32>().unwrap())
                .fold(0, |acc, x| acc + x)
        })
        .collect();

    elves.sort_by(|a, b| b.partial_cmp(a).unwrap());

    elves[0] + elves[1] + elves[2]
}

AOC!(1);
