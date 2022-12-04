fn shape_score(you: char) -> u32 {
    match you {
        'A' => 1,
        'B' => 2,
        'C' => 3,
        _ => panic!("Invalid you move"),
    }
}

fn score(enemy: char, outcome: char) -> u32 {
    assert!(matches!(enemy, 'A' | 'B' | 'C'), "Invalid enemy move");
    assert!(matches!(outcome, 'X' | 'Y' | 'Z'), "Invalid outcome");
    let outcome_score = match outcome {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => panic!("Invalid outcome"),
    };
    let you = match (enemy, outcome) {
        (enemy, 'Y') => enemy,
        ('A', 'X') => 'C',
        ('A', 'Z') => 'B',
        ('B', 'X') => 'A',
        ('B', 'Z') => 'C',
        ('C', 'X') => 'B',
        ('C', 'Z') => 'A',
        _ => panic!("Invalid input move or outcome"),
    };
    outcome_score + shape_score(you)
}

pub fn solve(input: String) -> String {
    input
        .lines()
        .map(|line| score(line.chars().next().unwrap(), line.chars().last().unwrap()))
        .sum::<u32>()
        .to_string()
}
