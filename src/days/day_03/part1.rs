use std::collections::HashSet;

use anyhow::{Context, anyhow};

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
        .map(|line| line.split_at(line.chars().count() / 2))
        .map(|(first, second)| {
            (
                first.chars().collect::<HashSet<_>>(),
                second.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(first, second)| {
            priority(
                first
                    .intersection(&second)
                    .next()
                    .context("invalid input, no common character")?
                    .to_owned(),
            )
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .sum::<u32>()
        .to_string())
}
