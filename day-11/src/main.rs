use std::collections::VecDeque;

use aoc_core::*;

#[derive(Debug)]
struct Monkey<'a> {
    items: VecDeque<i64>,
    items_inspected: u64,
    operation_args: (&'a str, &'a str, &'a str),
    test_args: (i64, usize, usize),
}

impl<'a> Monkey<'a> {
    pub fn parse(data: &'a str) -> Self {
        let lines: Vec<_> = data.lines().skip(1).collect();

        let (_, items) = lines[0].split_once(":").unwrap();
        let items: VecDeque<_> = items
            .split(',')
            .map(|value| value.trim().parse().unwrap())
            .collect();

        let (_, operation) = lines[1].split_once("=").unwrap();
        let operation = operation.trim().split(" ").collect::<Vec<_>>();

        let operation_args = (operation[0], operation[1], operation[2]);

        let (_, test_divisor) = lines[2].split_once("by ").unwrap();
        let test_divisor = test_divisor.parse().unwrap();

        let (_, test_true) = lines[3].split_once("monkey ").unwrap();
        let test_true = test_true.parse().unwrap();

        let (_, test_false) = lines[4].split_once("monkey ").unwrap();
        let test_false = test_false.parse().unwrap();

        Self {
            items,
            items_inspected: 0,
            operation_args,
            test_args: (test_divisor, test_true, test_false),
        }
    }

    pub fn execute_operation<F: FnMut(i64) -> i64>(&mut self, worry_fn: &mut F) -> Option<(i64, usize)> {
        let worry_level = match self.items.pop_front() {
            Some(level) => level,
            None => return None,
        };

        self.items_inspected += 1;

        let (lhs, op, rhs) = self.operation_args;

        let lhs = match lhs {
            "old" => worry_level,
            other => other.parse().unwrap(),
        };

        let rhs = match rhs {
            "old" => worry_level,
            other => other.parse().unwrap(),
        };

        let result = match op {
            "*" => lhs * rhs,
            "+" => lhs + rhs,
            _ => unreachable!(),
        };

        let result = worry_fn(result);

        let next = if result % self.test_args.0 == 0 {
            self.test_args.1
        } else {
            self.test_args.2
        };

        Some((result, next))
    }
}

fn execute<F: FnMut(i64) -> i64>(monkeys: &mut Vec<Monkey>, rounds: i64, worry_fn: &mut F) -> u64 {
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            while let Some((worry_level, next)) = monkeys[i].execute_operation(worry_fn) {
                monkeys[next].items.push_back(worry_level);
            }
        }
    }

    monkeys.sort_by(|a, b| b.items_inspected.cmp(&a.items_inspected));

    monkeys[0].items_inspected * monkeys[1].items_inspected
}

fn part_one(data: &str) -> u64 {
    let mut monkeys: Vec<_> = data
        .split("\r\n\r\n")
        .map(|monkey| Monkey::parse(monkey))
        .collect();

    execute(&mut monkeys, 20, &mut |x| x / 3)
}

fn part_two(data: &str) -> u64 {
    let mut monkeys: Vec<_> = data
        .split("\r\n\r\n")
        .map(|monkey| Monkey::parse(monkey))
        .collect();

    let base = monkeys.iter().fold(1, |product, m| product * m.test_args.0);

    execute(&mut monkeys, 10_000, &mut |x| x % base)
}

AOC!(11);
