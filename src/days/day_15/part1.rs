use anyhow::{anyhow, ensure, Context};
use itertools::Itertools;
use regex::Regex;

fn solve_from_strings<'a, I>(strings: I, regex: Regex, row_of_interest: i32) -> anyhow::Result<u32>
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
    let borders = input_coordinates
        .into_iter()
        .filter_map(move |(sensor_x, sensor_y, beacon_x, beacon_y)| {
            let distance = sensor_x.abs_diff(beacon_x) + sensor_y.abs_diff(beacon_y);
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
        .collect::<Vec<_>>();
    if borders.is_empty() {
        return Ok(0);
    }

    let mut prev_x = borders.first().unwrap().0;
    let mut depth = 1;
    let mut count: u32 = 0;
    for (x, border_type) in borders {
        if depth > 0 {
            count += x.abs_diff(prev_x);
        }
        depth += border_type;
        prev_x = x;
    }
    Ok(count)
}

pub fn solve(input: String) -> anyhow::Result<String> {
    const ROW_OF_INTEREST: i32 = 2000000;
    let regex = Regex::new(
        r"Sensor at x=(?P<sensor_x>-?\d+), y=(?P<sensor_y>-?\d+): closest beacon is at x=(?P<beacon_x>-?\d+), y=(?P<beacon_y>-?\d+)"
    ).unwrap();
    Ok(solve_from_strings(input.lines(), regex, ROW_OF_INTEREST)?.to_string())
}
