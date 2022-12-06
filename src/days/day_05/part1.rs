use regex::Regex;

fn transpose(input: Vec<&str>) -> Vec<String> {
    (0..input[0].len())
        .map(|i| {
            input
                .iter()
                .map(|line| line.chars().nth(i).unwrap())
                .collect::<String>()
        })
        .collect::<Vec<_>>()
}

pub fn solve(input: String) -> String {
    let (stack_string, moves_string) = match input.split("\n\n").collect::<Vec<_>>()[..] {
        [stack_string, moves_string] => (stack_string, moves_string),
        _ => panic!("Unexpected input format"),
    };
    let mut stack_string_lines = stack_string.lines();
    stack_string_lines.next_back();
    let transposed = transpose(stack_string_lines.collect::<Vec<_>>());
    let mut stacks = transposed
        .iter()
        .map(|line| line.trim().chars().rev().collect::<Vec<_>>())
        .filter(|line| !line.is_empty())
        .filter(|line| line.iter().all(|ch| ch.is_alphabetic()))
        .collect::<Vec<_>>();
    let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();
    for move_line in moves_string.lines() {
        let captures = regex
            .captures_iter(move_line)
            .next()
            .expect("There should be one capture for every move line");
        let (amount, from, to) = match (1..=3)
            .map(|i| {
                captures[i]
                    .parse::<usize>()
                    .expect("All capture groups should be parsable as usize")
            })
            .collect::<Vec<_>>()[..]
        {
            [amount, from, to] => (amount, from, to),
            _ => panic!("Regex should capture 3 groups"),
        };
        let stack_len = stacks[from - 1].len();
        let mut to_move = stacks[from - 1][stack_len - amount..].to_owned();
        to_move.reverse();
        stacks[from - 1].truncate(stack_len - amount);
        stacks[to - 1].extend(to_move);
    }
    stacks
        .iter()
        .filter(|stack| !stack.is_empty())
        .map(|stack| stack.last().unwrap())
        .collect::<String>()
}
