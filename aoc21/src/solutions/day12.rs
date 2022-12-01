use std::collections::{HashMap, HashSet};

pub fn part1(input: &str) -> i32 {
    let map = parse(input);
    count_solutions(&map, "start", HashSet::new(), false)
}

pub fn part2(input: &str) -> i32 {
    let map = parse(input);
    count_solutions(&map, "start", HashSet::new(), true)
}

fn count_solutions<'a>(
    map: &HashMap<&str, HashSet<&str>>,
    current: &'a str,
    mut visited: HashSet<&'a str>,
    small_twice: bool,
) -> i32 {
    if current == "end" {
        return 1;
    }

    visited.insert(current);

    map.get(current)
        .unwrap()
        .iter()
        .map(|cave| {
            if util::is_lowercase(cave) {
                if !visited.contains(cave) {
                    count_solutions(map, cave, visited.clone(), small_twice)
                } else if small_twice && cave.len() < 3 {
                    count_solutions(map, cave, visited.clone(), false)
                } else {
                    0
                }
            } else {
                count_solutions(map, cave, visited.clone(), small_twice)
            }
        })
        .sum()
}

fn parse(input: &str) -> HashMap<&str, HashSet<&str>> {
    let mut map: HashMap<&str, HashSet<&str>> = HashMap::new();

    for line in input.lines() {
        let (a, b) = line.split_once('-').unwrap();
        map.entry(a).or_default().insert(b);
        map.entry(b).or_default().insert(a);
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_ex() {
        let input1 = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let input2 = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        let input3 = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

        assert_eq!(part1(input1), 10);
        assert_eq!(part1(input2), 19);
        assert_eq!(part1(input3), 226);
    }

    #[test]
    fn part2_ex() {
        let input1 = "\
start-A
start-b
A-c
A-b
b-d
A-end
b-end";

        let input2 = "\
dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc";

        let input3 = "\
fs-end
he-DX
fs-he
start-DX
pj-DX
end-zg
zg-sl
zg-pj
pj-he
RW-he
fs-DX
pj-RW
zg-RW
start-pj
he-WI
zg-he
pj-fs
start-RW";

        assert_eq!(part2(input1), 36);
        assert_eq!(part2(input2), 103);
        assert_eq!(part2(input3), 3509);
    }
}
