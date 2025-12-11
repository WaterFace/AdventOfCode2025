use std::collections::HashMap;

use crate::util;

fn paths_from<'a>(
    source: &'a str,
    target: &'a str,
    adjacency: &HashMap<&'a str, Vec<&'a str>>,
    memo: &mut HashMap<&'a str, u64>,
) -> u64 {
    if let Some(n) = memo.get(source) {
        return *n;
    }

    if source == target {
        return 1;
    }

    let mut n = 0;
    let Some(neighbors) = adjacency.get(source) else {
        return 0;
    };
    for v in neighbors {
        n += paths_from(v, target, adjacency, memo);
    }
    memo.insert(source, n);

    n
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (node, neighbors) = line.split_once(": ").unwrap();
        let next = adjacency.entry(node).or_default();
        for n in neighbors.split(" ") {
            next.push(n);
        }
    }

    let mut memo = HashMap::new();
    let n = paths_from("you", "out", &adjacency, &mut memo);
    println!("{n}");
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut adjacency: HashMap<&str, Vec<&str>> = HashMap::new();
    for line in input.lines() {
        let (node, neighbors) = line.split_once(": ").unwrap();
        let next = adjacency.entry(node).or_default();
        for n in neighbors.split(" ") {
            next.push(n);
        }
    }

    let mut n = 0;
    n += paths_from("svr", "dac", &adjacency, &mut HashMap::new())
        * paths_from("dac", "fft", &adjacency, &mut HashMap::new())
        * paths_from("fft", "out", &adjacency, &mut HashMap::new());

    n += paths_from("svr", "fft", &adjacency, &mut HashMap::new())
        * paths_from("fft", "dac", &adjacency, &mut HashMap::new())
        * paths_from("dac", "out", &adjacency, &mut HashMap::new());
    println!("{n}");
}
