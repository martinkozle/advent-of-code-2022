use anyhow::{anyhow, ensure};
use itertools::Itertools;

pub fn solve(input: String) -> anyhow::Result<String> {
    let grid: Vec<Vec<u32>> = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| {
                    c.to_digit(10)
                        .ok_or_else(|| anyhow!("non digit character in input"))
                })
                .collect::<anyhow::Result<Vec<_>>>()
        })
        .collect::<anyhow::Result<_>>()?;
    ensure!(
        grid.iter().map(|row| row.len()).all_equal(),
        "all rows need to have the same number of digits"
    );
    let mut best = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let current = grid[y][x];
            let right = match grid[y]
                .iter()
                .skip(x + 1)
                .enumerate()
                .find(|(_, el)| **el >= current)
            {
                Some((i, _)) => i + 1,
                _ => grid[y].len() - x - 1,
            };
            let left = match grid[y]
                .iter()
                .take(x)
                .rev()
                .enumerate()
                .find(|(_, el)| **el >= current)
            {
                Some((i, _)) => i + 1,
                _ => x,
            };
            let down = match grid
                .iter()
                .map(|row| row[x])
                .skip(y + 1)
                .enumerate()
                .find(|(_, el)| *el >= current)
            {
                Some((i, _)) => i + 1,
                _ => grid.len() - y - 1,
            };
            let up = match grid
                .iter()
                .map(|row| row[x])
                .take(y)
                .rev()
                .enumerate()
                .find(|(_, el)| *el >= current)
            {
                Some((i, _)) => i + 1,
                _ => y,
            };
            best = best.max(right * left * down * up);
        }
    }
    Ok(best.to_string())
}
