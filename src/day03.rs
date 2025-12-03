use crate::util;

fn bank_max_n(bank: &[u64], n: usize) -> u64 {
    let mut table = vec![vec![0; bank.len()]; n + 1];

    for k in 1..=n {
        for i in (0..bank.len()).rev() {
            if k > bank.len() - i {
                continue;
            }
            if k == 1 {
                if i == bank.len() - 1 {
                    table[k][i] = bank[i];
                } else {
                    table[k][i] = u64::max(bank[i], table[k][i + 1]);
                }
                continue;
            }

            table[k][i] = u64::max(
                bank[i] * 10u64.pow(k as u32 - 1) + table[k - 1][i + 1],
                table[k][i + 1],
            );
        }
    }

    table[n][0]
}

fn parse_bank(s: &str) -> impl Iterator<Item = u64> {
    s.bytes().map(|c| (c - b'0') as u64)
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut scratch = vec![];
    let mut total = 0;
    for bank in input.lines() {
        scratch.clear();
        scratch.extend(parse_bank(bank));
        total += bank_max_n(&scratch, 2);
    }
    println!("{total}")
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut scratch = vec![];
    let mut total = 0;
    for bank in input.lines() {
        scratch.clear();
        scratch.extend(parse_bank(bank));
        total += bank_max_n(&scratch, 12);
    }
    println!("{total}")
}
