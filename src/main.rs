mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;
mod day10;
mod day11;
mod day12;

pub mod util;

fn main() {
    let day = std::env::args()
        .nth(1)
        .expect("Enter a day. e.g. \"1.1\" for day 1, part 1.");
    let filename = std::env::args().nth(2);
    match day.as_ref() {
        "1.1" => day01::part1(&filename.unwrap_or("input/day01.txt".to_string())),
        "1.2" => day01::part2(&filename.unwrap_or("input/day01.txt".to_string())),

        "2.1" => day02::part1(&filename.unwrap_or("input/day02.txt".to_string())),
        "2.2" => day02::part2(&filename.unwrap_or("input/day02.txt".to_string())),

        "3.1" => day03::part1(&filename.unwrap_or("input/day03.txt".to_string())),
        "3.2" => day03::part2(&filename.unwrap_or("input/day03.txt".to_string())),

        "4.1" => day04::part1(&filename.unwrap_or("input/day04.txt".to_string())),
        "4.2" => day04::part2(&filename.unwrap_or("input/day04.txt".to_string())),

        "5.1" => day05::part1(&filename.unwrap_or("input/day05.txt".to_string())),
        "5.2" => day05::part2(&filename.unwrap_or("input/day05.txt".to_string())),

        "6.1" => day06::part1(&filename.unwrap_or("input/day06.txt".to_string())),
        "6.2" => day06::part2(&filename.unwrap_or("input/day06.txt".to_string())),

        "7.1" => day07::part1(&filename.unwrap_or("input/day07.txt".to_string())),
        "7.2" => day07::part2(&filename.unwrap_or("input/day07.txt".to_string())),

        "8.1" => day08::part1(&filename.unwrap_or("input/day08.txt".to_string())),
        "8.2" => day08::part2(&filename.unwrap_or("input/day08.txt".to_string())),

        "9.1" => day09::part1(&filename.unwrap_or("input/day09.txt".to_string())),
        "9.2" => day09::part2(&filename.unwrap_or("input/day09.txt".to_string())),

        "10.1" => day10::part1(&filename.unwrap_or("input/day10.txt".to_string())),
        "10.2" => day10::part2(&filename.unwrap_or("input/day10.txt".to_string())),

        "11.1" => day11::part1(&filename.unwrap_or("input/day11.txt".to_string())),
        "11.2" => day11::part2(&filename.unwrap_or("input/day11.txt".to_string())),

        "12.1" => day12::part1(&filename.unwrap_or("input/day12.txt".to_string())),
        "12.2" => day12::part2(&filename.unwrap_or("input/day12.txt".to_string())),

        _ => println!("Part {day} not available."),
    }
}
