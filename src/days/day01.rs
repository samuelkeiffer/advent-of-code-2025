use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day01_test.txt"), 3);
    dbg!(part1("assets/day01.txt"));
    assert_eq!(part2("assets/day01_test.txt"), 6);
    dbg!(part2("assets/day01.txt"));
}

fn part2(file: &str) -> u32 {
    let rotations = parse_file(file);
    let mut pos = 50;
    let mut count = 0;
    for rot in rotations {
        let old_pos = pos;
        let mut dist_test = rot.dist;
        if dist_test > 100 {
            count += (dist_test as u32) / 100;
            dist_test = dist_test % 100;
        }
        let dist = if rot.left { -dist_test } else { dist_test };
        pos = (pos + dist + 100) % 100;
        if pos == 0 || (old_pos + dist != pos && old_pos != 0) {
            count += 1;
        }
    }
    count
}

fn part1(file: &str) -> u32 {
    let rotations = parse_file(file);
    let mut pos = 50;
    let mut count = 0;
    for rot in rotations {
        let dist = if rot.left { -rot.dist } else { rot.dist };
        pos = (pos + dist) % 100;
        if pos == 0 {
            count += 1;
        }
    }
    count
}

fn parse_file(file: &str) -> Vec<Rotation> {
    let rotations = read_file(file);
    rotations.lines().filter_map(|line| {
        let (dir, dist) = line.split_at(1);
        let dist: i32 = dist.parse().unwrap();
        match dir {
            "L" => Some(Rotation { left: true, dist }),
            "R" => Some(Rotation { left: false, dist }),
            _ => None,
        }
    }).collect::<Vec<Rotation>>()
}

struct Rotation {
    left: bool,
    dist: i32,
}
