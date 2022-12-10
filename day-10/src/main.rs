use aoc_core::*;

fn part_one(data: &str) -> i32 {
    let mut cycles = vec![220, 180, 140, 100, 60, 20];

    let mut register: i32 = 1;
    let mut current_cycle = 0;
    let mut result = 0;

    for instruction in data.lines() {
        let mut increment = || -> bool {
            current_cycle += 1;

            if current_cycle == *cycles.last().unwrap() {
                result += current_cycle as i32 * register;
                cycles.pop().unwrap();

                return cycles.len() > 0;
            }

            true
        };

        match &instruction[..4] {
            "noop" => {
                if !increment() {
                    break;
                }
            }
            "addx" => {
                let (_, value) = instruction.split_once(" ").unwrap();

                if !increment() || !increment() {
                    break;
                }

                register += value.parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        };
    }

    result
}

fn part_two(data: &str) {
    let height: i32 = 6;
    let width: i32 = 40;
    let len = (height * width) as usize;
    let mut screen_buffer = vec![' '; len];
    let mut register: i32 = 1;
    let mut current_cycle = 0;
    let mut row_offset = 0;

    for instruction in data.lines() {
        let mut increment = || {
            for offset in -1..=1 {
                let idx: i32 = register + offset + row_offset;

                if idx == current_cycle {
                    screen_buffer[idx as usize] = '#';
                }
            }

            current_cycle += 1;

            if current_cycle % width == 0 {
                row_offset += 40;
            }
        };

        match &instruction[..4] {
            "noop" => {
                increment();
            }
            "addx" => {
                let (_, value) = instruction.split_once(" ").unwrap();

                increment();
                increment();

                register += value.parse::<i32>().unwrap();
            }
            _ => unreachable!(),
        };
    }

    println!("_________________________________________");
    for chunk in screen_buffer.chunks(width as usize) {
        let value: String = chunk.iter().collect();
        println!("|{}|", value);
    }
    println!("‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾‾");
}

AOC!(10);
