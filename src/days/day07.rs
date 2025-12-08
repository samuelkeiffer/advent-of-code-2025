use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day07_test.txt"), 21);
    dbg!(part1("assets/day07.txt"));
    assert_eq!(part2("assets/day07_test.txt"), 40);
    dbg!(part2("assets/day07.txt"));
}

fn part2(file: &str) -> u64 {
    let manifold = parse_file(file);
    let mut manifolds = HashMap::new();
    manifolds.insert(manifold, 1);
    let mut count = 1;
    loop {
        let mut future_manifolds = HashMap::new();
        let mut break_now = false;
        manifolds.iter().for_each(|(manifold, futures)| {
            let (future_a, inc, break_next) = get_future(manifold, Some(true));
            let (future_b, _, _) = get_future(manifold, Some(false));
            if inc > 0 {
                count += futures;
            }
            if future_a == future_b {
                future_manifolds
                    .entry(future_a)
                    .and_modify(|f| *f += futures)
                    .or_insert(*futures);
            } else {
                future_manifolds
                    .entry(future_a)
                    .and_modify(|f| *f += futures)
                    .or_insert(*futures);
                future_manifolds
                    .entry(future_b)
                    .and_modify(|f| *f += futures)
                    .or_insert(*futures);
            }
            break_now = break_next;
        });
        if break_now {
            break;
        }
        manifolds = future_manifolds;
    }
    count
}

fn part1(file: &str) -> u64 {
    let mut manifold = parse_file(file);
    let mut count = 0;
    loop {
        let (future, inc, break_next) = get_future(&manifold, None);
        count += inc;
        if break_next {
            break;
        }
        manifold = future;
    }
    count
}

fn get_future(manifold: &[Vec<Item>], left: Option<bool>) -> (Vec<Vec<Item>>, u64, bool) {
    let mut future = manifold.to_owned();
    let mut break_next = false;
    let mut count = 0;
    manifold.iter().enumerate().for_each(|(i, row)| {
        row.iter().enumerate().for_each(|(j, item)| match item {
            Item::Empty => {}
            Item::Splitter => {}
            Item::Tachyon => {
                future[i][j] = Item::Empty;
                if let Some(below) = manifold.get(i + 1).and_then(|row| row.get(j)) {
                    match below {
                        Item::Empty => {
                            future[i + 1][j] = Item::Tachyon;
                        }
                        Item::Splitter => match left {
                            None => {
                                future[i + 1][j - 1] = Item::Tachyon;
                                future[i + 1][j + 1] = Item::Tachyon;
                                count += 1;
                            }
                            Some(true) => {
                                future[i + 1][j - 1] = Item::Tachyon;
                                count += 1;
                            }
                            Some(false) => {
                                future[i + 1][j + 1] = Item::Tachyon;
                            }
                        },
                        Item::Tachyon => {}
                    }
                } else {
                    break_next = true;
                }
            }
        });
    });
    (future, count, break_next)
}

fn parse_file(file: &str) -> Vec<Vec<Item>> {
    let file = read_file(file);
    file.lines()
        .map(|line| {
            line.chars()
                .map(|ch| match ch {
                    '.' => Item::Empty,
                    '^' => Item::Splitter,
                    'S' => Item::Tachyon,
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
enum Item {
    Empty,
    Splitter,
    Tachyon,
}
