use std::collections::{HashSet, VecDeque};

use crate::util::{self, Grid};

#[derive(PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Tile {
    Empty,
    Full,
}

struct Shape {
    grids: Vec<Grid<Tile>>,
    area: usize,
}

impl Shape {
    fn parse_shape(s: &str) -> Shape {
        let mut area = 0;
        let grid = Grid::from_str(s.trim(), |c| match c {
            '#' => {
                area += 1;
                Tile::Full
            }
            '.' => Tile::Empty,
            _ => panic!("unexpected input: {c}"),
        })
        .unwrap();
        let mut grids = HashSet::new();
        let mut grid = grid.clone();
        for _ in 0..4 {
            grids.insert(grid.clone());
            grid = rotate_grid(&grid);
        }
        grid = flip_grid(&grid);
        for _ in 0..4 {
            grids.insert(grid.clone());
            grid = rotate_grid(&grid);
        }

        Shape {
            grids: grids.into_iter().collect(),
            area,
        }
    }

    fn parse_shapes(s: &str) -> Vec<Shape> {
        let mut shapes = vec![];
        for entry in s.split("\n\n") {
            let (a, b) = entry.split_once(":").unwrap();
            if a.len() > 1 {
                break;
            }

            let shape = Shape::parse_shape(b);
            shapes.push(shape);
        }
        shapes
    }
}

fn rotate_grid<T: Clone>(grid: &Grid<T>) -> Grid<T> {
    assert!(grid.width == 3 && grid.height == 3);

    let &[a, b, c, d, e, f, g, h, i] = &grid.data.as_slice() else {
        unreachable!()
    };

    let new_data = [g, d, a, h, e, b, i, f, c].map(|t| t.clone());

    Grid::from_slice(&new_data, 3, 3).unwrap()
}

fn flip_grid<T: Clone>(grid: &Grid<T>) -> Grid<T> {
    assert!(grid.width == 3 && grid.height == 3);

    let &[a, b, c, d, e, f, g, h, i] = &grid.data.as_slice() else {
        unreachable!()
    };

    let new_data = [c, b, a, f, e, d, i, h, g].map(|t| t.clone());

    Grid::from_slice(&new_data, 3, 3).unwrap()
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let shapes = Shape::parse_shapes(&input);
    let mut regions = vec![];
    let remainder = input.split("\n\n").nth(shapes.len()).unwrap();
    for region in remainder.lines() {
        let (size, counts) = region.split_once(": ").unwrap();
        let (x, y) = size.split_once("x").unwrap();
        let (x, y) = (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap());
        let counts = util::parse_ints(counts)
            .unwrap()
            .iter()
            .map(|&i| i as usize)
            .collect::<Vec<_>>();

        regions.push((x, y, counts));
    }

    let mut n = 0;
    for (x, y, counts) in regions.iter() {
        let total_area = x * y;
        let mut area_used = 0;
        for i in 0..counts.len() {
            area_used += counts[i] * shapes[i].area;
        }

        if area_used > total_area {
            continue;
        }

        // stupid
        n += 1;

        // let mut queue = VecDeque::from([(Grid::from_scalar(Tile::Empty, *x, *y), counts.clone())]);
        // let mut explored = HashSet::new();
        // while let Some((grid, mut counts)) = queue.pop_front() {
        //     // println!("{:?}", counts);
        //     // grid.pretty_print(|t| match t {
        //     //     Tile::Empty => ".",
        //     //     Tile::Full => "#",
        //     // });
        //     // println!();
        //     if explored.contains(&(grid.clone(), counts.clone())) {
        //         continue;
        //     } else {
        //         explored.insert((grid.clone(), counts.clone()));
        //     }
        //     if counts.iter().all(|i| *i == 0) {
        //         n += 1;
        //         println!("{n}");
        //         continue 'outer;
        //     }

        //     let mut shape_idx = None;
        //     for (i, count) in counts.iter_mut().enumerate() {
        //         if *count > 0 {
        //             shape_idx = Some(i);
        //             *count -= 1;
        //             break;
        //         }
        //     }
        //     let Some(shape_idx) = shape_idx else {
        //         unreachable!()
        //     };

        //     for ((x, y), _) in grid.iter() {
        //         'a: for shape in &shapes[shape_idx].grids {
        //             for ((dx, dy), t) in shape.iter() {
        //                 if matches!(t, Tile::Full)
        //                     && !matches!(grid.get(x + dx, y + dy), Some(Tile::Empty))
        //                 {
        //                     continue 'a;
        //                 }
        //             }

        //             // found a spot
        //             let mut new_grid = grid.clone();
        //             for ((dx, dy), t) in shape.iter() {
        //                 let Some(new_t) = new_grid.get_mut(x + dx, y + dy) else {
        //                     unreachable!()
        //                 };
        //                 *new_t = *t;
        //             }
        //             queue.push_back((new_grid, counts.clone()));
        //         }
        //     }
        // }
    }

    println!("{n}");
}

pub fn part2(_input: &str) {
    println!("Part 2 unimplemented.");
}
