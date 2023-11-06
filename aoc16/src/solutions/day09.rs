pub fn part1(input: &str) -> usize {
    input.lines().map(decompress).map(|s| s.len()).sum()
}

pub fn part2(input: &str) -> usize {
    input.lines().map(fully_decompressed_length).sum()
}

fn decompress(compressed: &str) -> String {
    let mut decompressed = String::new();
    let mut chars = compressed.chars();

    while let Some(ch) = chars.next() {
        if ch == '(' {
            let (amount, times) = extract_marker(&mut chars);
            decompressed.push_str(&repeat(&mut chars, amount, times));
        } else {
            decompressed.push(ch);
        }
    }

    decompressed
}

fn extract_marker(chars: &mut impl Iterator<Item = char>) -> (usize, usize) {
    let mut marker = String::new();

    for ch in chars.by_ref() {
        if ch == ')' {
            break;
        }
        marker.push(ch);
    }

    let (amount, times) = marker.split_once('x').unwrap();
    (amount.parse().unwrap(), times.parse().unwrap())
}

fn repeat(chars: &mut impl Iterator<Item = char>, amount: usize, times: usize) -> String {
    chars.take(amount).collect::<String>().repeat(times)
}

fn fully_decompressed_length(line: &str) -> usize {
    decrompressed_length_helper(&mut line.chars(), 1)
}

fn decrompressed_length_helper(
    mut section: &mut impl Iterator<Item = char>,
    multiplier: usize,
) -> usize {
    let mut result = 0;

    while let Some(ch) = section.next() {
        if ch == '(' {
            let (amount, times) = extract_marker(&mut section);

            let taken = section.take(amount).collect::<String>();
            result += decrompressed_length_helper(&mut taken.chars(), multiplier * times);
        } else {
            result += multiplier;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_marker_test() {
        assert_eq!((1, 5), extract_marker(&mut "1x5)BC".chars()));
    }

    #[test]
    fn decrompress_test() {
        assert_eq!("ADVENT".to_string(), decompress("ADVENT"));
        assert_eq!("ABBBBBC".to_string(), decompress("A(1x5)BC"));
        assert_eq!("XYZXYZXYZ".to_string(), decompress("(3x3)XYZ"));
        assert_eq!("ABCBCDEFEFG".to_string(), decompress("A(2x2)BCD(2x2)EFG"));
        assert_eq!("(1x3)A".to_string(), decompress("(6x1)(1x3)A"));
        assert_eq!(
            "X(3x3)ABC(3x3)ABCY".to_string(),
            decompress("X(8x2)(3x3)ABCY")
        );
    }

    #[test]
    fn fully_decompressed_length_test() {
        assert_eq!(9, fully_decompressed_length("(3x3)XYZ"));
        assert_eq!(20, fully_decompressed_length("X(8x2)(3x3)ABCY"));
        assert_eq!(
            241920,
            fully_decompressed_length("(27x12)(20x12)(13x14)(7x10)(1x12)A")
        );
        assert_eq!(
            445,
            fully_decompressed_length("(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN")
        );
    }
}
