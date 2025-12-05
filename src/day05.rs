use crate::util;

struct Ingredients {
    ranges: Vec<(u64, u64)>,
}

impl Ingredients {
    fn new() -> Self {
        Ingredients { ranges: vec![] }
    }

    fn is_fresh(&self, id: u64) -> bool {
        self.ranges.iter().any(|&(a, b)| a <= id && id <= b)
    }

    fn add(&mut self, a: u64, b: u64) {
        self.ranges.push((a, b));
    }

    fn merge_intervals(&mut self) {
        if self.ranges.is_empty() {
            return;
        }

        self.ranges.sort();

        let mut merged = vec![];
        let (mut last_start, mut last_end) = self.ranges[0];

        for (start, end) in std::mem::take(&mut self.ranges) {
            if last_end < start {
                // no overlap
                merged.push((last_start, last_end));
                last_start = start;
                last_end = end;
            } else {
                last_end = last_end.max(end);
            }
        }

        merged.push((last_start, last_end));
        *self = Ingredients { ranges: merged }
    }

    fn count(&mut self) -> u64 {
        self.merge_intervals();

        let mut count = 0;
        for (a, b) in &self.ranges {
            count += b - a + 1;
        }

        count
    }
}

pub fn part1(input: &str) {
    let input = util::read_file(input).unwrap();

    let (fresh, available) = input.split_once("\n\n").unwrap();
    let mut buf = vec![];
    let mut ingredients = Ingredients::new();
    for line in fresh.lines() {
        buf.clear();

        util::parse_ints_buf_u64(line, &mut buf).unwrap();
        assert!(buf.len() == 2);
        ingredients.add(buf[0], buf[1]);
    }

    let available = util::parse_ints_u64(available).unwrap();
    let mut count = 0;
    for id in available {
        if ingredients.is_fresh(id) {
            println!("ingredient ID {id} is fresh");
            count += 1;
        } else {
            println!("ingredient ID {id} is spoiled");
        }
    }

    println!("{count}");
}

pub fn part2(input: &str) {
    let input = util::read_file(input).unwrap();

    let (fresh, _) = input.split_once("\n\n").unwrap();
    let mut buf = vec![];
    let mut ingredients = Ingredients::new();
    for line in fresh.lines() {
        buf.clear();

        util::parse_ints_buf_u64(line, &mut buf).unwrap();
        assert!(buf.len() == 2);
        ingredients.add(buf[0], buf[1]);
    }

    println!("{}", ingredients.count());
}
