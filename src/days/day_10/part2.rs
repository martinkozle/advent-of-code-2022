use anyhow::{bail, ensure};
use itertools::Itertools;

type Crt = [[char; 40]; 6];

trait Drawable {
    fn draw(&mut self, sprite_location: i32, cycle: usize) -> anyhow::Result<()>;
}

impl Drawable for Crt {
    fn draw(&mut self, sprite_location: i32, cycle: usize) -> anyhow::Result<()> {
        ensure!(cycle < 40 * 6, "cycle is larger than total CRT resolution");
        let row: usize = (cycle - 1) / 40;
        let col: usize = (cycle - 1) % 40;
        if sprite_location.abs_diff(col as i32) <= 1 {
            self[row][col] = '#';
        }
        Ok(())
    }
}

pub fn solve(input: String) -> anyhow::Result<String> {
    let mut cycle: usize = 0;
    let mut x: i32 = 1;
    let mut crt: Crt = [['.'; 40]; 6];
    for line in input.lines() {
        cycle += 1;
        crt.draw(x, cycle)?;
        match line {
            "noop" => {}
            other => match other.split_once(' ') {
                Some(("addx", amount)) if amount.parse::<i32>().is_ok() => {
                    cycle += 1;
                    crt.draw(x, cycle)?;
                    x += amount.parse::<i32>().unwrap();
                }
                _ => bail!("unexpected input command"),
            },
        };
    }
    Ok(crt
        .iter()
        .map(|row| row.iter().collect::<String>())
        .join("\n"))
}
