use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day02_test.txt"), 1227775554);
    dbg!(part1("assets/day02.txt"));
    assert_eq!(part2("assets/day02_test.txt"), 4174379265);
    dbg!(part2("assets/day02.txt"));
}

fn part2(file: &str) -> u64 {
    let id_ranges = parse_file(file);
    id_ranges.iter().map(|range| {
        (range.0..=range.1).filter(|id: &u64| {
            let log = id.ilog10();
            (2..50).filter(|test| {
                (log + 1) % test == 0
            }).map(|repeat| {
                (0..repeat).map(|x| 10_u64.pow((log + 1) * x / repeat)).sum::<u64>()
            }).any(|factor| id % factor == 0)
        }).sum::<u64>()
    }).sum()
}

fn part1(file: &str) -> u64 {
    let id_ranges = parse_file(file);
    id_ranges.iter().map(|range| {
        (range.0..=range.1).filter(|id: &u64| {
            let log = id.ilog10();
            let factor = 10_u64.pow((log + 1) / 2) + 1;
            log % 2 != 0 && id % factor == 0
        }).sum::<u64>()
    }).sum()
}

fn parse_file(file: &str) -> Vec<(u64, u64)> {
    let id_ranges = read_file(file);
    id_ranges.split(',').filter_map(|range| {
        range.split_once('-').and_then(|(s, e)| s.parse().ok().zip(e.parse().ok()))
    }).collect::<Vec<(u64, u64)>>()
}