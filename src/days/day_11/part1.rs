use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
};

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
    fn from_text(text: &str, regex: &Regex) -> Self {
        let captures = regex
            .captures(text)
            .expect("Input text should match regex expression");
        let op = captures.name("op").unwrap().as_str().to_string();
        let op_value_string = captures.name("op_value").unwrap().as_str().to_string();
        Monkey {
            id: captures.name("id").unwrap().as_str().parse().unwrap(),
            items: captures
                .name("items")
                .unwrap()
                .as_str()
                .split(", ")
                .map(|item| item.parse().unwrap())
                .collect(),
            operation: Box::new(move |old| {
                let op_value = match op_value_string.as_str() {
                    "old" => old,
                    other => other.parse().unwrap(),
                };
                match op.as_str() {
                    "+" => old + op_value,
                    "*" => old * op_value,
                    _ => panic!("Unsupported operation"),
                }
            }),
            divisor: captures.name("divisor").unwrap().as_str().parse().unwrap(),
            if_true_throw_monkey: captures
                .name("if_true_throw_monkey")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            if_false_throw_monkey: captures
                .name("if_false_throw_monkey")
                .unwrap()
                .as_str()
                .parse()
                .unwrap(),
            inspections: 0,
        }
    }

    fn inspect(&self, worry: u32) -> u32 {
        (self.operation)(worry) / 3
    }

    fn inspect_first_and_throw(&mut self) -> (usize, u32) {
        let worry = self.items.pop_front().unwrap();
        let new_worry = self.inspect(worry);
        self.inspections += 1;
        if new_worry % self.divisor == 0 {
            (self.if_true_throw_monkey, new_worry)
        } else {
            (self.if_false_throw_monkey, new_worry)
        }
    }
}

#[derive(Debug)]
struct Monkeys {
    monkeys: Vec<Monkey>,
}

impl Monkeys {
    fn from_texts<'a, I>(texts: I, regex: Regex) -> Self
    where
        I: IntoIterator<Item = &'a str>,
    {
        Monkeys {
            monkeys: texts
                .into_iter()
                .map(|split| Monkey::from_text(split, &regex))
                .collect_vec(),
        }
    }

    fn play_round(&mut self) {
        let mut thrown_items = HashMap::<usize, Vec<u32>>::new();
        for monkey in self.monkeys.iter_mut() {
            if thrown_items.contains_key(&monkey.id) {
                let monkey_thrown_items = thrown_items.get_mut(&monkey.id).unwrap();
                monkey.items.extend(monkey_thrown_items.iter());
                monkey_thrown_items.clear();
            }
            while !monkey.items.is_empty() {
                let (to_monkey, new_worry) = monkey.inspect_first_and_throw();
                thrown_items.entry(to_monkey).or_default().push(new_worry);
            }
        }
        for (to_monkey, monkey_thrown_items) in thrown_items.iter() {
            self.monkeys[*to_monkey]
                .items
                .extend(monkey_thrown_items.iter());
        }
    }

    fn get_monkey_business(&self) -> u32 {
        self.monkeys
            .iter()
            .map(|monkey| monkey.inspections)
            .sorted_by(|inspections1, inspections2| inspections1.cmp(inspections2))
            .rev()
            .take(2)
            .reduce(|inspections1, inspections2| inspections1 * inspections2)
            .unwrap()
    }
}

pub fn solve(input: String) -> String {
    let monkey_input_regex: Regex = Regex::new(
        r#"Monkey (?P<id>\d+):
  Starting items: (?P<items>[\d, ]*)
  Operation: new = old (?P<op>.) (?P<op_value>.+)
  Test: divisible by (?P<divisor>\d+)
    If true: throw to monkey (?P<if_true_throw_monkey>\d+)
    If false: throw to monkey (?P<if_false_throw_monkey>\d+)"#,
    )
    .unwrap();
    let mut monkeys = Monkeys::from_texts(input.split("\n\n"), monkey_input_regex);
    for _round in 1..21 {
        monkeys.play_round();
        // println!("Round: {}", _round);
        // println!("{:#?}", monkeys);
    }
    monkeys.get_monkey_business().to_string()
}