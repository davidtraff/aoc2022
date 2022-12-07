use std::{collections::HashMap, path::PathBuf};

use aoc_core::*;

fn parse_tree(data: &str) -> HashMap<PathBuf, u64> {
    let commands = data.split("$").skip(1).map(|cmd| cmd.trim());

    let mut tree = HashMap::new();
    let mut current_path = PathBuf::new();

    for cmd in commands {
        match &cmd[..2] {
            "cd" => {
                match &cmd[3..] {
                    ".." => current_path = current_path.parent().unwrap().to_path_buf(),
                    path => current_path.push(path),
                };
            }
            "ls" => {
                let size: u64 = cmd
                    .lines()
                    .skip(1)
                    .filter_map(|line| {
                        let (size, _) = line.split_once(" ").unwrap();

                        size.parse::<u64>().ok()
                    })
                    .sum();

                tree.insert(current_path.clone(), size);

                let mut current = current_path.parent();
                while let Some(parent) = current {
                    *tree.get_mut(parent).unwrap() += size;
                    current = parent.parent();
                }
            }
            _ => unreachable!(),
        }
    }

    tree
}

fn part_one(data: &str) -> u64 {
    let tree = parse_tree(data);

    tree.into_values().filter(|size| *size <= 100_000).sum()
}

fn part_two(data: &str) -> u64 {
    let tree = parse_tree(data);

    let needed = 30_000_000;
    let available = 70_000_000 - tree.get(&PathBuf::from("/")).unwrap();

    tree.into_values()
        .filter(|size| available + size >= needed)
        .min()
        .unwrap()
}

AOC!(7);
