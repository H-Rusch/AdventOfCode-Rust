use std::collections::VecDeque;

pub fn part1(input: &str) -> usize {
    let amount = input.parse().unwrap();
    let mut elves = build_deque(amount, 0);

    while elves.len() > 1 {
        // elf taking presents is added to the back of the list
        elves.rotate_left(1);
        // elf which has their presents taken is removed from the list
        elves.pop_front();
    }

    *elves.front().unwrap()
}

pub fn part2(input: &str) -> usize {
    let amount = input.parse().unwrap();
    // do not track the middle element, but divide the elements in two different lists
    let mut left = build_deque(amount / 2, 0);
    let mut right = build_deque(amount, amount / 2);

    while left.len() + right.len() > 1 {
        let current = left.pop_front().unwrap();

        if left.len() == right.len() {
            left.pop_back();
        } else {
            right.pop_front();
        }

        right.push_back(current);
        // move the first right value to the back of the left values, as we are moving to the right
        left.push_back(right.pop_front().unwrap());
    }

    *left.front().unwrap()
}

fn build_deque(amount: usize, starting_from: usize) -> VecDeque<usize> {
    VecDeque::from_iter((starting_from..amount).map(|v| v + 1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "5";

    #[test]
    fn part1_ex() {
        assert_eq!(3, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(2, part2(INPUT));
    }
}
