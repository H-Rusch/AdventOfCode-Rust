use std::collections::HashSet;

pub fn part1(input: &str) -> usize {
    let mut head_pos = (0, 0);
    let mut tail_pos = head_pos;

    let instructions = parse(input);
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::from([tail_pos]); 

    for (direction, num) in instructions {
        for _ in 0..num {
            change_head_position(&mut head_pos, direction);
            change_knot_position(&head_pos, &mut tail_pos);
            tail_positions.insert(tail_pos);
        }
    }

    tail_positions.len()
}

pub fn part2(input: &str) -> usize {
    let mut knot_positions = [(0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0), (0, 0)];

    let instructions = parse(input);
    let mut tail_positions: HashSet<(i32, i32)> = HashSet::from([knot_positions[9]]); 

    for (direction, num) in instructions {
        for _ in 0..num {
            change_head_position(&mut knot_positions[0], direction);
            for i in 1..knot_positions.len() {
                let leading_knot = knot_positions[i - 1];
                change_knot_position(&leading_knot, &mut knot_positions[i]);
            }
            tail_positions.insert(knot_positions[9]);
        }
    }

    tail_positions.len()
}

fn change_head_position(head_pos: &mut (i32, i32), direction: char) {
    match direction {
        'R' => {head_pos.0 += 1},
        'L' => {head_pos.0 -= 1},
        'U' => {head_pos.1 += 1},
        'D' => {head_pos.1 -= 1},
        _ => unreachable!(),
    };
} 

fn change_knot_position(head_pos: &(i32, i32), tail_pos: &mut (i32, i32)) {
    // tail is adjacent or on the same tile
    if head_pos.0.abs_diff(tail_pos.0) <= 1 && head_pos.1.abs_diff(tail_pos.1) <= 1 {
        return;
    }

    let change_x = (head_pos.0 - tail_pos.0).signum();
    let change_y = (head_pos.1 - tail_pos.1).signum();

    *tail_pos = (tail_pos.0 + change_x, tail_pos.1 + change_y);
}



fn parse(input: &str) -> Vec<(char, i32)> {
    input.lines()
        .map(|line| line.split_once(' ').unwrap())
        .map(|(dir, num)| (dir.parse().unwrap(), num.parse().unwrap()))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

    const INPUT2: &str = "\
R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20";

    #[test]
    fn part1_ex() {
        assert_eq!(13, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(1, part2(INPUT));
        assert_eq!(36, part2(INPUT2));
    }
}
