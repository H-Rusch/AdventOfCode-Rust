use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
enum Operation {
    Rectangle(usize, usize),
    RotateRow(usize, usize),
    RotateColumn(usize, usize),
}

impl Operation {
    fn from(line: &str) -> Self {
        if let Some(capture) = RECT_REGEX.captures(line) {
            let x = capture.get(1).unwrap().as_str().parse().unwrap();
            let y = capture.get(2).unwrap().as_str().parse().unwrap();

            return Operation::Rectangle(x, y);
        }
        if let Some(capture) = ROTATE_REGEX.captures(line) {
            let n = capture.get(2).unwrap().as_str().parse().unwrap();
            let amount = capture.get(3).unwrap().as_str().parse().unwrap();

            return match capture.get(1).unwrap().as_str() {
                "row y" => Operation::RotateRow(n, amount),
                "column x" => Operation::RotateColumn(n, amount),
                _ => unreachable!(),
            };
        }
        unreachable!();
    }

    fn execute(&self, display: &mut [[bool; WIDTH]; HEIGHT]) {
        match *self {
            Operation::Rectangle(x, y) => rectangle(display, x, y),
            Operation::RotateColumn(column, amount) => rotate_column(display, column, amount),
            Operation::RotateRow(row, amount) => rotate_row(display, row, amount),
        }
    }
}

const HEIGHT: usize = 6;
const WIDTH: usize = 50;

lazy_static! {
    static ref RECT_REGEX: Regex = Regex::new(r"^rect (\d+)x(\d+)$").unwrap();
    static ref ROTATE_REGEX: Regex =
        Regex::new(r"^rotate (row y|column x)=(\d+) by (\d+)$").unwrap();
}

pub fn part1(input: &str) -> usize {
    let display = start_display(input);

    display.iter().flatten().filter(|b| **b).count()
}

pub fn part2(input: &str) -> String {
    let display = start_display(input);

    let mut display_string = String::new();
    for row in display {
        display_string.push('\n');
        for b in row {
            match b {
                true => display_string.push('▮'),
                false => display_string.push(' '),
            }
        }
    }

    display_string
}

fn start_display(input: &str) -> [[bool; WIDTH]; HEIGHT] {
    let mut display = build_display();
    parse(input).iter().for_each(|op| op.execute(&mut display));

    display
}

fn rectangle(display: &mut [[bool; WIDTH]; HEIGHT], x: usize, y: usize) {
    for row in display.iter_mut().take(y) {
        for cell in row.iter_mut().take(x) {
            *cell = true;
        }
    }
}

fn rotate_column(display: &mut [[bool; WIDTH]; HEIGHT], column: usize, amount: usize) {
    let old_column: Vec<bool> = (0..HEIGHT).map(|y| display[y][column]).collect();

    for (y, row) in display.iter_mut().enumerate() {
        row[column] = old_column[(y + HEIGHT - amount) % HEIGHT];
    }
}

fn rotate_row(display: &mut [[bool; WIDTH]; HEIGHT], row: usize, amount: usize) {
    let row = &mut display[row];
    let old_row = *row;

    for (i, cell) in row.iter_mut().enumerate() {
        *cell = old_row[(i + WIDTH - amount) % WIDTH];
    }
}

fn build_display() -> [[bool; WIDTH]; HEIGHT] {
    [[false; 50]; 6]
}

fn parse(input: &str) -> Vec<Operation> {
    input.lines().map(Operation::from).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rectangle_test() {
        let op = Operation::from("rect 3x2");
        let mut display = build_display();

        op.execute(&mut display);

        assert!(display[0][0]);
        assert!(display[0][2]);
        assert!(display[1][0]);
        assert!(display[1][2]);
        assert!(!display[2][0]);
    }

    #[test]
    fn rotate_row_test() {
        let op = Operation::from("rotate row y=0 by 1");
        let mut display = build_display();
        display[0][WIDTH - 1] = true;
        display[0][4] = true;

        op.execute(&mut display);

        assert!(!display[0][WIDTH - 1]);
        assert!(display[0][0]);
        assert!(!display[0][4]);
        assert!(display[0][5]);
    }

    #[test]
    fn rotate_column_test() {
        let op = Operation::from("rotate column x=0 by 1");
        let mut display = build_display();
        display[HEIGHT - 1][0] = true;
        display[1][0] = true;

        op.execute(&mut display);

        assert!(!display[HEIGHT - 1][0]);
        assert!(display[0][0]);
        assert!(!display[1][0]);
        assert!(display[2][0]);
    }
}
