use std::cmp::Ord;
use std::cmp::Ordering;
use std::str::FromStr;

#[derive(PartialEq, Eq)]
enum Packet {
    Value(u32),
    List(Vec<Packet>),
}

impl FromStr for Packet {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(s) = s.strip_prefix('[') {
            if let Some(s) = s.strip_suffix(']') {
                // list
                let mut opened = 0;
                let mut items = Vec::new();
                let mut builder = String::new();

                for c in s.chars() {
                    match c {
                        '[' => {
                            opened += 1;
                            builder.push('[');
                        }
                        ']' => {
                            opened -= 1;
                            builder.push(']');
                        }
                        ',' => {
                            if opened == 0 {
                                let packet = builder.parse()?;
                                items.push(packet);
                                builder.clear();
                            } else {
                                builder.push(',');
                            }
                        }
                        c => builder.push(c),
                    }
                }
                if !builder.is_empty() {
                    let packet = builder.parse()?;
                    items.push(packet);
                }
                return Ok(Packet::List(items));
            }
        } else {
            // integer
            return Ok(Packet::Value(s.parse::<u32>().unwrap()));
        }

        Err(())
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Packet::Value(v1), Packet::Value(v2)) => v1.cmp(v2),
            (Packet::List(l1), Packet::List(l2)) => {
                for (packet1, packet2) in l1.iter().zip(l2.iter()) {
                    let comparison = packet1.cmp(packet2);
                    if comparison != Ordering::Equal {
                        return comparison;
                    }
                }

                l1.len().cmp(&l2.len())
            }
            (Packet::List(_), Packet::Value(value)) => {
                self.cmp(&Packet::List(vec![Packet::Value(*value)]))
            }
            (Packet::Value(value), Packet::List(_)) => {
                Packet::List(vec![Packet::Value(*value)]).cmp(other)
            }
        }
    }
}

pub fn part1(input: &str) -> usize {
    let pairs = parse1(input);
    pairs
        .iter()
        .enumerate()
        .filter(|(_, (packet1, packet2))| packet1 < packet2)
        .map(|(i, _)| i + 1) // index should start at 1
        .sum()
}

pub fn part2(input: &str) -> usize {
    let mut packets = parse2(input); 
    let divider1 = Packet::List(vec![Packet::List(vec![Packet::Value(2)])]);
    let divider2 = Packet::List(vec![Packet::List(vec![Packet::Value(6)])]);

    packets.sort();

    let pos1 = packets.iter().position(|packet| packet == &divider1).unwrap() + 1;
    let pos2 = packets.iter().position(|packet| packet == &divider2).unwrap() + 1;

    pos1 * pos2
}

fn parse1(input: &str) -> Vec<(Packet, Packet)> {
    input
        .split("\n\n")
        .map(|pairs| {
            let mut iter = pairs.split('\n');
            let left = iter.next().unwrap().parse().unwrap();
            let right = iter.next().unwrap().parse().unwrap();

            (left, right)
        })
        .collect()
}

fn parse2(input: &str) -> Vec<Packet> {
    let mut packets: Vec<Packet> = input
        .split('\n')
        .filter_map(|line| {
            if line.is_empty() {
                None
            } else {
                Some(line.parse().unwrap())
            }
        })
        .collect();
    packets.push(Packet::List(vec![Packet::List(vec![Packet::Value(2)])]));
    packets.push(Packet::List(vec![Packet::List(vec![Packet::Value(6)])]));

    packets
}


#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day13.txt");

    #[test]
    fn part1_ex() {
        assert_eq!(13, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(140, part2(INPUT));
    }
}
