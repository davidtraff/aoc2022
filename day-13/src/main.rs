use serde::Deserialize;
use std::cmp::Ordering;

use aoc_core::*;

#[derive(Debug, Clone, Eq, PartialEq, Deserialize)]
#[serde(untagged)]
enum Node {
    Integer(u8),
    List(Vec<Node>),
}

impl Node {
    pub fn from_str(value: &str) -> Self {
        serde_json::from_str::<Node>(value).unwrap()
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        use Node::*;
        use Ordering::*;

        match (self, other) {
            (List(a), List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.cmp(b) {
                        Equal => {}
                        o => return o,
                    };
                }

                a.len().cmp(&b.len())
            }

            (List(_), Integer(b)) => self.cmp(&List(vec![Integer(*b)])),
            (Integer(a), List(_)) => List(vec![Integer(*a)]).cmp(other),

            (Integer(a), Integer(b)) => a.cmp(b),
        }
    }
}

fn part_one(data: &str) -> usize {
    let pairs = data
        .split("\r\n\r\n")
        .map(|pair| pair.split_once("\r\n").unwrap())
        .enumerate();

    let mut sum = 0;

    for (idx, (a_str, b_str)) in pairs {
        let a = Node::from_str(a_str);
        let b = Node::from_str(b_str);

        if a < b {
            sum += idx + 1;
        }
    }

    sum
}

fn part_two(data: &str) -> usize {
    let mut pairs: Vec<_> = data
        .split("\r\n")
        .filter(|line| !line.is_empty())
        .map(|str| Node::from_str(str))
        .collect();

    let a = Node::from_str("[[2]]");
    let b = Node::from_str("[[6]]");

    pairs.push(a.clone());
    pairs.push(b.clone());

    pairs.sort();

    let mut a_idx = 0;
    let mut b_idx = 0;
    for idx in 0..pairs.len() {
        if pairs[idx].eq(&a) {
            a_idx = idx + 1;
        } else if pairs[idx].eq(&b) {
            b_idx = idx + 1;
        }
    }

    a_idx * b_idx
}

AOC!(13);
