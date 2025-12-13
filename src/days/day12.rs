use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day12_test.txt", 2), 2);
    dbg!(part1("assets/day12.txt", 0));
}

fn part1(file: &str, checked: u64) -> u64 {
    let (shapes, regions) = parse_file(file);
    let sizes = shapes
        .iter()
        .map(|shape| shape.iter().flatten().filter(|x| **x).count())
        .collect::<Vec<_>>();
    let mut excess = 0;
    let mut check = 0;
    let mut small = 0;
    regions.iter().for_each(|region| {
        let min_size = region
            .presents
            .iter()
            .zip(sizes.iter())
            .map(|(p, s)| p * s)
            .sum::<usize>() as u32;
        let size = region.x * region.y;
        let presents = region.presents.iter().sum::<usize>() as u32;
        let grids = (region.x / 3) * (region.y / 3);
        if size < min_size {
            small += 1;
        } else if grids >= presents {
            excess += 1;
        } else {
            check += 1;
        }
    });
    dbg!(check);
    excess + checked
}

fn parse_file(file: &str) -> (Vec<[[bool; 3]; 3]>, Vec<Region>) {
    let file = read_file(file);
    let mut shapes = vec![[[false; 3]; 3]; 6];
    let mut regions = Vec::new();
    let mut parse_shape = Some((0, 0));
    file.lines().for_each(|line| {
        if let Some((shape, row)) = &mut parse_shape {
            if line.contains(['.', '#']) {
                line.chars().enumerate().for_each(|(i, ch)| {
                    shapes[*shape][*row][i] = matches!(ch, '#');
                });
                *row = (*row + 1) % 3;
                if *row == 0 {
                    *shape += 1;
                }
                if *shape > 5 {
                    parse_shape = None;
                }
            }
        } else if let Some((size, presents)) = line.split_once(": ") {
            let size = size
                .split_once('x')
                .and_then(|(x, y)| x.parse().ok().zip(y.parse().ok()));
            let presents = presents
                .split(' ')
                .filter_map(|x| x.parse().ok())
                .collect::<Vec<_>>();
            if let Some(size) = size {
                let region = Region {
                    x: size.0,
                    y: size.1,
                    presents,
                };
                regions.push(region);
            }
        }
    });
    (shapes, regions)
}

struct Region {
    x: u32,
    y: u32,
    presents: Vec<usize>,
}
