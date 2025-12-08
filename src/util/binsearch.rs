pub fn binsearch_leftmost(n: usize, f: impl Fn(usize) -> bool) -> usize {
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    l
}

pub fn binsearch_leftmost_exponential(f: impl Fn(usize) -> bool) -> usize {
    let mut n = 1;
    while !f(n) {
        n *= 2;
    }

    binsearch_leftmost(n, f)
}

pub fn binsearch_rightmost(n: usize, f: impl Fn(usize) -> bool) -> usize {
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = l + (r - l) / 2;
        if f(m) {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r - 1
}

pub fn binsearch_rightmost_exponential(f: impl Fn(usize) -> bool) -> usize {
    let mut n = 1;
    while !f(n) {
        n *= 2;
    }

    binsearch_rightmost(n, f)
}
