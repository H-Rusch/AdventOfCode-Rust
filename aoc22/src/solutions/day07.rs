use std::{cell::RefCell, collections::VecDeque, rc::Rc};

#[derive(Debug)]
struct Node {
    _name: String,
    size: u32,
    children: Vec<Rc<RefCell<Node>>>,
}

impl Node {
    fn new(name: String, size: u32) -> Node {
        Node {
            _name: name,
            size,
            children: vec![],
        }
    }

    fn calc_size(&self, current_size: u32) -> u32 {
        let size = current_size + self.size;
        let children_size = self
            .children
            .iter()
            .map(|c| c.borrow().calc_size(size))
            .sum::<u32>();

        self.size + children_size
    }

    fn update_sizes(&mut self) {
        self.size = self.calc_size(0);

        for child in &self.children {
            child.borrow_mut().update_sizes();
        }
    }
}

pub fn part1(input: &str) -> u32 {
    let root = Rc::clone(&parse(input).borrow().children[0]);
    let dirs = flattened_dirs(Rc::clone(&root));

    dirs.iter()
        .filter(|dir| dir.borrow().size <= 100_000)
        .map(|dir| dir.borrow().size)
        .sum()
}

const TOTAL_SPACE: u32 = 70_000_000;
const SPACE_NEEDED: u32 = 30_000_000;

pub fn part2(input: &str) -> u32 {
    let root = Rc::clone(&parse(input).borrow().children[0]);
    let dirs = flattened_dirs(Rc::clone(&root));

    let unused_space = TOTAL_SPACE - root.borrow().size;

    let mut sizes: Vec<u32> = dirs.iter().map(|dir| dir.borrow().size).collect();
    sizes.sort();
    *sizes
        .iter()
        .find(|&&size| size + unused_space >= SPACE_NEEDED)
        .unwrap()
}

fn flattened_dirs(root: Rc<RefCell<Node>>) -> Vec<Rc<RefCell<Node>>> {
    // visit all nodes and collect the directories into a Vec
    let mut directories: Vec<Rc<RefCell<Node>>> = Vec::new();
    let mut queue: VecDeque<Rc<RefCell<Node>>> = VecDeque::from([root]);

    while let Some(node) = queue.pop_front() {
        if !node.borrow().children.is_empty() {
            directories.push(Rc::clone(&node));
        }

        queue.extend(node.borrow().children.iter().map(Rc::clone));
    }

    directories
}

fn parse(input: &str) -> Rc<RefCell<Node>> {
    let parent = Rc::new(RefCell::new(Node::new("".to_string(), 0)));
    let mut current = Rc::clone(&parent);
    let mut parents: Vec<Rc<RefCell<Node>>> = Vec::from([Rc::clone(&parent)]);

    for line in input.lines() {
        let contents: Vec<&str> = line.split_whitespace().collect();

        if contents[0] == "$" {
            if contents[1] == "cd" {
                if contents[2] == ".." {
                    current = parents.pop().unwrap();
                    continue;
                }
                parents.push(Rc::clone(&current));
                let name = contents[2].to_string();
                let child = Rc::new(RefCell::new(Node::new(name, 0)));
                current.borrow_mut().children.push(Rc::clone(&child));
                current = child;
            }
            // ignore $ ls
        } else if let Ok(n) = contents[0].parse::<u32>() {
            let name = contents[1].to_string();
            current
                .borrow_mut()
                .children
                .push(Rc::new(RefCell::new(Node::new(name, n))));
        }
        // ignore dir <xyz>
    }

    parent.borrow_mut().update_sizes();
    parent
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "\
$ cd /
$ ls
dir a
14848514 b.txt
8504156 c.dat
dir d
$ cd a
$ ls
dir e
29116 f
2557 g
62596 h.lst
$ cd e
$ ls
584 i
$ cd ..
$ cd ..
$ cd d
$ ls
4060174 j
8033020 d.log
5626152 d.ext
7214296 k";

    #[test]
    fn part1_ex() {
        assert_eq!(95437, part1(INPUT));
    }

    #[test]
    fn part2_ex() {
        assert_eq!(24933642, part2(INPUT));
    }
}
