use anyhow::anyhow;
use itertools::Itertools;

pub fn solve(input: String) -> anyhow::Result<String> {
    Ok((input
        .chars()
        .collect::<Vec<_>>()
        .windows(4)
        .find_position(|window| window.iter().all_unique())
        .ok_or_else(|| anyhow!("input doesn't contain a window of 4 unique chars"))?
        .0
        + 4)
    .to_string())
}
