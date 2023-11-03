enum Payload {
    Operator(Vec<Packet>),
    Literal(u64),
}

struct Packet {
    version: u32,
    type_id: u32,
    payload: Payload,
}

impl Packet {
    fn sum_versions(&self) -> u32 {
        match &self.payload {
            Payload::Literal(_) => self.version,
            Payload::Operator(v) => {
                self.version + v.iter().map(|p| p.sum_versions()).sum::<u32>()
            }
        }
    }

    fn calculate(&self) -> u64 {
        match &self.payload {
            Payload::Literal(n) => *n,
            Payload::Operator(v) => calc_subpackets(self.type_id, v),
        }
    } 
}

fn calc_subpackets(type_id: u32, subpackets: &[Packet]) -> u64 {
    match type_id {
        0 => subpackets.iter().map(|p| p.calculate()).sum(),
        1 => subpackets.iter().map(|p| p.calculate()).product(),
        2 => subpackets.iter().map(|p| p.calculate()).min().unwrap(),
        3 => subpackets.iter().map(|p| p.calculate()).max().unwrap(),
        5 => (subpackets[0].calculate() > subpackets[1].calculate()) as u64,
        6 => (subpackets[0].calculate() < subpackets[1].calculate()) as u64,
        7 => (subpackets[0].calculate() == subpackets[1].calculate()) as u64,
        _ => unreachable!()
    }
} 

pub fn part1(input: &str) -> u32 {
    let bits = parse(input);
    let packet = parse_packet(&mut bits.into_iter());

    packet.sum_versions()
}

pub fn part2(input: &str) -> u64 {
    let bits = parse(input);
    let packet = parse_packet(&mut bits.into_iter());

    packet.calculate()
}

fn parse_packet(bits: &mut dyn Iterator<Item = u8>) -> Packet {
    let version = read_num(&mut bits.take(3)) as u32;
    let type_id = read_num(&mut bits.take(3)) as u32;

    match type_id {
        4 => Packet {
            version,
            type_id,
            payload: Payload::Literal(parse_literal(bits)),
        },
        _ => Packet {
            version,
            type_id,
            payload: Payload::Operator(parse_subpackets(bits)),
        },
    }
}

fn parse_literal(bits: &mut dyn Iterator<Item = u8>) -> u64 {
    let mut n = 0;
    loop {
        let v = bits.next().unwrap();
        n = (n << 4) + read_num(&mut bits.take(4));

        if v != 1 {
            break;
        }
    }
    n
}

fn parse_subpackets(bits: &mut dyn Iterator<Item = u8>) -> Vec<Packet> {
    let length_id = bits.next().unwrap();

    match length_id {
        0 => {
            let size = read_num(&mut bits.take(15)) as usize;
            subpackets_in_size(bits, size)
        }
        1 => {
            let num = read_num(&mut bits.take(11)) as usize;
            subpackets_by_num(bits, num)
        }
        _ => unreachable!(),
    }
}

fn subpackets_in_size(bits: &mut dyn Iterator<Item = u8>, size: usize) -> Vec<Packet> {
    let mut window = bits.take(size).peekable();
    let mut subpackets: Vec<Packet> = Vec::new();

    while window.peek().is_some() {
        subpackets.push(parse_packet(&mut window));
    }

    subpackets
}

fn subpackets_by_num(bits: &mut dyn Iterator<Item = u8>, num: usize) -> Vec<Packet> {
    let mut subpackets: Vec<Packet> = Vec::new();

    for _ in 0..num {
        subpackets.push(parse_packet(bits));
    }

    subpackets
}

fn read_num(bits: &mut dyn Iterator<Item = u8>) -> u64 {
    bits.fold(0, |n, b| (n << 1) + b as u64)
}

fn parse(input: &str) -> Vec<u8> {
    // convert hex-input into series of bits
    input
        .trim()
        .chars()
        .flat_map(|c| {
            let d = c.to_digit(16).unwrap() as u8;
            (0..4).rev().map(move |i| (d >> i) & 0b1)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_bin_correctly() {
        assert_eq!(read_num(&mut [1, 0, 0].into_iter().take(3)), 4);
        assert_eq!(
            read_num(&mut [1, 0, 0, 0, 0, 0, 0, 1, 0, 0].into_iter().take(10)),
            516
        );
        assert_eq!(
            read_num(
                &mut [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]
                    .into_iter()
                    .take(17)
            ),
            131071
        );
    }

    #[test]
    fn parse_literal_correct_value() {
        let mut literal = [1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 0, 0, 1, 0, 1, 0, 0, 0]
            .into_iter()
            .take(18);

        assert_eq!(parse_literal(&mut literal), 2021);
    }

    #[test]
    fn parse_operator_size() {
        let mut iterator = [
            0, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 1, 1, 1, 0, 1, 0, 0, 0,
            1, 0, 1, 0, 0, 1, 0, 1, 0, 0, 1, 0, 0, 0, 1, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0,
        ]
        .into_iter()
        .take(56);
        let payload = parse_packet(&mut iterator).payload;
        let v = match &payload {
            Payload::Operator(v) => v,
            _ => unreachable!(),
        };
        matches!(payload, Payload::Operator(_));
        assert_eq!(v.len(), 2);
        matches!(v[0].payload, Payload::Literal(_));
        matches!(v[1].payload, Payload::Literal(_));
    }

    #[test]
    fn parse_operator_num() {
        let mut iterator = [
            1, 1, 1, 0, 1, 1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 0, 1, 0, 1, 0, 0, 0, 0, 0, 0, 1,
            1, 0, 0, 1, 0, 0, 0, 0, 0, 1, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0, 1, 1, 0, 0, 0, 0, 0,
        ]
        .into_iter()
        .take(56);
        let payload = parse_packet(&mut iterator).payload;
        let v = match &payload {
            Payload::Operator(v) => v,
            _ => unreachable!(),
        };
        matches!(payload, Payload::Operator(_));
        assert_eq!(v.len(), 3);
        matches!(v[0].payload, Payload::Literal(_));
        matches!(v[1].payload, Payload::Literal(_));
        matches!(v[2].payload, Payload::Literal(_));
    }

    #[test]
    fn part1_ex() {
        for (packet, result) in [
            ("8A004A801A8002F478", 16),
            ("620080001611562C8802118E34", 12),
            ("C0015000016115A2E0802F182340", 23),
            ("A0016C880162017C3686B18A3D4780", 31),
        ] {
            assert_eq!(part1(packet), result);
        }
    }

    #[test]
    fn part2_ex() {
        for (packet, result) in [
            ("C200B40A82", 3),
            ("04005AC33890", 54),
            ("880086C3E88112", 7),
            ("CE00C43D881120", 9),
            ("D8005AC2A8F0", 1),
            ("F600BC2D8F", 0),
            ("9C005AC2F8F0", 0),
            ("9C0141080250320F1802104A08", 1),
            ("38006F45291200", 1),
        ] {
            assert_eq!(part2(packet), result);
        }
    }
}
