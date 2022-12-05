use aoc_core::*;

fn parse_stacks(data: &str) -> Vec<Vec<char>> {
    let mut lines: Vec<_> = data.split("\r\n").map(|line| line.split(' ')).collect();

    lines.reverse();

    let mut iter = lines.into_iter();

    // Skip column-numbers.
    iter.next().unwrap();

    let mut stacks = vec![vec![]; 9];

    for row in iter {
        let mut column_iter = row.into_iter();

        for i in 0..stacks.len() {
            let column = match column_iter.next() {
                Some(c) => c,
                None => break,
            };

            if column.len() == 0 {
                column_iter.next().unwrap();
                column_iter.next().unwrap();
                column_iter.next().unwrap();

                continue;
            }

            stacks[i].push(column.chars().nth(1).unwrap());
        }
    }

    stacks
}

fn parse_move(value: &str) -> (usize, usize, usize) {
    let mut values = value
        .split(' ')
        .filter_map(|value| value.parse::<usize>().ok());

    (
        values.next().unwrap(),
        values.next().unwrap() - 1usize,
        values.next().unwrap() - 1usize,
    )
}

fn part_one(data: &str) -> String {
    let (stacks, moves) = data.split_once("\r\n\r\n").unwrap();

    let mut stacks = parse_stacks(stacks);

    for row in moves.split("\r\n") {
        let (amount, from, to) = parse_move(row);

        for _ in 0..amount {
            let value = stacks[from].pop().unwrap();

            stacks[to].push(value);
        }
    }

    stacks
        .iter()
        .map(|column| *column.last().unwrap())
        .collect()
}

fn part_two(data: &str) -> String {
    let (stacks, moves) = data.split_once("\r\n\r\n").unwrap();

    let mut stacks = parse_stacks(stacks);

    for row in moves.split("\r\n") {
        let (amount, from, to) = parse_move(row);

        let moved_items = stacks[from]
            .iter()
            .rev()
            .take(amount)
            .rev();

        let mut temp: Vec<char> = vec![];

        temp.extend(moved_items);

        for item in temp {
            stacks[from].pop().unwrap();
            stacks[to].push(item);
        }
    }

    stacks
        .iter()
        .map(|column| *column.last().unwrap())
        .collect()
}

AOC!(5);
