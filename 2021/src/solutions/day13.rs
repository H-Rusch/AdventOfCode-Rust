use regex::Regex;
use std::collections::HashSet;

#[derive(Debug)]
struct Fold<'a> {
    axis: &'a str,
    num: u32,
}

pub fn part1(input: &str) -> usize {
    let (points, folds) = parse(input);
    let points = fold_points(points, &folds[0]);

    points.len()
}

pub fn part2(input: &str) -> String {
    let (mut points, folds) = parse(input);
    for fold in folds {
        points = fold_points(points, &fold);
    }

    string_to_print(points)
}

fn string_to_print(points: HashSet<(u32, u32)>) -> String {
    let mut s = String::new();
    for y in 0..=6 {
        s.push('\n');
        for x in 0..=38 {
            if points.contains(&(x, y)) {
                s.push_str("■ ");
            } else {
                s.push_str("  ");
            }
        }
    }

    s
}

fn fold_points(points: HashSet<(u32, u32)>, fold: &Fold) -> HashSet<(u32, u32)> {
    points.iter().map(|coord| do_fold(coord, fold)).collect()
}

fn do_fold(coord: &(u32, u32), fold: &Fold) -> (u32, u32) {
    let mut x = coord.0;
    let mut y = coord.1;

    if fold.axis == "y" && y > fold.num {
        y = fold.num - (fold.num.abs_diff(y));
    } else if fold.axis == "x" && x > fold.num {
        x = fold.num - (fold.num.abs_diff(x));
    }

    (x, y)
}

fn parse(input: &str) -> (HashSet<(u32, u32)>, Vec<Fold>) {
    let (points, instructions) = input.split_once("\n\n").unwrap();

    let points: HashSet<(u32, u32)> = points
        .lines()
        .map(|line| line.split_once(',').unwrap())
        .map(|(x, y)| (x.parse::<u32>().unwrap(), y.parse::<u32>().unwrap()))
        .collect();

    let fold_regex = Regex::new(r"fold along (y|x)=(\d+)").unwrap();
    let instructions: Vec<Fold> = instructions
        .lines()
        .map(|line| fold_regex.captures(line).unwrap())
        .map(|caps| {
            (
                caps.get(1).unwrap().as_str(),
                caps[2].parse::<u32>().unwrap(),
            )
        })
        .map(|(axis, num)| Fold { axis, num })
        .collect();

    (points, instructions)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn coordinate_fold() {
        let c = (12, 6);
        let fold1 = Fold { axis: "x", num: 14 };
        let fold2 = Fold { axis: "x", num: 8 };

        assert_eq!(do_fold(&c, &fold1), (12, 6));
        assert_eq!(do_fold(&c, &fold2), (4, 6));
    }

    #[test]
    fn part1_ex() {
        let input = "\
6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5";

        assert_eq!(part1(input), 17);
    }
}
