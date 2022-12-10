pub fn solve(input: String) -> String {
    let grid: Vec<Vec<u16>> = input
        .lines()
        .map(|line| line.chars().map(|c| c as u16 - '0' as u16).collect())
        .collect();
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
    best.to_string()
}
