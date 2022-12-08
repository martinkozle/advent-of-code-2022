use itertools::Itertools;

pub fn solve(input: String) -> String {
    (input
        .chars()
        .collect::<Vec<_>>()
        .windows(14)
        .find_position(|window| window.iter().all_unique())
        .unwrap()
        .0
        + 14)
        .to_string()
}