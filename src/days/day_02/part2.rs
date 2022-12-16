use anyhow::anyhow;
use itertools::Itertools;

#[derive(PartialEq, Clone)]
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

    fn shape_score(&self) -> u32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

enum Outcome {
    Loss,
    Tie,
    Win,
}

impl Outcome {
    fn from_outcome(outcome: &char) -> anyhow::Result<Self> {
        match outcome {
            'X' => Ok(Self::Loss),
            'Y' => Ok(Self::Tie),
            'Z' => Ok(Self::Win),
            _ => Err(anyhow!("invalid outcome")),
        }
    }
}

fn score(enemy: Move, outcome: Outcome) -> u32 {
    let outcome_score = match &outcome {
        Outcome::Loss => 0,
        Outcome::Tie => 3,
        Outcome::Win => 6,
    };
    let you = match (&enemy, &outcome) {
        (enemy, Outcome::Tie) => enemy.clone(),
        (Move::Rock, Outcome::Loss) | (Move::Paper, Outcome::Win) => Move::Scissors,
        (Move::Rock, Outcome::Win) | (Move::Scissors, Outcome::Loss) => Move::Paper,
        (Move::Paper, Outcome::Loss) | (Move::Scissors, Outcome::Win) => Move::Rock,
    };
    outcome_score + you.shape_score()
}

pub fn solve(input: String) -> anyhow::Result<String> {
    Ok(input
        .lines()
        .map(|line| match line.chars().collect_tuple() {
            Some((char1, _, char2)) => Ok(score(
                Move::from_enemy(&char1)?,
                Outcome::from_outcome(&char2)?,
            )),
            _ => Err(anyhow!("line didn't contain 2 chars")),
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .sum::<u32>()
        .to_string())
}
