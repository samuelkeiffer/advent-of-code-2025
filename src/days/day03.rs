use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day03_test.txt"), 357);
    dbg!(part1("assets/day03.txt"));
    assert_eq!(part2("assets/day03_test.txt"), 3121910778619);
    dbg!(part2("assets/day03.txt"));
}

fn part2(file: &str) -> u64 {
    let banks = parse_file(file);
    banks
        .iter()
        .map(|bank| {
            let mut index = 0;
            (0..12)
                .rev()
                .map(|digit| {
                    let (i, joltage) = bank
                        .iter()
                        .enumerate()
                        .take(bank.len() - digit)
                        .skip(index)
                        .max_by_key(|(i, x)| **x as i32 * 1000 - *i as i32)
                        .unwrap();
                    index = i + 1;
                    *joltage as u64 * 10_u64.pow(digit as u32)
                })
                .sum::<u64>()
        })
        .sum()
}

fn part1(file: &str) -> u32 {
    let banks = parse_file(file);
    banks
        .iter()
        .map(|bank| {
            let (i, first) = bank
                .iter()
                .enumerate()
                .take(bank.len() - 1)
                .max_by_key(|(i, x)| **x as i32 * 1000 - *i as i32)
                .unwrap();
            let second = bank.iter().skip(i + 1).max().unwrap();
            first * 10 + second
        })
        .sum()
}

fn parse_file(file: &str) -> Vec<Vec<u32>> {
    let file = read_file(file);
    file.lines()
        .map(|line| {
            line.chars()
                .filter_map(|ch| ch.to_digit(10))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
