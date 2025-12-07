use std::collections::{HashMap, VecDeque};

use crate::util::{self, Grid, Vec2, vec2};

enum Tile {
    Start,
    Empty,
    Splitter,
    Beam,
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut grid = util::Grid::from_str(&input, |c| match c {
        '.' => Tile::Empty,
        'S' => Tile::Start,
        '^' => Tile::Splitter,
        _ => panic!("unexpected input: {c}"),
    })
    .unwrap();

    let start_pos = grid
        .iter()
        .find(|(_, t)| matches!(t, Tile::Start))
        .map(|(p, _)| p)
        .expect("No start position");

    let mut num_splits = 0;
    let mut stack = vec![start_pos];
    while let Some(p) = stack.pop() {
        let mut p: util::Vec2 = p.into();

        loop {
            {
                let h = grid.height as i32;
                let w = grid.width as i32;
                if p.y < 0 || h <= p.y || p.x < 0 || w <= p.x {
                    break;
                }
            }

            match grid[p] {
                Tile::Beam => break,
                Tile::Empty => {
                    grid[p] = Tile::Beam;
                }
                Tile::Start => {
                    // do nothing
                }
                Tile::Splitter => {
                    num_splits += 1;
                    stack.push((p + vec2(-1, 0)).into());
                    stack.push((p + vec2(1, 0)).into());
                    break;
                }
            }
            p.y += 1;
        }
    }
    grid.pretty_print(|t| match t {
        Tile::Beam => "|",
        Tile::Empty => " ",
        Tile::Splitter => "^",
        Tile::Start => "S",
    });
    println!("{num_splits}");
}

fn paths_from(grid: &Grid<Tile>, pos: Vec2, memo: &mut HashMap<Vec2, usize>) -> usize {
    if let Some(n) = memo.get(&pos) {
        return *n;
    }

    let mut p = pos;
    loop {
        {
            let h = grid.height as i32;
            let w = grid.width as i32;
            if p.y < 0 || h <= p.y || p.x < 0 || w <= p.x {
                memo.insert(pos, 1);
                return 1;
            }
        }

        match grid[p] {
            Tile::Beam => {
                unreachable!()
            }
            Tile::Empty | Tile::Start => {
                // do nothing
            }
            Tile::Splitter => {
                let mut n = 0;
                n += paths_from(grid, p + vec2(-1, 0), memo);
                n += paths_from(grid, p + vec2(1, 0), memo);
                memo.insert(pos, n);
                return n;
            }
        }
        p.y += 1;
    }
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let grid = util::Grid::from_str(&input, |c| match c {
        '.' => Tile::Empty,
        'S' => Tile::Start,
        '^' => Tile::Splitter,
        _ => panic!("unexpected input: {c}"),
    })
    .unwrap();

    let start_pos = grid
        .iter()
        .find(|(_, t)| matches!(t, Tile::Start))
        .map(|(p, _)| p)
        .expect("No start position");

    let mut memo = HashMap::new();
    let num_paths = paths_from(&grid, start_pos.into(), &mut memo);
    println!("{num_paths}");
}
