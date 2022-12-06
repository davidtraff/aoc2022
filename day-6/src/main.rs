use aoc_core::*;

fn contains_duplicate(slice: &[u8]) -> bool {
    for src in 0..slice.len() {
        for dst in (src + 1)..slice.len() {
            if slice[src] == slice[dst] {
                return true;
            }
        }
    }

    false
}

fn find_start(data: &str, window_size: usize) -> usize {
    let mut idx = window_size - 1;

    for slice in data.as_bytes().windows(window_size) {
        idx += 1;

        if !contains_duplicate(slice) {
            break;
        }
    }

    idx
}

fn part_one(data: &str) -> usize {
    find_start(data, 4)
}

fn part_two(data: &str) -> usize {
    find_start(data, 14)
}

AOC!(6);
