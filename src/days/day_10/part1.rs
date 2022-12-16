use anyhow::bail;

pub fn solve(input: String) -> anyhow::Result<String> {
    let mut cycle = 0;
    let mut x = 1;
    let mut sum = 0;
    for line in input.lines() {
        cycle += 1;
        if cycle == 20 || (cycle - 20) % 40 == 0 {
            sum += cycle * x;
        }
        match line {
            "noop" => {}
            other => match other.split_once(' ') {
                Some(("addx", amount)) if amount.parse::<i32>().is_ok() => {
                    cycle += 1;
                    if cycle == 20 || (cycle - 20) % 40 == 0 {
                        sum += cycle * x;
                    }
                    x += amount.parse::<i32>().unwrap();
                }
                _ => bail!("unexpected input command"),
            },
        };
    }
    Ok(sum.to_string())
}
