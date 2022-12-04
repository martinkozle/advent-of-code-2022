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
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            chunk
                .iter()
                .map(|line| line.chars().collect::<HashSet<_>>())
        })
        .map(|mut sets| {
            (
                sets.next().unwrap(),
                sets.next().unwrap(),
                sets.next().unwrap(),
            )
        })
        .map(|(set1, set2, set3)| {
            set1.intersection(&set2)
                .copied()
                .collect::<HashSet<_>>()
                .intersection(&set3)
                .next()
                .unwrap()
                .to_owned()
        })
        .map(priority)
        .sum::<u32>()
        .to_string()
}
