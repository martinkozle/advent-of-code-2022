use std::{cmp::Reverse, collections::BinaryHeap};

pub fn solve(input: String) -> String {
    let elf_calories = input.split("\n\n").map(|group| {
        group
            .trim()
            .lines()
            .map(|line| line.parse::<u32>().unwrap())
            .sum::<u32>()
    });
    let mut min_heap = BinaryHeap::<Reverse<_>>::new();
    for elf in elf_calories {
        if min_heap.len() < 3 {
            min_heap.push(Reverse(elf))
        } else if elf > min_heap.peek().unwrap().0 {
            let mut smallest = min_heap.peek_mut().unwrap();
            *smallest = Reverse(elf);
        }
    }
    let answer: u32 = min_heap.iter().map(|reverse| reverse.0).sum();
    answer.to_string()
}
