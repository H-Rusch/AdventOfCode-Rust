use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use md5::{self};

const KEY_COUNT: usize = 64;
const STRETCHED_ITERATIONS: usize = 2016;
const KEY_VERIFICATION_ITERATIONS: usize = 1000;


struct CachedHashCalculator {
    cache: HashMap<String, String>,
    hash_algorithm: Box<dyn Hashing>,
}

impl CachedHashCalculator {
    fn new(hash_algorithm: impl Hashing + 'static) -> Self {
        CachedHashCalculator {
            cache: HashMap::new(),
            hash_algorithm: Box::new(hash_algorithm),
        }
    }

    fn get_hash(&mut self, to_hash: &str) -> String {
        if !self.cache.contains_key(to_hash) {
            let result = self.hash_algorithm.hash(to_hash);
            self.cache.insert(to_hash.to_string(), result);
        }

        self.cache.get(to_hash).unwrap().to_string()
    }
}

struct BasicHash;
struct StretchedHash;

trait Hashing {
    fn hash(&self, to_hash: &str) -> String;
}

impl Hashing for BasicHash {
    fn hash(&self, to_hash: &str) -> String {
        let digest = md5::compute(to_hash);
        format!("{:x}", digest)
    }
}

impl Hashing for StretchedHash {
    fn hash(&self, to_hash: &str) -> String {
        let mut digest = md5::compute(to_hash);
        for _ in 0..STRETCHED_ITERATIONS {
            digest = md5::compute(format!("{:x}", digest));
        }
        format!("{:x}", digest)
    }
}

pub fn part1(input: &str) -> usize {
    let hasher = CachedHashCalculator::new(BasicHash);
    generate_keys(input, hasher)
}

pub fn part2(input: &str) -> usize {
    let hasher = CachedHashCalculator::new(StretchedHash);
    generate_keys(input, hasher)
}

fn generate_keys(salt: &str, mut hasher: CachedHashCalculator) -> usize {
    let mut keys_generated = 0;
    let mut index = 0;

    while keys_generated < KEY_COUNT {
        let to_hash = build_string_to_hash(salt, index);
        let hashed = hasher.get_hash(&to_hash);

        if is_valid_key(&hashed, salt, index, &mut hasher) {
            keys_generated += 1;
        }

        index += 1;
    }

    index - 1
}

fn build_string_to_hash(salt: &str, index: usize) -> String {
    format!("{salt}{index}")
}

fn is_valid_key(hashed: &str, salt: &str, index: usize, hasher: &mut CachedHashCalculator) -> bool {
    if let Some(ch) = find_triple(hashed) {
        for i in 1..KEY_VERIFICATION_ITERATIONS {
            let to_hash = build_string_to_hash(salt, index + i);
            let hashed = hasher.get_hash(&to_hash);
            if find_quintuple(&hashed, ch) {
                return true;
            }
        }
    }

    false
}

fn find_triple(hashed: &str) -> Option<char> {
    for (c1, c2, c3) in hashed.chars().tuple_windows() {
        if HashSet::<char>::from([c1, c2, c3]).len() == 1 {
            return Some(c1);
        }
    }

    None
}

fn find_quintuple(hashed: &str, ch: char) -> bool {
    for (c1, c2, c3, c4, c5) in hashed.chars().tuple_windows() {
        if c1 == ch && HashSet::<char>::from([c1, c2, c3, c4, c5]).len() == 1 {
            return true;
        }
    }

    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_hashing_test() {
        let hasher = BasicHash;

        assert_eq!(
            "577571be4de9dcce85a041ba0410f29f".to_string(),
            hasher.hash("abc0")
        );
        assert_eq!(
            "0034e0923cc38887a57bd7b1d4f953df".to_string(),
            hasher.hash("abc18")
        );
    }

    #[test]
    fn stretched_hashing_test() {
        let hasher = StretchedHash;
        let to_hash = "abc0";

        let result = hasher.hash(to_hash);

        assert_eq!("a107ff634856bb300138cac6568c0f24".to_string(), result);
    }

    #[test]
    fn find_triple_test() {
        assert!(find_triple("577571be4de9dcce85a041ba0410f29f").is_none());
        assert!(find_triple("0034e0923cc38887a57bd7b1d4f953df").is_some());
        assert_eq!('8', find_triple("0034e0923cc38887a57bd7b1d4f953df").unwrap());
    }

    #[test]
    fn find_quintuple_test() {
        assert!(!find_quintuple("577571be4de9dcce85a041ba0410f29f", '8'));
        assert!(find_quintuple("3aeeeee1367614f3061d165a5fe3cac3", 'e'));
    }
}
