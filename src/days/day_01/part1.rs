use anyhow::anyhow;

pub fn solve(input: String) -> anyhow::Result<String> {
    Ok(input
        .split("\n\n")
        .map(|group| {
            Ok(group
                .trim()
                .lines()
                .map(|line| Ok(line.parse::<u32>()?))
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .sum::<u32>())
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .max()
        .ok_or_else(|| anyhow!("no groups in input"))?
        .to_string())
}
