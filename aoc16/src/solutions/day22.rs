use itertools::Itertools;
use regex::Regex;
use util::grid::Coordinate;

#[derive(PartialEq, Eq)]
struct Node {
    coordinate: Coordinate,
    size: usize,
    used: usize,
}

impl Node {
    fn new(x: i32, y: i32, size: usize, used: usize) -> Self {
        let coordinate = Coordinate::from(x, y);
        Node {
            coordinate,
            size,
            used,
        }
    }
}

pub fn part1(input: &str) -> usize {
    let nodes = parse(input);
    nodes
        .iter()
        .permutations(2)
        .filter(|permut| is_viable_pair(permut[0], permut[1]))
        .count()
}

pub fn part2(input: &str) -> &str {
    let nodes = parse(input);
    print_grid(&nodes);
    /*
    - 42 steps to go to G (top right corner):
      go left 10
      go up 6
      go right 26
    - G at (max_x - 1, 0) = (32, 0) 
      -> has to be moved 32 times to the left
    - going from the following configuration:

    . G _
    . . .

    to this configuration:

    G _ .
    . . .

    takes 5 steps
    - therefore moving G 32 steps to the left takes 32 * 5 steps which is 160 steps
    - those are added to the 42 initial steps: 
    42 steps + 160 steps = 202 steps
    => it takes 202 steps to move G to (0, 0)
    */

    "Solved the problem by hand after printing out my grid. Solution is included as a comment."
}

fn is_viable_pair(node_a: &Node, node_b: &Node) -> bool {
    node_a.used != 0 && node_a != node_b && node_b.size - node_b.used >= node_a.used
}

fn print_grid(nodes: &[Node]) {
    let max_x = nodes.iter().max_by_key(|node| node.coordinate.x).unwrap().coordinate.x;
    let max_y = nodes.iter().max_by_key(|node| node.coordinate.y).unwrap().coordinate.y;
    let mut output_string = String::new();

    for y in 0..=max_y {
        for x in 0..=max_x {
            let coordinate = Coordinate::from(x, y);
            let node = nodes
                .iter()
                .find(|node| node.coordinate == coordinate)
                .unwrap();

            if node.size >= 100 {
                output_string.push('#');
            } else if node.used == 0 {
                output_string.push('_');
            } else {
                output_string.push('.');
            }
            output_string.push(' ');
        }
        output_string.push('\n');
    }

    println!("{output_string}")
}

fn parse(input: &str) -> Vec<Node> {
    let node_pattern = Regex::new(r"^/dev/grid/node-x(\d+)-y(\d+)\s+(\d+)T\s+(\d+)T").unwrap();
    input
        .lines()
        .skip(2)
        .map(|line| {
            let captures = node_pattern.captures(line).unwrap();
            let x = captures.get(1).unwrap().as_str().parse().unwrap();
            let y = captures.get(2).unwrap().as_str().parse().unwrap();
            let size = captures.get(3).unwrap().as_str().parse().unwrap();
            let used = captures.get(4).unwrap().as_str().parse().unwrap();
            Node::new(x, y, size, used)
        })
        .collect()
}
