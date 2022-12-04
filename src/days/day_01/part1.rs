pub fn solve(input: String) -> String {
    input
        .split("\n\n")
        .map(|group| {
            group
                .trim()
                .lines()
                .map(|line| line.parse::<u32>().unwrap())
                .sum::<u32>()
        })
        .max()
        .unwrap()
        .to_string()
}
