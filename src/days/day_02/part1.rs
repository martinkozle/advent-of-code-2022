fn shape_score(you: char) -> u32 {
    match you {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!("Invalid you move"),
    }
}

fn score(enemy: char, you: char) -> u32 {
    assert!(matches!(enemy, 'A' | 'B' | 'C'), "Invalid enemy move");
    assert!(matches!(you, 'X' | 'Y' | 'Z'), "Invalid you move");
    let converted_you = match you {
        'X' => 'A',
        'Y' => 'B',
        'Z' => 'C',
        _ => panic!("Invalid you move"),
    };
    let outcome_score = match (enemy, converted_you) {
        (enemy, you) if enemy == you => 3,
        ('A', 'B') => 6,
        ('A', 'C') => 0,
        ('B', 'A') => 0,
        ('B', 'C') => 6,
        ('C', 'A') => 6,
        ('C', 'B') => 0,
        _ => panic!("Invalid input moves"),
    };
    outcome_score + shape_score(converted_you)
}

pub fn solve(input: String) -> String {
    input
        .lines()
        .map(|line| score(line.chars().next().unwrap(), line.chars().last().unwrap()))
        .sum::<u32>()
        .to_string()
}
