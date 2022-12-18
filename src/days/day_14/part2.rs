use std::{collections::HashMap, fmt::Display};

use anyhow::{anyhow, bail};
use itertools::Itertools;

struct CaveSlice {
    tiles: HashMap<(u32, u32), char>,
    lowest_wall_y: u32,
}

impl Display for CaveSlice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.tiles.is_empty() {
            return write!(f, ".");
        }
        let min_x = self.tiles.keys().map(|(x, _)| x).min().unwrap();
        let max_x = self.tiles.keys().map(|(x, _)| x).max().unwrap();
        let min_y: u32 = 0;
        let max_y = self.tiles.keys().map(|(_, y)| y).max().unwrap();
        let matrix = (min_y..=*max_y)
            .map(|y| {
                (*min_x..=*max_x)
                    .map(|x| self.tiles.get(&(x, y)).unwrap_or(&'.'))
                    .collect::<String>()
            })
            .join("\n");
        write!(f, "{}", matrix)
    }
}

impl CaveSlice {
    fn from_string(string: &str) -> anyhow::Result<Self> {
        let mut tiles = string
            .lines()
            .map(|line| {
                Ok(line
                    .split(" -> ")
                    .map(|coords| match coords.split_once(',') {
                        Some((x, y)) => Ok((x.parse::<u32>()?, y.parse::<u32>()?)),
                        None => Err(anyhow!("invalid input coords `{}`", coords)),
                    })
                    .collect::<anyhow::Result<Vec<_>>>()?
                    .into_iter()
                    .tuple_windows()
                    .flat_map(|((from_x, from_y), (to_x, to_y))| {
                        (from_x.min(to_x)..=from_x.max(to_x)).flat_map(move |x| {
                            (from_y.min(to_y)..=from_y.max(to_y)).map(move |y| ((x, y), '#'))
                        })
                    }))
            })
            .collect::<anyhow::Result<Vec<_>>>()?
            .into_iter()
            .flatten()
            .collect::<HashMap<_, _>>();
        let mut lowest_wall_y = *tiles
            .iter()
            .filter(|(_, c)| **c == '#')
            .map(|((_, y), _)| y)
            .max()
            .unwrap_or(&0);
        tiles.extend(
            (0..=1000)
                .map(|x| ((x, lowest_wall_y + 2), '#'))
                .collect::<HashMap<(u32, u32), char>>()
                .drain(),
        );
        lowest_wall_y += 2;
        Ok(CaveSlice {
            tiles,
            lowest_wall_y,
        })
    }

    fn simulate_sand(&mut self, spawn_position: (u32, u32)) -> anyhow::Result<bool> {
        if self.tiles.contains_key(&spawn_position) {
            return Ok(false);
        }
        let mut position_x;
        let mut position_y;
        (position_x, position_y) = spawn_position;
        let mut flag = true;
        while flag {
            flag = false;
            for (mx, my) in [(0, 1), (-1, 1), (1, 1)] {
                let new_position_x = u32::try_from(position_x as i64 + mx)?;
                let new_position_y = u32::try_from(position_y as i64 + my)?;
                if !self.tiles.contains_key(&(new_position_x, new_position_y)) {
                    position_x = new_position_x;
                    position_y = new_position_y;
                    flag = true;
                    break;
                }
            }
            if position_y > self.lowest_wall_y {
                // for part 2 we want to error if this happens
                bail!("sand fell off the bottom platform");
            }
        }
        self.tiles.insert((position_x, position_y), 'O');
        Ok(true)
    }
}

pub fn solve(input: String) -> anyhow::Result<String> {
    let mut cave_slice = CaveSlice::from_string(&input)?;
    while cave_slice.simulate_sand((500, 0))? {}
    // println!("{}", cave_slice);
    Ok(cave_slice
        .tiles
        .values()
        .filter(|c| **c == 'O')
        .count()
        .to_string())
}
