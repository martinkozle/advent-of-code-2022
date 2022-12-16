use std::{collections::HashSet, cmp::Ordering};

use anyhow::bail;

fn compare(num1: i32, num2: i32) -> i32 {
    match num1.cmp(&num2) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1,
    }
}

pub fn solve(input: String) -> anyhow::Result<String> {
    let mut pos_h: (i32, i32) = (0, 0);
    let mut positions_t: [(i32, i32); 9] = [(0, 0); 9];
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
                        _ => bail!("invalid direction"),
                    };
                    let mut pos_prev = pos_h;
                    for pos_t in positions_t.iter_mut() {
                        if pos_prev.0.abs_diff(pos_t.0) > 1 || pos_prev.1.abs_diff(pos_t.1) > 1 {
                            *pos_t = (
                                pos_t.0 + compare(pos_prev.0, pos_t.0),
                                pos_t.1 + compare(pos_prev.1, pos_t.1),
                            );
                        }
                        pos_prev = *pos_t;
                    }
                    visited.insert(*positions_t.last().unwrap());
                }
            }
            _ => bail!("unexpected input line"),
        }
    }
    Ok(visited.len().to_string())
}
