use crate::util;
use util::grid;

#[derive(Debug, PartialEq, Eq)]
enum Tile {
    Paper,
    Empty,
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let grid = grid::Grid::from_str(&input, |c| match c {
        '.' => Tile::Empty,
        '@' => Tile::Paper,
        _ => panic!("Unexpected input: {c}"),
    })
    .unwrap();

    let mut count = 0;
    for ((x, y), t) in grid.iter() {
        if matches!(t, Tile::Empty) {
            continue;
        }

        let n = grid
            .neighbors(x, y, false)
            .filter(|(_, t)| matches!(t, Tile::Paper))
            .count();
        if n < 4 {
            count += 1;
        }
    }

    println!("{count}");
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut grid = grid::Grid::from_str(&input, |c| match c {
        '.' => Tile::Empty,
        '@' => Tile::Paper,
        _ => panic!("Unexpected input: {c}"),
    })
    .unwrap();

    let mut to_be_removed = vec![];
    let mut count = 0;
    loop {
        for ((x, y), t) in grid.iter() {
            if matches!(t, Tile::Empty) {
                continue;
            }

            let n = grid
                .neighbors(x, y, false)
                .filter(|(_, t)| matches!(t, Tile::Paper))
                .count();
            if n < 4 {
                to_be_removed.push((x, y));
            }
        }

        if to_be_removed.is_empty() {
            break;
        }
        count += to_be_removed.len();

        for (x, y) in to_be_removed.drain(..) {
            grid[(x, y)] = Tile::Empty;
        }
    }

    println!("{count}");
}
