use crate::util;

fn num_digits(i: u64) -> u32 {
    i.ilog10() + 1
}

#[inline]
fn first_half(i: u64, n: u32) -> u64 {
    i / 10_u64.pow(n / 2)
}

#[inline]
fn last_half(i: u64, n: u32) -> u64 {
    i % 10_u64.pow(n / 2)
}

fn is_valid1(i: u64) -> bool {
    let n = num_digits(i);
    if !n.is_multiple_of(2) {
        return true;
    }

    first_half(i, n) != last_half(i, n)
}

fn digit_string(mut i: u64, buf: &mut Vec<u8>) {
    buf.clear();
    let n = num_digits(i) as usize;
    buf.resize(n, 0);

    for k in (0..n).rev() {
        buf[k] = (i % 10) as u8;
        i /= 10;
    }
}

fn is_valid2(s: &[u8]) -> bool {
    for n in 1..=s.len() / 2 {
        let mut chunks = s.chunks_exact(n);
        if !chunks.remainder().is_empty() {
            continue;
        }

        let Some(first) = chunks.next() else {
            continue;
        };

        if chunks.all(|c| c == first) {
            return false;
        }
    }
    true
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut total: u64 = 0;
    for range in input.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }
        let (a, b) = range.split_once('-').unwrap();
        let a = a.parse::<u64>().unwrap();
        let b = b.parse::<u64>().unwrap();

        for i in a..=b {
            if !is_valid1(i) {
                total += i;
            }
        }
    }

    println!("{total}")
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut total: u64 = 0;
    let mut scratch = vec![];
    for range in input.split(',') {
        let range = range.trim();
        if range.is_empty() {
            continue;
        }
        let (a, b) = range.split_once('-').unwrap();
        let a = a.parse::<u64>().unwrap();
        let b = b.parse::<u64>().unwrap();

        for i in a..=b {
            digit_string(i, &mut scratch);
            if !is_valid2(&scratch) {
                total += i;
            }
        }
    }

    println!("{total}")
}
