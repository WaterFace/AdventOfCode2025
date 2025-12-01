use crate::util;

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut dial = 50;
    let mut count = 0;
    for line in input.lines() {
        let (dir, n) = line.split_at(1);
        let n: i32 = n.parse().unwrap();

        match dir {
            "L" => dial -= n,
            "R" => dial += n,
            _ => panic!("unexpected input: {line}"),
        }
        dial %= 100;
        if dial == 0 {
            count += 1;
        }
    }

    println!("{count}");
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let mut dial: i32 = 50;
    let mut count = 0;
    for line in input.lines() {
        let (dir, n) = line.split_at(1);
        let n: i32 = n.parse().unwrap();

        let rot = match dir {
            "L" => -n,
            "R" => n,
            _ => panic!("unexpected input: {line}"),
        };

        let mut full_rots = i32::abs(n / 100);
        let rem = rot % 100;

        if (dial > 0 && dial + rem <= 0) || dial + rem >= 100 {
            full_rots += 1;
        }
        count += full_rots;

        dial = (dial + rot).rem_euclid(100);
    }

    println!("{count}");
}
