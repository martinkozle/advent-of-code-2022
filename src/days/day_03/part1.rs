use std::collections::HashSet;

fn priority(item: char) -> u32 {
    match item {
        lowercase if lowercase.is_lowercase() => 1 + lowercase as u32 - 'a' as u32,
        uppercase if uppercase.is_uppercase() => 27 + uppercase as u32 - 'A' as u32,
        _ => panic!("Invalid item"),
    }
}

pub fn solve(input: String) -> String {
    input
        .lines()
        .map(|line| line.split_at(line.chars().count() / 2))
        .map(|(first, second)| {
            (
                first.chars().collect::<HashSet<_>>(),
                second.chars().collect::<HashSet<_>>(),
            )
        })
        .map(|(first, second)| first.intersection(&second).next().unwrap().to_owned())
        .map(priority)
        .sum::<u32>()
        .to_string()
}
