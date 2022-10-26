#[allow(unused)]
#[derive(Debug)]
struct Bingo {
    numbers: Vec<i32>,
    boards: Vec<Board>,
}

impl Bingo {
    fn play_game(&mut self, first: bool) -> Result<i32, ()> {
        let board_count = self.boards.len();
        let mut won = 0;

        for n in self.numbers.iter() {
            for board in self.boards.iter_mut() {
                if board.guess(n) && board.check_finished() {
                    won += 1;
                    if first || won == board_count {
                        return Ok(board.calc_score(n));
                    }
                }
            }
            self.boards.retain(|b| !b.check_finished());
        }

        Err(())
    }
}

#[derive(Debug)]
struct Board {
    grid: [[i32; 5]; 5],
    guessed: [[bool; 5]; 5],
}

impl Board {
    fn new(grid: [[i32; 5]; 5]) -> Board {
        Board {
            grid,
            guessed: [[false; 5]; 5],
        }
    }

    // check if num in grid. Mark the entry if guessed correctly
    fn guess(&mut self, num: &i32) -> bool {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, cell) in row.iter().enumerate() {
                if cell == num {
                    self.guessed[y][x] = true;
                    return true;
                }
            }
        }

        false
    }

    fn check_finished(&self) -> bool {
        // check rows
        let option = self.guessed.iter()
            .find(|r| r.iter().all(|b| *b));

        if let Some(_) = option {
            return true;
        } else {
            // check columns
            for x in 0..self.guessed.len() {
                let result = self.guessed.iter()
                    .map(|r| r[x])
                    .all(|b| b);
                if result {
                    return true;
                }
            }
        }

        false
    }

    fn calc_score(&self, num: &i32) -> i32 {
        num * self.sum_unmarked()
    }

    fn sum_unmarked(&self) -> i32 {
        self.grid
            .iter()
            .enumerate()
            .map(|(y, r)| {
                r.iter()
                    .enumerate()
                    .map(|(x, n)| if !self.guessed[y][x] { n } else { &0 })
                    .sum::<i32>()
            })
            .sum::<i32>()
    }
}

pub fn part1(input: &str) -> i32 {
    let mut bingo = parse(input);
    bingo.play_game(true).unwrap()
}

pub fn part2(input: &str) -> i32 {
    let mut bingo = parse(input);
    bingo.play_game(false).unwrap()
}

fn parse(input: &str) -> Bingo {
    let input = input.replace("\r", ""); // remove windows carriage return for files read in from windows
    let mut input = input.split("\n\n");

    let numbers: Vec<i32> = input
        .next()
        .unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect();

    let boards: Vec<Board> = input
        .map(|chunk| {
            let mut grid = [[0; 5]; 5];
            for (y, line) in chunk.lines().enumerate() {
                for (x, num) in line.split_whitespace().enumerate() {
                    grid[y][x] = num.parse().unwrap();
                }
            }
            Board::new(grid)
        })
        .collect();

    Bingo { numbers, boards }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(part1(input), 4512);

    }

    #[test]
    fn sum_all_numbers_if_none_marked() {
        let input = "\
1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";

        let bingo = parse(input);
        
        assert_eq!(bingo.boards[0].sum_unmarked(), 300);
    }

    #[test]
    fn sum_only_unmarked_numbers() {
        let input = "\
1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19";

        let mut bingo = parse(input);
        bingo.boards[0].guess(&23);
        bingo.boards[0].guess(&16);
        bingo.boards[0].guess(&6);
        bingo.boards[0].guess(&5);
        
        assert_eq!(bingo.boards[0].sum_unmarked(), 250);
    }

    #[test]
    fn part2_ex() {
        let input = "\
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7";

        assert_eq!(part2(input), 1924);

    }

}
