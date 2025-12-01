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

    let mut dial = 50;
    let mut count = 0;
    for line in input.lines() {
        let (dir, n) = line.split_at(1);
        let n: i32 = n.parse().unwrap();

        let d = match dir {
            "L" => -1,
            "R" => 1,
            _ => panic!("unexpected input: {line}"),
        };

        for _ in 0..n {
            dial += d;
            if dial % 100 == 0 {
                count += 1;
            }
        }
        dial %= 100;
    }

    println!("{count}");
}
