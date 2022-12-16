mod days;
use anyhow::{bail, Context};
use clap::Parser;
use days::*;
use std::{
    fs,
    io::{self, Read},
    time::Instant,
};

/// Advent of Code 2022
#[derive(Parser, Debug)]
#[command(author = "martinkozle", version, about, long_about = None)]
struct Args {
    /// Challenge day
    day: u32,
    /// Day part
    #[arg(value_parser = clap::value_parser!(u32).range(1..=2), default_value_t = 1)]
    part: u32,
    /// Flag to take input from standard input instead of file,
    /// useful for the small examples or piping in custom input
    #[arg(short, long, action)]
    std_input: bool,
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let solve_function: fn(String) -> anyhow::Result<String> = match (args.day, args.part) {
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
        (6, 1) => day_06::part1::solve,
        (6, 2) => day_06::part2::solve,
        (7, 1) => day_07::part1::solve,
        (7, 2) => day_07::part2::solve,
        (8, 1) => day_08::part1::solve,
        (8, 2) => day_08::part2::solve,
        (9, 1) => day_09::part1::solve,
        (9, 2) => day_09::part2::solve,
        (10, 1) => day_10::part1::solve,
        (10, 2) => day_10::part2::solve,
        (11, 1) => day_11::part1::solve,
        (11, 2) => day_11::part2::solve,
        (12, 1) => day_12::part1::solve,
        (12, 2) => day_12::part2::solve,
        (13, 1) => day_13::part1::solve,
        (13, 2) => day_13::part2::solve,
        // Add new days above this line
        _ => {
            bail!("Unsolved day or part");
        }
    };
    let file_to_read = format!("inputs/day_{:02}.txt", args.day);
    let input = if args.std_input {
        let mut buffer = String::new();
        io::stdin()
            .read_to_string(&mut buffer)
            .context("error occured while reading from standard input")?;
        Ok(buffer)
    } else {
        fs::read_to_string(&file_to_read).context(format!(
            "Error occured while reading file `{}`",
            file_to_read
        ))
    }?;
    let start = Instant::now();
    let answer = solve_function(input)?;
    let duration = start.elapsed();
    println!("{}", answer);
    println!("Solution took {:?}", duration);
    Ok(())
}
