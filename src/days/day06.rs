use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day06_test.txt"), 4277556);
    dbg!(part1("assets/day06.txt"));
    assert_eq!(part2("assets/day06_test.txt"), 3263827);
    dbg!(part2("assets/day06.txt"));
}

fn part2(file: &str) -> u64 {
    let problems = parse_file2(file);
    problems
        .iter()
        .map(|p| match p.add {
            Some(true) => p.numbers.iter().sum(),
            Some(false) => p.numbers.iter().product(),
            None => 0,
        })
        .sum()
}

fn part1(file: &str) -> u64 {
    let problems = parse_file(file);
    problems
        .iter()
        .map(|p| match p.add {
            Some(true) => p.numbers.iter().sum(),
            Some(false) => p.numbers.iter().product(),
            None => 0,
        })
        .sum()
}

fn parse_file2(file: &str) -> Vec<Problem> {
    let file = read_file(file);
    let row_len = file.lines().map(|line| line.chars().count()).max().unwrap();
    let mut problems = Vec::new();
    let mut problem = Problem::default();
    (0..row_len).rev().for_each(|col| {
        let mut number = 0;
        let mut operation = None;
        file.lines().for_each(|line| {
            if let Some(ch) = line.chars().nth(col) {
                match ch {
                    '+' => operation = Some(true),
                    '*' => operation = Some(false),
                    x if x.is_ascii_digit() => number = number * 10 + x.to_digit(10).unwrap(),
                    _ => {}
                }
            }
        });
        if number > 0 {
            problem.numbers.push(number as u64);
        }
        if operation.is_some() {
            problem.add = operation;
            problems.push(std::mem::take(&mut problem));
        }
    });
    assert!(problems.iter().all(|p| p.add.is_some()));
    problems
}

fn parse_file(file: &str) -> Vec<Problem> {
    let file = read_file(file);
    let mut problems = Vec::new();
    file.lines().for_each(|line| {
        line.split_ascii_whitespace()
            .enumerate()
            .for_each(|(i, x)| {
                if let Ok(num) = x.parse() {
                    if problems.len() <= i {
                        problems.push(Problem {
                            add: None,
                            numbers: vec![num],
                        });
                    } else {
                        problems[i].numbers.push(num);
                    }
                } else {
                    match x {
                        "+" => problems[i].add = Some(true),
                        "*" => problems[i].add = Some(false),
                        _ => unreachable!(),
                    }
                }
            });
    });
    assert!(problems.iter().all(|p| p.add.is_some()));
    problems
}

#[derive(Default)]
struct Problem {
    add: Option<bool>,
    numbers: Vec<u64>,
}
