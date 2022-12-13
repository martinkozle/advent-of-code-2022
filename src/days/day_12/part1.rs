use petgraph::{algo::dijkstra, prelude::DiGraphMap};

fn convert_start_and_end(chr: char) -> char {
    match chr {
        'S' => 'a',
        'E' => 'z',
        other => other,
    }
}

pub fn solve(input: String) -> String {
    let input_matrix: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();
    let mut graph = DiGraphMap::<(usize, usize), (char, char)>::new();
    let mut start: (usize, usize) = (0, 0);
    let mut end: (usize, usize) = (0, 0);
    for (y, row) in input_matrix.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            for (my, mx) in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
                if (0..input_matrix.len()).contains(&((y as isize + my) as usize))
                    && (0..row.len()).contains(&((x as isize + mx) as usize))
                {
                    if *elem == 'S' {
                        start = (y, x);
                    } else if *elem == 'E' {
                        end = (y, x);
                    }
                    let to_y = (y as isize + my) as usize;
                    let to_x = (x as isize + mx) as usize;
                    let from_char = convert_start_and_end(*elem);
                    let to_char = convert_start_and_end(input_matrix[to_y][to_x]);
                    if from_char as i16 - to_char as i16 >= -1 {
                        graph.add_edge((y, x), (to_y, to_x), (from_char, to_char));
                    }
                }
            }
        }
    }
    dijkstra(&graph, start, Some(end), |_| 1)[&end].to_string()
}
