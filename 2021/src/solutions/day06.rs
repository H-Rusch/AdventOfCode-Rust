use std::collections::LinkedList;

pub fn part1(input: &str) -> u64 {
    let arr = parse(input);
    day_step(arr, 80)
}

pub fn part2(input: &str) -> u64 {
    let arr = parse(input); 
    day_step(arr, 256)
}

fn day_step(arr: [u64; 9], num: usize) -> u64 {
    let mut fishes = LinkedList::from(arr);

    for _ in 0..num {
        let front = fishes.pop_front().unwrap();
        *fishes.iter_mut().nth(6).unwrap() += front;
        fishes.push_back(front);
    }
    
    sum_fish(&fishes)
}

fn sum_fish(list: &LinkedList<u64>) -> u64 {
    list.iter().map(|n| *n).sum()
}

fn parse(input: &str) -> [u64; 9] {
    let mut arr = [0; 9];
        
    for n in input.trim().split(",") {
        arr[n.parse::<usize>().unwrap()] += 1;
    }

    arr
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "3,4,3,1,2";

        assert_eq!(part1(input), 5934);
    }

    #[test]
    fn part2_ex() {
        let input = "3,4,3,1,2";

        assert_eq!(part2(input), 26984457539);
    }
}