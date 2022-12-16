use anyhow::anyhow;
use itertools::Itertools;

pub fn solve(input: String) -> anyhow::Result<String> {
    Ok(input
        .lines()
        .map(|line| {
            match line
                .split(',')
                .map(|range| {
                    match range
                        .split('-')
                        .map(|num| num.parse::<u32>())
                        .collect_tuple()
                    {
                        Some((Ok(num1), Ok(num2))) => Ok((num1, num2)),
                        _ => Err(anyhow!("invalid range in input: `{}`", range)),
                    }
                })
                .collect_tuple()
            {
                Some((range1, range2)) => Ok((range1?, range2?)),
                None => Err(anyhow!("invalid line in input: `{}`", line)),
            }
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .filter(|((fr1, to1), (fr2, to2))| fr1 <= to2 && to1 >= fr2)
        .count()
        .to_string())
}
