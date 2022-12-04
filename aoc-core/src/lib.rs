pub use env_logger;
pub use env_logger::Env;
pub use humantime::format_duration;
pub use log::{debug, error, info, trace};

#[macro_export]
macro_rules! AOC {
    ($day:literal) => {
        use std::time::SystemTime;

        fn main() {
            env_logger::init_from_env(Env::default().filter_or("loglevel", "trace"));

            const data: &'static str = include_str!("../input");
            info!("Running solution for Advent of Code 2022 day {}", $day);

            info!("Running part 1...");
            let now = SystemTime::now();
            let result = part_one(data);
            let duration = now.elapsed().unwrap();
            info!(
                "Part 1 completed in {} with result: \"{:?}\"",
                format_duration(duration),
                result
            );

            info!("Running part two...");
            let now = SystemTime::now();
            let result = part_two(data);
            let duration = now.elapsed().unwrap();
            info!(
                "Part 2 completed in {} with result: \"{:?}\"",
                format_duration(duration),
                result
            );

            info!("Finished!");
        }
    };
}
