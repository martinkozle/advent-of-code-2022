use std::collections::HashSet;

fn compare(num1: i32, num2: i32) -> i32 {
    match num1 > num2 {
        true => 1,
        false => match num1 < num2 {
            true => -1,
            false => 0,
        },
    }
}

pub fn solve(input: String) -> String {
    let mut pos_h: (i32, i32) = (0, 0);
    let mut pos_t: (i32, i32) = (0, 0);
    let mut visited = HashSet::<(i32, i32)>::new();
    visited.insert((0, 0));
    for line in input.lines() {
        match line.split_once(' ') {
            Some((direction, amount)) if amount.parse::<usize>().is_ok() => {
                for _ in 0..amount.parse::<usize>().unwrap() {
                    pos_h = match direction {
                        "U" => (pos_h.0 - 1, pos_h.1),
                        "D" => (pos_h.0 + 1, pos_h.1),
                        "L" => (pos_h.0, pos_h.1 - 1),
                        "R" => (pos_h.0, pos_h.1 + 1),
                        _ => panic!("Invalid direction"),
                    };
                    if pos_h.0.abs_diff(pos_t.0) > 1 || pos_h.1.abs_diff(pos_t.1) > 1 {
                        pos_t = (
                            pos_t.0 + compare(pos_h.0, pos_t.0),
                            pos_t.1 + compare(pos_h.1, pos_t.1),
                        );
                        visited.insert(pos_t);
                    }
                }
            }
            _ => panic!("Unexpected input line"),
        }
    }
    visited.len().to_string()
}
