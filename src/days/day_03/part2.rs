use std::collections::HashSet;

use anyhow::{anyhow, Context};
use itertools::Itertools;

fn priority(item: char) -> anyhow::Result<u32> {
    match item {
        lowercase if lowercase.is_ascii_lowercase() => Ok(1 + lowercase as u32 - 'a' as u32),
        uppercase if uppercase.is_ascii_uppercase() => Ok(27 + uppercase as u32 - 'A' as u32),
        _ => Err(anyhow!("invalid item")),
    }
}

pub fn solve(input: String) -> anyhow::Result<String> {
    Ok(input
        .lines()
        .collect_vec()
        .chunks(3)
        .map(|chunk| {
            match chunk
                .iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .collect_tuple()
            {
                Some((set1, set2, set3)) => priority(
                    set1.intersection(&set2)
                        .copied()
                        .collect::<HashSet<_>>()
                        .intersection(&set3)
                        .next()
                        .context("invalid input, no common character")?
                        .to_owned(),
                ),
                _ => Err(anyhow!("line count not divisible by 3")),
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .sum::<u32>()
        .to_string())
}
