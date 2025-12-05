pub mod dir;
pub mod ext;
pub mod grid;
pub mod parse_ints;
pub mod sparse_grid;
pub mod stats;
pub mod vec2;
pub mod vec3;

pub use dir::Dir;
pub use ext::*;
pub use grid::Grid;
pub use parse_ints::*;
pub use sparse_grid::SparseGrid;
pub use vec2::{Vec2, vec2};
pub use vec3::{Vec3, vec3};

pub fn read_file(filename: &str) -> Result<String, std::io::Error> {
    std::fs::read_to_string(filename)
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
