use anyhow::{anyhow, ensure};
use itertools::Itertools;
use petgraph::{algo::dijkstra, prelude::DiGraphMap};

fn convert_start_and_end(chr: char) -> char {
    match chr {
        'S' => 'a',
        'E' => 'z',
        other => other,
    }
}

pub fn solve(input: String) -> anyhow::Result<String> {
    ensure!(
        input.lines().map(|line| line.chars().count()).all_equal(),
        "all rows in input don't have same char count"
    );
    ensure!(
        input
            .chars()
            .all(|c| matches!(c, 'a'..='z' | 'S' | 'E' | '\n')),
        "invalid input domain"
    );
    ensure!(
        input.chars().filter(|c| *c == 'S').count() == 1,
        "S isn't contained in input exactly once"
    );
    ensure!(
        input.chars().filter(|c| *c == 'S').count() == 1,
        "E isn't contained in input exactly once"
    );
    let input_matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut graph = DiGraphMap::<(usize, usize), (char, char)>::new();
    let mut end: (usize, usize) = (0, 0);
    for (y, row) in input_matrix.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            for (dy, dx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if (0..input_matrix.len()).contains(&((y as isize + dy) as usize))
                    && (0..row.len()).contains(&((x as isize + dx) as usize))
                {
                    if *elem == 'E' {
                        end = (y, x);
                    }
                    let to_y = (y as isize + dy) as usize;
                    let to_x = (x as isize + dx) as usize;
                    let from_char = convert_start_and_end(*elem);
                    let to_char = convert_start_and_end(input_matrix[to_y][to_x]);
                    if to_char as i16 - from_char as i16 >= -1 {
                        graph.add_edge((y, x), (to_y, to_x), (from_char, to_char));
                    }
                }
            }
        }
    }
    Ok(dijkstra(&graph, end, None, |_| 1)
        .iter()
        .filter_map(|((y, x), weight)| {
            if convert_start_and_end(input_matrix[*y][*x]) == 'a' {
                Some(weight)
            } else {
                None
            }
        })
        .min()
        .ok_or_else(|| anyhow!("no path exists from end to an 'a'"))?
        .to_string())
}
