use crate::util;

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut rows = vec![];
    let mut ops = vec![];
    for line in input.lines() {
        let line = line.trim();
        if line.starts_with('*') || line.starts_with('+') {
            ops.clear();
            ops.extend(line.split_ascii_whitespace());
        } else {
            rows.push(util::parse_ints_u64(line).unwrap());
        }
    }

    let mut acc: Vec<_> = ops
        .iter()
        .copied()
        .map(|s| match s {
            "*" => 1,
            "+" => 0,
            _ => panic!("unexpected input: {s}"),
        })
        .collect();

    for row in rows {
        for ((a, op), x) in acc
            .iter_mut()
            .zip(ops.iter().copied())
            .zip(row.iter().copied())
        {
            match op {
                "*" => *a *= x,
                "+" => *a += x,
                _ => panic!("unexpected input: {op}"),
            }
        }
    }

    println!("{}", acc.iter().sum::<u64>());
}

enum Tile {
    C(u8),
    Space,
}

enum Op {
    Plus,
    Times,
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let grid = util::Grid::from_str(&input, |c| match c {
        ' ' => Tile::Space,
        c => Tile::C(c as u8),
    })
    .unwrap();

    let mut problems: Vec<u64> = vec![];
    // points to the top-left character of a new problem
    let mut x: i32 = 0;
    loop {
        if x as usize >= grid.width {
            break;
        }

        //find the operation
        let (op, mut acc) = match grid[(x, grid.height as i32 - 1)] {
            Tile::C(b'*') => (Op::Times, 1),
            Tile::C(b'+') => (Op::Plus, 0),
            _ => panic!(),
        };

        loop {
            if x as usize >= grid.width {
                break;
            }

            let mut all_spaces = true;
            let mut operand = 0;
            for y in 0..grid.height - 1 {
                let y = y as i32;
                if let Tile::C(c) = grid[(x, y)] {
                    all_spaces = false;
                    let n = (c - b'0') as u64;
                    operand *= 10;
                    operand += n;
                }
            }

            x += 1;
            if all_spaces {
                break;
            } else {
                match op {
                    Op::Plus => acc += operand,
                    Op::Times => acc *= operand,
                }
            }
        }
        problems.push(acc);
    }

    println!("{}", problems.into_iter().sum::<u64>());
}
