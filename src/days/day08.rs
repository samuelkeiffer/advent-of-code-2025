use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day08_test.txt", 10), 40);
    dbg!(part1("assets/day08.txt", 1000));
    assert_eq!(part2("assets/day08_test.txt"), 25272);
    dbg!(part2("assets/day08.txt"));
}

fn part2(file: &str) -> i64 {
    let (box_pairs, count) = parse_file(file);
    let mut circuits: Vec<HashSet<Vec3<i64>>> = Vec::new();
    for (box_a, box_b) in box_pairs.values() {
        let mut a_index = None;
        let mut b_index = None;
        circuits.iter().enumerate().for_each(|(i, circuit)| {
            if circuit.contains(box_a) {
                a_index = Some(i);
            }
            if circuit.contains(box_b) {
                b_index = Some(i);
            }
        });
        if let Some((a_i, b_i)) = a_index.zip(b_index) {
            if a_i != b_i {
                let i = a_i.min(b_i);
                let j = a_i.max(b_i);
                let circuit = circuits.remove(j);
                circuits[i].extend(&circuit);
            }
        } else if let Some(i) = a_index.or(b_index) {
            circuits[i].insert(*box_a);
            circuits[i].insert(*box_b);
        } else {
            let mut circuit = HashSet::new();
            circuit.insert(*box_a);
            circuit.insert(*box_b);
            circuits.push(circuit);
        }
        if circuits[0].len() == count {
            return box_a.x * box_b.x;
        }
    }
    0
}

fn part1(file: &str, connections: u32) -> u64 {
    let (box_pairs, _) = parse_file(file);
    let mut circuits = box_pairs
        .values()
        .take(connections as usize)
        .map(|(a, b)| {
            let mut set = HashSet::new();
            set.insert(*a);
            set.insert(*b);
            set
        })
        .collect::<Vec<_>>();
    loop {
        let mut indices = None;
        for (i, circuit_a) in circuits.iter().enumerate() {
            if indices.is_some() {
                break;
            }
            for (j, circuit_b) in circuits.iter().enumerate().skip(i + 1) {
                if circuit_a.iter().any(|box_a| circuit_b.contains(box_a)) {
                    indices = Some((i, j));
                    break;
                }
            }
        }
        if let Some((i, j)) = indices {
            let circuit_b = circuits.remove(j);
            circuits[i].extend(&circuit_b);
        } else {
            break;
        }
    }
    circuits
        .iter()
        .map(|circuit| circuit.len() as u64)
        .k_largest(3)
        .product()
}

#[allow(clippy::type_complexity)]
fn parse_file(file: &str) -> (BTreeMap<i64, (Vec3<i64>, Vec3<i64>)>, usize) {
    let file = read_file(file);
    let boxes = file
        .lines()
        .map(|line| {
            line.split(",")
                .filter_map(|num| num.parse().ok())
                .collect::<Vec3<i64>>()
        })
        .collect::<Vec<_>>();
    (
        boxes
            .iter()
            .enumerate()
            .flat_map(|(i, box_a)| {
                boxes
                    .iter()
                    .skip(i + 1)
                    .map(|box_b| (box_a.distance_squared(*box_b), (*box_a, *box_b)))
            })
            .collect::<BTreeMap<i64, (Vec3<i64>, Vec3<i64>)>>(),
        boxes.len(),
    )
}
