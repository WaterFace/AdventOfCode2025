pub mod dir;
pub mod ext;
pub mod grid;
pub mod sparse_grid;
pub mod stats;
pub mod vec2;
pub mod vec3;

pub use dir::Dir;
pub use ext::*;
pub use grid::Grid;
pub use sparse_grid::SparseGrid;
pub use vec2::{vec2, Vec2};
pub use vec3::{vec3, Vec3};

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
}

pub fn parse_ints_u64(s: &str) -> Result<Vec<u64>, std::num::ParseIntError> {
    parse_ints_radix_u64(s, 10)
}

pub fn parse_ints_radix_u64(s: &str, radix: u32) -> Result<Vec<u64>, std::num::ParseIntError> {
    let mut parsed = Vec::new();

    let mut char_indices = s.char_indices();

    let mut current = None;

    loop {
        let next = char_indices.next();
        if let Some((i, c)) = next {
            if c.is_digit(radix) {
                current = match current {
                    // We're currently in the middle of a digit string,
                    // so continue it
                    Some((start, _)) => Some((start, i)),
                    // We're not in the middle of a digit string, so start a new one
                    None => Some((i, i)),
                };
                continue;
            }
        }

        if let Some((start, end)) = current {
            let n_str = &s[start..=end];
            let n = u64::from_str_radix(n_str, radix)?;
            parsed.push(n);
            current = None;
        }

        if next.is_none() {
            break;
        }
    }

    Ok(parsed)
}

pub fn parse_ints_u32(s: &str) -> Result<Vec<u32>, std::num::ParseIntError> {
    parse_ints_radix_u32(s, 10)
}

pub fn parse_ints_radix_u32(s: &str, radix: u32) -> Result<Vec<u32>, std::num::ParseIntError> {
    let mut parsed = Vec::new();

    let mut char_indices = s.char_indices();

    let mut current = None;

    loop {
        let next = char_indices.next();
        if let Some((i, c)) = next {
            if c.is_digit(radix) {
                current = match current {
                    // We're currently in the middle of a digit string,
                    // so continue it
                    Some((start, _)) => Some((start, i)),
                    // We're not in the middle of a digit string, so start a new one
                    None => Some((i, i)),
                };
                continue;
            }
        }

        if let Some((start, end)) = current {
            let n_str = &s[start..=end];
            let n = u32::from_str_radix(n_str, radix)?;
            parsed.push(n);
            current = None;
        }

        if next.is_none() {
            break;
        }
    }

    Ok(parsed)
}

pub fn parse_ints(s: &str) -> Result<Vec<i32>, std::num::ParseIntError> {
    parse_ints_radix(s, 10)
}

pub fn parse_ints_radix(s: &str, radix: u32) -> Result<Vec<i32>, std::num::ParseIntError> {
    let mut parsed = Vec::new();

    let mut char_indices = s.char_indices();

    let mut current = None;

    loop {
        let next = char_indices.next();
        if let Some((i, c)) = next {
            if c.is_digit(radix) {
                current = match current {
                    // We're currently in the middle of a digit string,
                    // so continue it
                    Some((start, _)) => Some((start, i)),
                    // We're not in the middle of a digit string, so start a new one
                    None => Some((i, i)),
                };
                continue;
            } else if c == '-' {
                current = Some((i, i)); // Either way, we need to start a new string here
                continue;
            }
        }

        if let Some((start, end)) = current {
            let n_str = &s[start..=end];
            let n = i32::from_str_radix(n_str, radix)?;
            parsed.push(n);
            current = None;
        }

        if next.is_none() {
            break;
        }
    }

    Ok(parsed)
}

pub fn parse_ints_i64(s: &str) -> Result<Vec<i64>, std::num::ParseIntError> {
    parse_ints_radix_i64(s, 10)
}

pub fn parse_ints_radix_i64(s: &str, radix: u32) -> Result<Vec<i64>, std::num::ParseIntError> {
    let mut parsed = Vec::new();

    let mut char_indices = s.char_indices();

    let mut current = None;

    loop {
        let next = char_indices.next();
        if let Some((i, c)) = next {
            if c.is_digit(radix) {
                current = match current {
                    // We're currently in the middle of a digit string,
                    // so continue it
                    Some((start, _)) => Some((start, i)),
                    // We're not in the middle of a digit string, so start a new one
                    None => Some((i, i)),
                };
                continue;
            } else if c == '-' {
                current = Some((i, i)); // Either way, we need to start a new string here
                continue;
            }
        }

        if let Some((start, end)) = current {
            let n_str = &s[start..=end];
            let n = i64::from_str_radix(n_str, radix)?;
            parsed.push(n);
            current = None;
        }

        if next.is_none() {
            break;
        }
    }

    Ok(parsed)
}

pub fn gcd(a: i64, b: i64) -> i64 {
    // assert!(a > 0 && b > 0);
    let (a, b) = (a.abs(), b.abs());
    let (mut a, mut b) = (i64::max(a, b), i64::min(a, b));

    while b > 0 {
        let tmp = b;
        b = a % b;
        a = tmp;
    }

    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    a * b / gcd(a, b)
}

pub fn lcm3(a: i64, b: i64, c: i64) -> i64 {
    lcm(a, lcm(b, c))
}

/// returns `x` such that `a*x === 1 (mod m)`
pub fn modular_inverse(a: i64, m: i64) -> i64 {
    let (x, _) = bezout(a, m);
    x
}

/// returns the bezout coefficients (x, y) such that ax+by = gcd(a, b)
pub fn bezout(a: i64, b: i64) -> (i64, i64) {
    let mut old_r = a;
    let mut r = b;

    let mut old_s = 1;
    let mut s = 0;

    let mut old_t = 0;
    let mut t = 1;

    while r != 0 {
        let quotient = old_r / r;

        let tmp = r;
        r = old_r - quotient * r;
        old_r = tmp;

        let tmp = s;
        s = old_s - quotient * s;
        old_s = tmp;

        let tmp = t;
        t = old_t - quotient * t;
        old_t = tmp;
    }

    (old_s, old_t)
}
