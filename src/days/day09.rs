use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day09_test.txt"), 50);
    dbg!(part1("assets/day09.txt"));
    assert_eq!(part2("assets/day09_test.txt"), 24);
    dbg!(part2("assets/day09.txt"));
}

fn part2(file: &str) -> i64 {
    let (tile_pairs, edges) = parse_file(file);
    tile_pairs
        .iter()
        .map(|((x0, y0), (x1, y1))| {
            let area = (x1 - x0 + 1) * (y1 - y0 + 1);
            (area, ((x0, y0), (x1, y1)))
        })
        .sorted_by_key(|(area, _)| *area)
        .rev()
        .filter(|(_, ((x0, y0), (x1, y1)))| {
            edges.iter().all(|((xa, ya), (xb, yb))| {
                (xa <= x0 && xb <= x0)
                    || (xa >= x1 && xb >= x1)
                    || (ya <= y0 && yb <= y0)
                    || (ya >= y1 && yb >= y1)
            })
        })
        .map(|(area, _)| area)
        .next()
        .unwrap_or(0)
}

fn part1(file: &str) -> i64 {
    let (tile_pairs, _) = parse_file(file);
    tile_pairs
        .iter()
        .map(|((x0, y0), (x1, y1))| (x1 - x0 + 1) * (y1 - y0 + 1))
        .max()
        .unwrap_or(0)
}

#[allow(clippy::type_complexity)]
fn parse_file(file: &str) -> (Vec<((i64, i64), (i64, i64))>, Vec<((i64, i64), (i64, i64))>) {
    let file = read_file(file);
    let tiles = file
        .lines()
        .filter_map(|line| {
            line.split_once(',')
                .and_then(|(a, b)| a.parse::<i64>().ok().zip(b.parse::<i64>().ok()))
        })
        .collect::<Vec<_>>();
    let pairs = tiles
        .iter()
        .enumerate()
        .flat_map(|(i, (xa, ya))| {
            tiles
                .iter()
                .skip(i + 1)
                .map(|(xb, yb)| ((*xa.min(xb), *ya.min(yb)), (*xa.max(xb), *ya.max(yb))))
        })
        .collect::<Vec<_>>();
    let edges = tiles
        .iter()
        .copied()
        .zip(tiles.iter().skip(1).chain(tiles.iter().take(1)).copied())
        .collect::<Vec<_>>();
    (pairs, edges)
}
