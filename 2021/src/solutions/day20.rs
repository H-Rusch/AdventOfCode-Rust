use std::collections::VecDeque;

pub fn part1(input: &str) -> usize {
    let (algorithm, mut grid) = parse(input);
    enhance(algorithm, &mut grid, 2);

    count_pixels(&grid)
}

pub fn part2(input: &str) -> usize {
    let (algorithm, mut grid) = parse(input);
    enhance(algorithm, &mut grid, 50);

    count_pixels(&grid)
}

fn enhance(algorithm: Vec<bool>, grid: &mut VecDeque<VecDeque<bool>>, iterations: usize) {
    for i in 0..iterations {
        let background = algorithm[0] && i % 2 == 1;
        pad(grid, background);

        let old: VecDeque<VecDeque<bool>> = grid.clone();

        for y in 1..grid.len() - 1 {
            for x in 1..grid[0].len() - 1 {
                let index = get_binary(x, y, &old, background);
                grid[y][x] = algorithm[index];
            }
        }
        trim_outer(grid);
    }
}

fn pad(grid: &mut VecDeque<VecDeque<bool>>, background: bool) {
    for _ in 0..3 {
        for row in grid.iter_mut() {
            row.push_front(background);
            row.push_back(background);
        }
        let to_add = VecDeque::from(vec![background; grid[0].len()]);
        grid.push_front(to_add.clone());
        grid.push_back(to_add.clone());
    }
}

fn trim_outer(grid: &mut VecDeque<VecDeque<bool>>) {
    grid.pop_back();
    grid.pop_front();
    for row in grid.iter_mut() {
        row.pop_back();
        row.pop_front();
    }
}

fn get_binary(x: usize, y: usize, grid: &VecDeque<VecDeque<bool>>, background: bool) -> usize {
    let bin_string = get_surrounding(x, y)
        .iter()
        .map(|&(dx, dy)| {
            if dx == 0 || dy == 0 || dx > grid[0].len() + 1 || dy > grid.len() + 1 {
                if background {
                    '1'
                } else {
                    '0'
                }
            } else if grid[dy][dx] {
                '1'
            } else {
                '0'
            }
        })
        .collect::<String>();
    usize::from_str_radix(&bin_string, 2).unwrap()
}

fn get_surrounding(x: usize, y: usize) -> [(usize, usize); 9] {
    [
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
}

fn count_pixels(grid: &VecDeque<VecDeque<bool>>) -> usize {
    grid.iter().flatten().filter(|b| **b).count()
}

fn parse(input: &str) -> (Vec<bool>, VecDeque<VecDeque<bool>>) {
    let (algorithm, img) = input.split_once("\n\n").unwrap();

    let algorithm = algorithm.chars().map(|c| c == '#').collect();

    let grid = img
        .lines()
        .map(|line| line.chars().map(|c| c == '#').collect())
        .collect();

    (algorithm, grid)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "\
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        assert_eq!(part1(input), 35);
    }

    #[test]
    fn part2_ex() {
        let input = "\
        ..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

        assert_eq!(part2(input), 3351);
    }
}
