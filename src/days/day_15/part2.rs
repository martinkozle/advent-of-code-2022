use std::collections::HashSet;

use anyhow::{anyhow, ensure, Context};
use itertools::Itertools;
use regex::Regex;

fn solve_from_strings<'a, I>(strings: I, regex: Regex, max_coordinate: i32) -> anyhow::Result<u64>
where
    I: IntoIterator<Item = &'a str>,
{
    const EXPECTED_NAMES: [&str; 4] = ["sensor_x", "sensor_y", "beacon_x", "beacon_y"];
    ensure!(
        regex
            .capture_names()
            .flatten()
            .all(|name| EXPECTED_NAMES.contains(&name)),
        "regex didn't contain all expected names"
    );
    let input_coordinates = strings
        .into_iter()
        .map(|string| {
            regex
                .captures(string)
                .ok_or_else(|| anyhow!("no captures were matched in string using regex"))
        })
        .collect::<anyhow::Result<Vec<_>>>()?
        .into_iter()
        .map(|captures| {
            let sensor_x = captures
                .name("sensor_x")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .context("sensor_x isn't parsable as i32")?;
            let sensor_y = captures
                .name("sensor_y")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .context("sensor_y isn't parsable as i32")?;
            let beacon_x = captures
                .name("beacon_x")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .context("beacon_x isn't parsable as i32")?;
            let beacon_y = captures
                .name("beacon_y")
                .unwrap()
                .as_str()
                .parse::<i32>()
                .context("beacon_y isn't parsable as i32")?;
            Ok((sensor_x, sensor_y, beacon_x, beacon_y))
        })
        .collect::<anyhow::Result<Vec<(i32, i32, i32, i32)>>>()?;
    ensure!(
        !input_coordinates.is_empty(),
        "strings is an empty iterator"
    );
    let borders_vec = (0..=max_coordinate)
        .map(|row_of_interest| {
            input_coordinates
                .iter()
                .filter_map(move |(sensor_x, sensor_y, beacon_x, beacon_y)| {
                    let distance = sensor_x.abs_diff(*beacon_x) + sensor_y.abs_diff(*beacon_y);
                    let my = sensor_y.abs_diff(row_of_interest);
                    if my > distance {
                        return None;
                    }
                    Some([
                        ((sensor_x - distance as i32 + my as i32), 1),
                        ((sensor_x + distance as i32 - my as i32), -1),
                    ])
                })
                .flatten()
                .sorted()
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();
    ensure!(
        borders_vec.iter().all(|borders| !borders.is_empty()),
        "all borders need to have at least one range"
    );
    let beacon_locations = input_coordinates
        .iter()
        .map(|(_, _, beacon_x, beacon_y)| (beacon_x, beacon_y))
        .collect::<HashSet<_>>();
    let possible_beacons = borders_vec
        .into_iter()
        .enumerate()
        .flat_map(|(y, borders)| {
            let mut possible = Vec::<(i32, i32)>::new();
            let mut prev_x = borders.first().unwrap().0;
            if prev_x > 0 {
                possible.extend((0..prev_x).map(|x| (x, y as i32)));
            }
            let mut depth = 1;
            for (x, border_type) in borders.iter().skip(1) {
                if depth == 0 {
                    possible.extend((prev_x + 1..*x).map(|x| (x, y as i32)));
                }
                depth += border_type;
                prev_x = *x;
            }
            if prev_x < max_coordinate {
                possible.extend((prev_x + 1..max_coordinate).map(|x| (x, y as i32)));
            }
            possible.into_iter()
        })
        .filter(|(x, y)| !beacon_locations.contains(&(x, y)))
        .collect::<Vec<_>>();
    ensure!(
        possible_beacons.len() == 1,
        format!(
            "number of possible beacons `{}` is not exactly 1",
            possible_beacons.len()
        )
    );

    let (x, y) = possible_beacons.first().unwrap();

    Ok(((*x as i64) * 4_000_000 + *y as i64) as u64)
}

pub fn solve(input: String) -> anyhow::Result<String> {
    const MAX_COORDINATE: i32 = 4_000_000;
    let regex = Regex::new(
        r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"
    ).unwrap();
    Ok(solve_from_strings(input.lines(), regex, MAX_COORDINATE)?.to_string())
}
