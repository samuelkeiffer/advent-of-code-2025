use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day04_test.txt"), 13);
    dbg!(part1("assets/day04.txt"));
    assert_eq!(part2("assets/day04_test.txt"), 43);
    dbg!(part2("assets/day04.txt"));
}

fn part2(file: &str) -> u64 {
    let mut floor = parse_file(file);
    let mut to_remove = Vec::new();
    let mut count = 0;
    loop {
        count += floor
            .iter()
            .enumerate()
            .map(|(i, row)| {
                row.iter()
                    .enumerate()
                    .filter(|(j, paper)| {
                        let can_remove = **paper && test_adjacent(&floor, i, *j, |x| *x, false) < 4;
                        if can_remove {
                            to_remove.push((i, *j));
                        }
                        can_remove
                    })
                    .count() as u64
            })
            .sum::<u64>();
        if to_remove.is_empty() {
            break;
        }
        to_remove.drain(..).for_each(|(i, j)| floor[i][j] = false)
    }
    count
}

fn part1(file: &str) -> u64 {
    let floor = parse_file(file);
    floor
        .iter()
        .enumerate()
        .map(|(i, row)| {
            row.iter()
                .enumerate()
                .filter(|(j, paper)| **paper && test_adjacent(&floor, i, *j, |x| *x, false) < 4)
                .count() as u64
        })
        .sum()
}

fn parse_file(file: &str) -> Vec<Vec<bool>> {
    let file = read_file(file);
    file.lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '@' => true,
                    '.' => false,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}
