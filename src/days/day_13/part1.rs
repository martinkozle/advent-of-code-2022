use std::cmp::Ordering;

use anyhow::{bail, Context};

#[derive(Debug)]
struct PacketList {
    packets: Vec<Packet>,
}

#[derive(Debug, Clone)]
struct PacketValue {
    value: u32,
}

#[derive(Debug)]
enum Packet {
    L(PacketList),
    V(PacketValue),
}

impl Packet {
    fn from_string(string: &str) -> anyhow::Result<Self> {
        let start_bracket = string.starts_with('[');
        let end_bracket = string.ends_with(']');
        if start_bracket ^ end_bracket {
            bail!("unmatched beginning or end bracket");
        }
        if !start_bracket && !end_bracket {
            return Ok(Self::V(PacketValue {
                value: string.parse::<u32>()?,
            }));
        }
        let mut packets = Vec::<Packet>::new();
        let mut collector = Vec::<char>::new();
        let mut depth: u32 = 0;
        let mut chars = string.chars();
        chars.next_back();
        for chr in chars.skip(1) {
            match chr {
                '[' => {
                    collector.push(chr);
                    depth += 1;
                }
                ']' => {
                    collector.push(chr);
                    depth -= 1;
                    if depth == 0 && !collector.is_empty() {
                        packets.push(Packet::from_string(&collector.iter().collect::<String>())?);
                        collector.clear();
                    }
                }
                ',' => {
                    if depth == 0 {
                        if !collector.is_empty() {
                            packets
                                .push(Packet::from_string(&collector.iter().collect::<String>())?);
                            collector.clear();
                        }
                    } else {
                        collector.push(chr);
                    }
                }
                digit if digit.is_ascii_digit() => collector.push(digit),
                other => bail!("invalid character `{}`", other),
            }
        }
        if !collector.is_empty() {
            packets.push(Packet::from_string(&collector.iter().collect::<String>())?);
        }
        Ok(Self::L(PacketList { packets }))
    }

    fn cmp(&self, other: &Packet) -> Ordering {
        match (self, other) {
            (Packet::V(packet1), Packet::V(packet2)) => packet1.cmp(packet2),
            (Packet::V(packet1), Packet::L(packet2)) => {
                PacketList::from_value(packet1).cmp(packet2)
            }
            (Packet::L(packet1), Packet::V(packet2)) => {
                packet1.cmp(&PacketList::from_value(packet2))
            }
            (Packet::L(packet1), Packet::L(packet2)) => packet1.cmp(packet2),
        }
    }
}

impl PacketValue {
    fn cmp(&self, other: &PacketValue) -> Ordering {
        self.value.cmp(&other.value)
    }
}

impl PacketList {
    fn from_value(packet: &PacketValue) -> Self {
        PacketList {
            packets: vec![Packet::V((*packet).clone())],
        }
    }

    fn cmp(&self, other: &PacketList) -> Ordering {
        self.packets
            .iter()
            .zip(other.packets.iter())
            .map(|(packet1, packet2)| packet1.cmp(packet2))
            .find(|ordering| ordering.is_ne())
            .unwrap_or_else(|| self.packets.len().cmp(&other.packets.len()))
    }
}

pub fn solve(input: String) -> anyhow::Result<String> {
    let pairs: Vec<(Packet, Packet)> = input
        .split("\n\n")
        .map(|pair| {
            let (packet1, packet2) = pair.trim().split_once('\n').context("Invalid pair")?;
            Ok((Packet::from_string(packet1)?, Packet::from_string(packet2)?))
        })
        .collect::<anyhow::Result<_>>()?;
    Ok(pairs
        .iter()
        .enumerate()
        .filter(|(_, (packet1, packet2))| packet1.cmp(packet2).is_lt())
        .map(|(i, _)| i + 1)
        .sum::<usize>()
        .to_string())
}
