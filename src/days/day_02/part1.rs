use anyhow::anyhow;
use itertools::Itertools;

#[derive(PartialEq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    fn from_enemy(enemy: &char) -> anyhow::Result<Self> {
        match enemy {
            'A' => Ok(Self::Rock),
            'B' => Ok(Self::Paper),
            'C' => Ok(Self::Scissors),
            _ => Err(anyhow!("invalid enemy move")),
        }
    }

    fn from_you(you: &char) -> anyhow::Result<Self> {
        match you {
            'X' => Ok(Self::Rock),
            'Y' => Ok(Self::Paper),
            'Z' => Ok(Self::Scissors),
            _ => Err(anyhow!("invalid you move")),
        }
    }

    fn shape_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

fn score(enemy: Move, you: Move) -> u32 {
    let outcome_score = match (&enemy, &you) {
        (m, n) if m == n => 3,
        (Move::Rock, Move::Rock)
        | (Move::Paper, Move::Paper)
        | (Move::Scissors, Move::Scissors) => 3,
        (Move::Rock, Move::Paper)
        | (Move::Paper, Move::Scissors)
        | (Move::Scissors, Move::Rock) => 6,
        (Move::Rock, Move::Scissors)
        | (Move::Paper, Move::Rock)
        | (Move::Scissors, Move::Paper) => 0,
    };
    outcome_score + you.shape_score()
}

pub fn solve(input: String) -> anyhow::Result<String> {
    Ok(input
        .lines()
        .map(|line| match line.chars().collect_tuple() {
            Some((char1, _, char2)) => {
                Ok(score(Move::from_enemy(&char1)?, Move::from_you(&char2)?))
            }
            _ => Err(anyhow!("line didn't contain 2 chars")),
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .sum::<u32>()
        .to_string())
}
