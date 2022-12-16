use std::{cmp::Reverse, collections::BinaryHeap};

pub fn solve(input: String) -> anyhow::Result<String> {
    let elf_calories = input
        .split("\n\n")
        .map(|group| {
            Ok(group
                .trim()
                .lines()
                .map(|line| Ok(line.parse::<u32>()?))
                .collect::<anyhow::Result<Vec<_>>>()?
                .into_iter()
                .sum::<u32>())
        })
        .collect::<anyhow::Result<Vec<_>>>()?;
    let mut min_heap = BinaryHeap::<Reverse<_>>::new();
    for elf in elf_calories {
        if min_heap.len() < 3 {
            min_heap.push(Reverse(elf))
        } else if let Some(mut smallest) = min_heap.peek_mut() {
            if elf > smallest.0 {
                *smallest = Reverse(elf);
            }
        }
    }
    Ok(min_heap
        .iter()
        .map(|reverse| reverse.0)
        .sum::<u32>()
        .to_string())
}
