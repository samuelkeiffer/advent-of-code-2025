use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day05_test.txt"), 3);
    dbg!(part1("assets/day05.txt"));
    assert_eq!(part2("assets/day05_test.txt"), 14);
    dbg!(part2("assets/day05.txt"));
}

fn part2(file: &str) -> u64 {
    let (id_ranges, _) = parse_file(file);
    let mut count = 0;
    id_ranges
        .iter()
        .enumerate()
        .for_each(|(i, (mut start, mut end))| {
            id_ranges.iter().take(i).for_each(|(st, en)| {
                if (st..=en).contains(&&start) {
                    start = en + 1;
                }
                if (st..=en).contains(&&end) {
                    end = st - 1;
                }
            });
            if end >= start {
                count += end + 1 - start;
            }
        });
    count
}

fn part1(file: &str) -> u64 {
    let (id_ranges, ingredients) = parse_file(file);
    ingredients
        .iter()
        .filter(|ingredient| {
            id_ranges
                .iter()
                .any(|(start, end)| (start..=end).contains(ingredient))
        })
        .count() as u64
}

fn parse_file(file: &str) -> (Vec<(u64, u64)>, Vec<u64>) {
    let file = read_file(file);
    let mut id_ranges = Vec::new();
    let mut ingredients = Vec::new();
    file.lines().for_each(|line| {
        if let Some((start, end)) = line.split_once('-') {
            id_ranges.push((start.parse().unwrap(), end.parse().unwrap()));
        } else if let Ok(ingredient) = line.parse() {
            ingredients.push(ingredient);
        }
    });
    id_ranges = id_ranges
        .into_iter()
        .sorted_by_key(|(start, _)| *start)
        .collect::<Vec<_>>();
    (id_ranges, ingredients)
}
