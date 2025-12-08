use std::{
    cmp::{self, Reverse},
    collections::{BinaryHeap, HashMap, HashSet, VecDeque},
    fmt::Display,
};

use crate::util::{self, Vec3, ext::SortedExt};

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
struct JunctionBox {
    pos: Vec3,
}

impl JunctionBox {
    fn distance_to(self, other: Self) -> u64 {
        (other.pos - self.pos).sqr_length_u64()
    }
}

impl Display for JunctionBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{},{},{}", self.pos.x, self.pos.y, self.pos.z)
    }
}

fn connected_components(
    vertices: &[JunctionBox],
    distances: &[(u64, JunctionBox, JunctionBox)],
    n: usize,
) -> Vec<usize> {
    let mut adjacency: HashMap<JunctionBox, Vec<JunctionBox>> = HashMap::new();

    for (_d, a, b) in distances.iter().take(n) {
        adjacency.entry(*a).or_default().push(*b);
        adjacency.entry(*b).or_default().push(*a);
    }

    // bfs over nodes in this subgraph to find connected components
    let mut explored_components: HashSet<JunctionBox> = HashSet::new();
    let mut explored: HashSet<JunctionBox> = HashSet::new();
    let mut queue = VecDeque::new();

    let mut component_sizes = BinaryHeap::new();

    for v in vertices.iter().copied() {
        if explored_components.contains(&v) {
            continue;
        } else {
            explored_components.insert(v);
        }
        queue.clear();
        queue.push_back(v);
        explored.clear();
        while let Some(v) = queue.pop_front() {
            if explored.contains(&v) {
                continue;
            }
            explored.insert(v);

            for w in adjacency.entry(v).or_default() {
                queue.push_back(*w);
            }
        }
        explored_components.extend(&explored);
        component_sizes.push(explored.len());
    }

    component_sizes.into_iter().collect()
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let junctions: Vec<_> = input
        .lines()
        .map(|line| {
            let ints = util::parse_ints(line).unwrap();
            assert!(ints.len() == 3);
            JunctionBox {
                pos: Vec3::new(ints[0], ints[1], ints[2]),
            }
        })
        .collect();

    let mut distances: Vec<(JunctionBox, JunctionBox, u64)> = Vec::new();
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            let a = junctions[i];
            let b = junctions[j];
            let (a, b) = (cmp::min(a, b), cmp::max(a, b));

            distances.push((a, b, a.distance_to(b)));
        }
    }

    let distances = distances
        .into_iter()
        .map(|(a, b, v)| (v, a, b))
        .collect::<Vec<_>>()
        .sorted();

    let component_sizes =
        connected_components(&junctions, &distances, 1000).sorted_by_key(|t| Reverse(*t));
    println!("{}", component_sizes.into_iter().take(3).product::<usize>());
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let junctions: Vec<_> = input
        .lines()
        .map(|line| {
            let ints = util::parse_ints(line).unwrap();
            assert!(ints.len() == 3);
            JunctionBox {
                pos: Vec3::new(ints[0], ints[1], ints[2]),
            }
        })
        .collect();

    let mut distances: Vec<(JunctionBox, JunctionBox, u64)> = Vec::new();
    for i in 0..junctions.len() {
        for j in i + 1..junctions.len() {
            let a = junctions[i];
            let b = junctions[j];
            let (a, b) = (cmp::min(a, b), cmp::max(a, b));

            distances.push((a, b, a.distance_to(b)));
        }
    }

    let distances = distances
        .into_iter()
        .map(|(a, b, v)| (v, a, b))
        .collect::<Vec<_>>()
        .sorted();

    let i = util::binsearch_leftmost_exponential(|i| {
        let component_sizes = connected_components(&junctions, &distances, i + 1);
        component_sizes.len() == 1
    });

    let (_, a, b) = distances[i];
    // println!("{a} <-> {b}: {}", a.pos.x as u64 * b.pos.x as u64);
    println!("{}", a.pos.x as u64 * b.pos.x as u64);
}
