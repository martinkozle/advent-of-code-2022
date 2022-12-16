use std::num::ParseIntError;

use anyhow::{anyhow, ensure};
use itertools::Itertools;
use regex::Regex;

fn transpose(input: Vec<&str>) -> Vec<String> {
    (0..input[0].chars().count())
        .map(|i| {
            input
                .iter()
                .map(|line| {
                    line.chars()
                        .nth(i)
                        .expect("every str in vector to have same count of chars")
                })
                .collect::<String>()
        })
        .collect_vec()
}

pub fn solve(input: String) -> anyhow::Result<String> {
    let (stack_string, moves_string) = input
        .split_once("\n\n")
        .ok_or_else(|| anyhow!("invalid input format"))?;
    let mut stack_string_lines = stack_string.lines();
    stack_string_lines.next_back();
    let stack_string_lines_vec = stack_string_lines.collect::<Vec<_>>();
    ensure!(
        stack_string_lines_vec
            .iter()
            .map(|line| line.len())
            .all_equal(),
        "every line in the stack part of the input needs to have the same number of chars"
    );
    let transposed = transpose(stack_string_lines_vec);
    let mut stacks = transposed
        .iter()
        .map(|line| line.trim().chars().rev().collect::<Vec<_>>())
        .filter(|line| !line.is_empty() && line.iter().all(|ch| ch.is_alphabetic()))
        .collect_vec();
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for move_line in moves_string.lines() {
        let captures = regex
            .captures(move_line)
            .ok_or_else(|| anyhow!("there should be a capture for every move line"))?;
        let (amount, from, to) = match (1..=3)
            .map(|i| captures[i].parse::<usize>())
            .collect_tuple()
        {
            Some((amount, from, to)) => {
                Ok::<(usize, usize, usize), ParseIntError>((amount?, from?, to?))
            }
            _ => unreachable!(),
        }?;
        ensure!(
            from - 1 < stacks.len(),
            format!(
                "invalid from stack `{}`, there are only `{}` stacks",
                from,
                stacks.len()
            )
        );
        ensure!(amount > 0, "invalid amount, should be a positive integer");
        ensure!(
            to - 1 < stacks.len(),
            format!(
                "invalid to stack `{}`, there are only `{}` stacks",
                to,
                stacks.len()
            )
        );
        let stack_len = stacks[from - 1].len();
        let to_move = stacks[from - 1].drain(stack_len - amount..).collect_vec();
        stacks[to - 1].extend(to_move);
    }
    Ok(stacks
        .iter()
        .flat_map(|stack| stack.last())
        .collect::<String>())
}
