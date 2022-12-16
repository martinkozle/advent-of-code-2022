use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use anyhow::{anyhow, bail, ensure, Context};
use itertools::Itertools;
use regex::Regex;

fn gcd(n: &u64, m: &u64) -> u64 {
    let mut n_mut = *n;
    let mut m_mut = *m;
    assert!(n_mut != 0 && m_mut != 0);
    while m_mut != 0 {
        if m < n {
            std::mem::swap(&mut m_mut, &mut n_mut);
        }
        m_mut %= n_mut;
    }
    n_mut
}

fn gcd_iter(slice: &[u64]) -> u64 {
    let first = *slice.first().unwrap();
    slice.iter().fold(first, |acc, x| gcd(&acc, x))
}

fn lcm_iter(slice: &[u64]) -> u64 {
    slice.iter().product::<u64>() / gcd_iter(slice)
}

struct Monkey {
    id: usize,
    items: VecDeque<u64>,
    operation: Box<dyn Fn(u64) -> u64>,
    divisor: u64,
    if_true_throw_monkey: usize,
    if_false_throw_monkey: usize,
    inspections: u64,
}

impl Debug for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{{id={}, items={:?}, if_true_throw_monkey={}, if_false_throw_monkey={}, inspections={}}}",
            self.id, self.items, self.if_true_throw_monkey, self.if_false_throw_monkey, self.inspections
        )
    }
}

impl Monkey {
    fn from_text(text: &str, regex: &Regex) -> anyhow::Result<Self> {
        const EXPECTED_NAMES: [&str; 7] = [
            "id",
            "items",
            "op",
            "op_value",
            "divisor",
            "if_true_throw_monkey",
            "if_false_throw_monkey",
        ];
        ensure!(
            regex
                .capture_names()
                .filter_map(|name| name)
                .all(|name| EXPECTED_NAMES.contains(&name)),
            "regex didn't contain all expected names"
        );
        let captures = regex
            .captures(text)
            .ok_or_else(|| anyhow!("no captures were matched in text using regex"))?;

        let op = captures.name("op").unwrap().as_str().to_string();
        ensure!(
            op == "+" || op == "*",
            format!("op `{}` isn't one of two supported operations", op)
        );
        Ok(Monkey {
            id: captures
                .name("id")
                .unwrap()
                .as_str()
                .parse()
                .context("id isn't parsable as usize")?,
            items: captures
                .name("items")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|item| {
                    item.parse()
                        .context("an item in items isn't parsable as u32")
                })
                .collect::<anyhow::Result<_>>()?,
            operation: match captures.name("op_value").unwrap().as_str() {
                "old" => Box::new(move |old| match op.as_str() {
                    "+" => old + old,
                    "*" => old * old,
                    _ => panic!("unsupported operation"),
                }),
                other_value if other_value.parse::<u64>().is_ok() => {
                    let op_value = other_value.parse::<u64>().unwrap();
                    Box::new(move |old| match op.as_str() {
                        "+" => old + op_value,
                        "*" => old * op_value,
                        _ => panic!("unsupported operation"),
                    })
                }
                _ => bail!(
                    "invalid op_value `{}`",
                    captures.name("op_value").unwrap().as_str()
                ),
            },
            divisor: captures
                .name("divisor")
                .unwrap()
                .as_str()
                .parse()
                .context("divisor isn't parsable as u32")?,
            if_true_throw_monkey: captures
                .name("if_true_throw_monkey")
                .unwrap()
                .as_str()
                .parse()
                .context("if_true_throw_monkey isn't parsable as usize")?,
            if_false_throw_monkey: captures
                .name("if_false_throw_monkey")
                .unwrap()
                .as_str()
                .parse()
                .context("if_false_throw_monkey isn't parsable as usize")?,
            inspections: 0,
        })
    }

    fn inspect(&self, worry: u64, modulus: u64) -> u64 {
        (self.operation)(worry) % modulus
    }

    fn inspect_first_and_throw(&mut self, modulus: u64) -> anyhow::Result<(usize, u64)> {
        let worry = self
            .items
            .pop_front()
            .context("monkey doesn't have an item to inspect and throw")?;
        let new_worry = self.inspect(worry, modulus);
        self.inspections += 1;
        if new_worry % self.divisor == 0 {
            Ok((self.if_true_throw_monkey, new_worry))
        } else {
            Ok((self.if_false_throw_monkey, new_worry))
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
    divisors_lcm: u64,
}

impl Monkeys {
    fn from_texts<'a, I>(texts: I, regex: Regex) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = &'a str>,
    {
        let monkeys_vec: Vec<Monkey> = texts
            .into_iter()
            .map(|split| Monkey::from_text(split, &regex))
            .collect::<anyhow::Result<_>>()?;
        let divisors_lcm = lcm_iter(
            &monkeys_vec
                .iter()
                .map(|monkey| monkey.divisor)
                .collect::<Vec<_>>(),
        );
        Ok(Monkeys {
            monkeys: monkeys_vec,
            divisors_lcm,
        })
    }

    fn play_round(&mut self) {
        let mut thrown_items = HashMap::<usize, Vec<u64>>::new();
        for monkey in self.monkeys.iter_mut() {
            if let Some(monkey_thrown_items) = thrown_items.get_mut(&monkey.id) {
                monkey.items.extend(monkey_thrown_items.drain(..));
            }
            while let Ok((to_monkey, new_worry)) = monkey.inspect_first_and_throw(self.divisors_lcm)
            {
                thrown_items.entry(to_monkey).or_default().push(new_worry);
            }
        }
        for (to_monkey, monkey_thrown_items) in thrown_items.iter_mut() {
            self.monkeys[*to_monkey]
                .items
                .extend(monkey_thrown_items.drain(..));
        }
    }

    fn get_monkey_business(&self) -> anyhow::Result<u64> {
        ensure!(
            self.monkeys.len() >= 2,
            "at least 2 monkeys are required to calculate monkey business"
        );
        Ok(self
            .monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .sorted_by(|inspections1, inspections2| inspections1.cmp(inspections2))
            .rev()
            .take(2)
            .reduce(|inspections1, inspections2| inspections1 * inspections2)
            .unwrap())
    }
}

pub fn solve(input: String) -> anyhow::Result<String> {
    let monkey_input_regex: Regex = Regex::new(
        r#"Monkey (?P<id>\d+):
  Starting items: (?P<items>[\d, ]*)
  Operation: new = old (?P<op>.) (?P<op_value>.+)
  Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<if_true_throw_monkey>\d+)
    If false: throw to monkey (?P<if_false_throw_monkey>\d+)"#,
    )
    .unwrap();
    let mut monkeys = Monkeys::from_texts(input.split("\n\n"), monkey_input_regex)?;
    for _round in 1..10001 {
        monkeys.play_round();
        // if [
        //     1, 20, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000,
        // ]
        // .contains(&_round)
        // {
        //     println!("Round: {}", _round);
        //     println!("{:#?}", monkeys);
        // }
    }
    Ok(monkeys.get_monkey_business()?.to_string())
}
