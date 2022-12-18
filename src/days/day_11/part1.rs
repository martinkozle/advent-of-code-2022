use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

use anyhow::{anyhow, bail, ensure, Context};
use itertools::Itertools;
use regex::Regex;

struct Monkey {
    id: usize,
    items: VecDeque<u32>,
    operation: Box<dyn Fn(u32) -> u32>,
    divisor: u32,
    if_true_throw_monkey: usize,
    if_false_throw_monkey: usize,
    inspections: u32,
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
    fn from_string(string: &str, regex: &Regex) -> anyhow::Result<Self> {
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
                .flatten()
                .all(|name| EXPECTED_NAMES.contains(&name)),
            "regex didn't contain all expected names"
        );
        let captures = regex
            .captures(string)
            .ok_or_else(|| anyhow!("no captures were matched in string using regex"))?;

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
                other_value if other_value.parse::<u32>().is_ok() => {
                    let op_value = other_value.parse::<u32>().unwrap();
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

    fn inspect(&self, worry: u32) -> u32 {
        (self.operation)(worry) / 3
    }

    fn inspect_first_and_throw(&mut self) -> anyhow::Result<(usize, u32)> {
        let worry = self
            .items
            .pop_front()
            .context("monkey doesn't have an item to inspect and throw")?;
        let new_worry = self.inspect(worry);
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
}

impl Monkeys {
    fn from_strings<'a, I>(strings: I, regex: Regex) -> anyhow::Result<Self>
    where
        I: IntoIterator<Item = &'a str>,
    {
        Ok(Monkeys {
            monkeys: strings
                .into_iter()
                .map(|string| Monkey::from_string(string, &regex))
                .collect::<anyhow::Result<_>>()?,
        })
    }

    fn play_round(&mut self) {
        let mut thrown_items = HashMap::<usize, Vec<u32>>::new();
        for monkey in self.monkeys.iter_mut() {
            if let Some(monkey_thrown_items) = thrown_items.get_mut(&monkey.id) {
                monkey.items.extend(monkey_thrown_items.drain(..));
            }
            while let Ok((to_monkey, new_worry)) = monkey.inspect_first_and_throw() {
                thrown_items.entry(to_monkey).or_default().push(new_worry);
            }
        }
        for (to_monkey, monkey_thrown_items) in thrown_items.iter_mut() {
            self.monkeys[*to_monkey]
                .items
                .extend(monkey_thrown_items.drain(..));
        }
    }

    fn get_monkey_business(&self) -> anyhow::Result<u32> {
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
    let mut monkeys = Monkeys::from_strings(input.split("\n\n"), monkey_input_regex)?;
    for _round in 1..21 {
        monkeys.play_round();
        // println!("Round: {}", _round);
        // println!("{:#?}", monkeys);
    }
    Ok(monkeys.get_monkey_business()?.to_string())
}
