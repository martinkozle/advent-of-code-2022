mod days;
use clap::Parser;
use days::*;
use std::{fs, time::Instant};

/// Advent of Code 2022
#[derive(Parser, Debug)]
#[command(author = "martinkozle", version, about, long_about = None)]
struct Args {
    /// Challenge day
    day: u32,
    /// Day part
    #[arg(value_parser = clap::value_parser!(u32).range(1..=2),default_value_t = 1)]
    part: u32,
}

fn main() {
    env_logger::init();
    let args = Args::parse();
    let file_to_read = format!("inputs/day_{:02}.txt", args.day);
    let solve_function = match (args.day, args.part) {
        (1, 1) => day_01::part1::solve,
        (1, 2) => day_01::part2::solve,
        (2, 1) => day_02::part1::solve,
        (2, 2) => day_02::part2::solve,
        (3, 1) => day_03::part1::solve,
        (3, 2) => day_03::part2::solve,
        (4, 1) => day_04::part1::solve,
        (4, 2) => day_04::part2::solve,
        (5, 1) => day_05::part1::solve,
        (5, 2) => day_05::part2::solve,
        _ => {
            log::error!("Unsolved day or part");
            return;
        }
    };
    match fs::read_to_string(&file_to_read) {
        Ok(input) => {
            let start = Instant::now();
            let answer = solve_function(input);
            let duration = start.elapsed();
            println!("{}", answer);
            log::info!("Solution took {:?}", duration)
        }
        Err(_) => log::error!("Could not read file '{}'", file_to_read),
    }
}
