use std::{collections::{HashSet, HashMap}, iter::{Peekable, Cycle, Enumerate}, str::Chars};

struct Block {
    x: usize, // left edge
    y: usize, // bottom edge
    width: usize,
    height: usize,
    settled: bool,
    tiles: Vec<(usize, usize)>,
}

impl Block {
    fn new(x: usize, y: usize, block_type: usize) -> Block {
        let settled = false;
        match block_type {
            0 => Block {
                x,
                y,
                settled,
                width: 4,
                height: 1,
                tiles: Vec::from([(x, y), (x + 1, y), (x + 2, y), (x + 3, y)]),
            },
            1 => Block {
                x,
                y,
                settled,
                width: 3,
                height: 3,
                tiles: Vec::from([
                    (x + 1, y),
                    (x, y + 1),
                    (x + 1, y + 1),
                    (x + 2, y + 1),
                    (x + 1, y + 2),
                ]),
            },
            2 => Block {
                x,
                y,
                settled,
                width: 3,
                height: 3,
                tiles: Vec::from([
                    (x, y),
                    (x + 1, y),
                    (x + 2, y),
                    (x + 2, y + 1),
                    (x + 2, y + 2),
                ]),
            },
            3 => Block {
                x,
                y,
                settled,
                width: 1,
                height: 4,
                tiles: Vec::from([(x, y), (x, y + 1), (x, y + 2), (x, y + 3)]),
            },
            4 => Block {
                x,
                y,
                settled,
                width: 2,
                height: 2,
                tiles: Vec::from([(x, y), (x, y + 1), (x + 1, y), (x + 1, y + 1)]),
            },
            _ => unreachable!(),
        }
    }

    fn move_left(&mut self, solid_tiles: &HashSet<(usize, usize)>) {
        // cannot move left on the left edge
        if self.x == 0 {
            return;
        }

        let moved: Vec<(usize, usize)> = self.tiles.iter().map(|(x, y)| (x - 1, *y)).collect();
        if !moved.iter().any(|coordinate| solid_tiles.contains(coordinate)) {
            self.tiles = moved;
            self.x -= 1;
        }
    }

    fn move_right(&mut self, solid_tiles: &HashSet<(usize, usize)>) {
        // cannot move right on the right edge
        if self.x + self.width == WIDTH {
            return;
        }

        let moved: Vec<(usize, usize)> = self.tiles.iter().map(|(x, y)| (x + 1, *y)).collect();
        if !moved.iter().any(|coordinate| solid_tiles.contains(coordinate)) {
            self.tiles = moved;
            self.x += 1;
        }
    }

    fn move_down(&mut self, solid_tiles: &HashSet<(usize, usize)>) {
        if self.y == 0 {
            self.settled = true;
            return;
        }

        let moved: Vec<(usize, usize)> = self.tiles.iter().map(|(x, y)| (*x, y - 1)).collect();
        if moved .iter().any(|coordinate| solid_tiles.contains(coordinate)) {
            self.settled = true;
        } else {
            self.tiles = moved;
            self.y -= 1;
        }
    }
}

const BLOCK_COUNT: usize = 5;
const WIDTH: usize = 7;

pub fn part1(input: &str) -> usize {
    play_tetris(input, 2022)
}

pub fn part2(input: &str) -> usize {
    play_tetris(input, 1_000_000_000_000)
}

fn play_tetris(input: &str, limit: usize) -> usize {
    let mut directions = parse(input);

    let mut tiles: HashSet<(usize, usize)> = HashSet::new();
    let mut max_height = 0;
    let mut start_height = 3;

    // (wind index, block index, [number of tiles down from the highest one; 7])  
    let mut cache: HashMap<(usize, usize, [usize; 7]), (usize, usize)> = HashMap::new();
    for i in 0..limit {
        let block_type = i % BLOCK_COUNT;
        let wind_index = directions.peek().unwrap().0;
        
        let state = (block_type, wind_index, get_top_depths(&tiles, max_height));

        if let Some((old_i, old_max)) = cache.insert(state, (i, max_height)) {
            let period = i - old_i;
            
            // no extra iterations
            if i % period == limit % period {
                let amount_in_period = max_height - old_max;
                let remaining = limit - i;

                return old_max + amount_in_period * ((remaining / period) + 1)
            }
        }

        let mut block = Block::new(2, start_height, block_type);
        while !block.settled {
            match directions.next().unwrap().1 {
                '<' => block.move_left(&tiles),
                '>' => block.move_right(&tiles),
                _ => unreachable!(),
            }

            block.move_down(&tiles);
        }

        tiles.extend(block.tiles.iter());

        max_height = max_height.max(block.y + block.height);
        start_height = max_height + 3;
    }

    max_height
}

fn get_top_depths(tiles: &HashSet<(usize, usize)>, max_height: usize) -> [usize; WIDTH] {
    let mut vals = [max_height; WIDTH];

    for (x, item) in vals.iter_mut().enumerate().take(WIDTH) {
        let mut y = max_height;
        while y > 0 {
            if tiles.contains(&(x, y)) {
                *item = max_height - y;
                break;
            }
            y -= 1;
        }
    }

    vals
}

#[allow(unused)]
fn print_tetris(tiles: &HashSet<(usize, usize)>) {
    println!("--- Printing Board ----");
    let max_y = tiles.iter().max_by_key(|c| c.1).unwrap_or(&(0, 0)).1;

    let mut vals = String::new();
    for y in 0..=max_y {
        for x in 0..WIDTH {
            let v = if tiles.contains(&(x, y)) { "# " } else { ". " };
            vals.push_str(v);
        }
        vals.push('\n');
    }

    for line in vals.lines().rev() {
        println!("{:?}", line);
    }
    println!()
}

fn parse(input: &str) -> Peekable<Cycle<Enumerate<Chars>>> {
    input.trim().chars().enumerate().cycle().peekable()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";

    #[test]
    fn part1_ex() {
        assert_eq!(3068, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(1514285714288, part2(INPUT));
    }
}
