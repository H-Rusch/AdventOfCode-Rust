use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;

lazy_static! {
    static ref ROOM_REGEX: Regex = Regex::new(r"^(.*)-(\d+)\[(.*)\]$").unwrap();
}

#[derive(Debug)]
struct Room {
    encrypted_name: String,
    id: u32,
    checksum: String,
}

impl Room {
    fn from(line: &str) -> Self {
        let captures = ROOM_REGEX.captures_iter(line).next().unwrap();
        Room {
            encrypted_name: captures[1].to_string(),
            id: captures[2].parse().unwrap(),
            checksum: captures[3].to_string(),
        }
    }
}

pub fn part1(input: &str) -> u32 {
    parse(input)
        .iter()
        .filter(|&room| check_room_real(room))
        .map(|room| room.id)
        .sum()
}

pub fn part2(input: &str) -> u32 {
    let search_for = "northpole object storage".to_string().to_ascii_lowercase();

    parse(input)
        .iter()
        .filter(|&room| check_room_real(room))
        .find(|&room| shift_encrypted(room) == search_for)
        .unwrap()
        .id
}

fn check_room_real(room: &Room) -> bool {
    let combined_chars: Vec<char> = room
        .encrypted_name
        .split('-')
        .flat_map(|s| s.chars())
        .collect();
    let unique_chars = combined_chars.iter().unique();
    let char_counts = combined_chars.iter().counts();
    let expected_checksum: String = unique_chars
        .sorted_by(|a, b| {
            (-(*char_counts.get(a).unwrap() as i32), a)
                .cmp(&(-(*char_counts.get(b).unwrap() as i32), b))
        })
        .take(5)
        .collect();

    expected_checksum == room.checksum
}

fn shift_encrypted(room: &Room) -> String {
    room.encrypted_name
        .chars()
        .map(|c| shift_char(c, room.id))
        .collect()
}

fn shift_char(c: char, shift: u32) -> char {
    if c == '-' {
        return ' ';
    }
    let shift = shift % 26;

    let ord = c as u32 - 'a' as u32;
    char::from_u32((ord + shift) % 26 + 'a' as u32).unwrap()
}

fn parse(input: &str) -> Vec<Room> {
    input.lines().map(Room::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../../examples/day04.txt");

    #[test]
    fn room_real_check_works() {
        assert!(check_room_real(&Room::from("aaaaa-bbb-z-y-x-123[abxyz]")));
        assert!(check_room_real(&Room::from("a-b-c-d-e-f-g-h-987[abcde]")));
        assert!(check_room_real(&Room::from("not-a-real-room-404[oarel]")));
        assert!(!check_room_real(&Room::from(
            "totally-real-room-200[decoy]"
        )));
    }

    #[test]
    fn part1_ex() {
        assert_eq!(1514, part1(INPUT));
    }

    #[test]
    fn shift_char_test() {
        assert_eq!('b', shift_char('a', 1));
        assert_eq!('a', shift_char('z', 1));
    }

    #[test]
    fn room_decryption() {
        let room = Room::from("qzmt-zixmtkozy-ivhz-343[abcde]");
        assert_eq!("very encrypted name".to_string(), shift_encrypted(&room));
    }
}
