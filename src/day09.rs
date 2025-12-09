use crate::util::{self, Vec2, vec2};

fn area(a: Vec2, b: Vec2) -> u64 {
    let w = a.x.abs_diff(b.x) as u64 + 1;
    let h = a.y.abs_diff(b.y) as u64 + 1;
    w * h
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut tiles = Vec::new();
    for line in input.lines() {
        let ints = util::parse_ints(line).unwrap();
        assert!(ints.len() == 2);
        let (x, y) = (ints[0], ints[1]);
        tiles.push(vec2(x, y));
    }

    let mut max_area = 0;

    for i in 0..tiles.len() {
        for j in i + 1..tiles.len() {
            let a = tiles[i];
            let b = tiles[j];

            let area = area(a, b);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{max_area}");
}

fn intersects_with_interior(p1: Vec2, p2: Vec2, polygon: &[Vec2]) -> bool {
    let top_left = vec2(p1.x.min(p2.x), p1.y.min(p2.y));
    let bottom_right = vec2(p1.x.max(p2.x), p1.y.max(p2.y));
    for window in polygon.windows(2) {
        assert!(window.len() == 2);
        let [v, u] = window else { unreachable!() };

        if v.x == u.x {
            // vertical line
            let (line_min_y, line_max_y) = (v.y.min(u.y), v.y.max(u.y));

            if (top_left.x < v.x && v.x < bottom_right.x)
                && !(line_max_y <= top_left.y || bottom_right.y <= line_min_y)
            {
                return true;
            }
        } else if v.y == u.y {
            // horizontal line

            let (line_min_x, line_max_x) = (v.x.min(u.x), v.x.max(u.x));

            if (top_left.y < v.y && v.y < bottom_right.y)
                && !(line_max_x <= top_left.x || bottom_right.x <= line_min_x)
            {
                return true;
            }
        } else {
            panic!("non-axis aligned edge");
        }
    }

    false
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut polygon = Vec::new();
    let mut first = None;
    for line in input.lines() {
        let ints = util::parse_ints(line).unwrap();
        assert!(ints.len() == 2);
        let (x, y) = (ints[0], ints[1]);
        let p = vec2(x, y);
        polygon.push(p);
        if first.is_none() {
            first = Some(p);
        }
    }
    if let Some(first) = first {
        polygon.push(first);
    }

    let mut max_area = 0;
    for i in 0..polygon.len() {
        for j in i + 1..polygon.len() {
            let a = polygon[i];
            let b = polygon[j];

            if intersects_with_interior(a, b, &polygon) {
                continue;
            }

            let area = area(a, b);
            if area > max_area {
                max_area = area;
            }
        }
    }

    println!("{max_area}");
}
