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
    let mut count: u32 = 0;
    for y in 0..grid.len() {
        for x in 0..grid[y].len() {
            let is_less_than_current_position = |el: &u32| *el < grid[y][x];
            if grid[y]
                .iter()
                .skip(x + 1)
                .all(is_less_than_current_position)
                || grid[y].iter().take(x).all(is_less_than_current_position)
                || grid
                    .iter()
                    .map(|row| &row[x])
                    .skip(y + 1)
                    .all(is_less_than_current_position)
                || grid
                    .iter()
                    .map(|row| &row[x])
                    .take(y)
                    .all(is_less_than_current_position)
            {
                count += 1
            }
        }
    }
    Ok(count.to_string())
}
