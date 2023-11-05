use itertools::all;
use md5::{self, Digest};

const PASSWORD_LENGTH: usize = 8;

pub fn part1(input: &str) -> String {
    let mut index = 0;
    let mut password = String::new();

    loop {
        let digest = hash_value(input, index);

        if let Some((c, _)) = test_hash(&digest) {
            password.push(c);
            if password.len() == PASSWORD_LENGTH {
                break;
            }
        }

        index += 1;
    }

    password
}

pub fn part2(input: &str) -> String {
    let mut index = 0;
    let mut password = [' '; PASSWORD_LENGTH];
    let mut index_set = [false; PASSWORD_LENGTH];

    loop {
        let digest = hash_value(input, index);

        if let Some((pos, c)) = test_hash(&digest) {
            let pos = pos.to_digit(16).unwrap() as usize;

            if (0..PASSWORD_LENGTH).contains(&pos) && !index_set[pos] {
                password[pos] = c;
                index_set[pos] = true;
            }

            if all(index_set, |b| b) {
                break;
            }
        }

        index += 1;
    }

    password.iter().collect()
}

fn hash_value(id: &str, index: u32) -> Digest {
    let to_hash = format!("{id}{index}");
    md5::compute(to_hash)
}

fn test_hash(digest: &Digest) -> Option<(char, char)> {
    let digest = format!("{:x}", digest);
    if !&digest.starts_with("00000") {
        return None;
    }

    let mut chars = digest.chars().skip(5);
    Some((chars.next().unwrap(), chars.next().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_extraction() {
        let result = test_hash(&hash_value("abc", 3231929));
        assert!(result.is_some());
        assert_eq!(('1', '5'), result.unwrap());
    }
}
