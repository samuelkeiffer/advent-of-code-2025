use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day10_test.txt"), 7);
    dbg!(part1("assets/day10.txt"));
    assert_eq!(part2("assets/day10_test.txt"), 33);
    dbg!(part2("assets/day10.txt"));
}

fn part2(file: &str) -> u64 {
    let machines = parse_file(file);
    machines
        .iter()
        .map(|machine| {
            let mut problem = Problem::new(OptimizationDirection::Minimize);
            let variables = machine
                .buttons
                .iter()
                .map(|_| problem.add_integer_var(1.0, (0, 255)))
                .collect::<Vec<_>>();
            machine.joltage.iter().enumerate().for_each(|(i, j)| {
                let mut linear_expr = LinearExpr::empty();
                machine
                    .buttons
                    .iter()
                    .zip(variables.iter())
                    .for_each(|(button, variable)| {
                        if button.contains(&i) {
                            linear_expr.add(*variable, 1.0);
                        }
                    });
                problem.add_constraint(linear_expr, ComparisonOp::Eq, *j as f64);
            });
            problem.solve().unwrap().objective().round() as u64
        })
        .sum()
}

fn part1(file: &str) -> u64 {
    let machines = parse_file(file);
    machines
        .iter()
        .map(|machine| {
            let start = vec![false; machine.lights.len()];

            let result = astar(
                &start,
                |l| successors(l.clone(), &machine.buttons),
                |l| heuristic(l, &machine.lights) / 3,
                |l| heuristic(l, &machine.lights) == 0,
            );
            result.unwrap().1
        })
        .sum::<usize>() as u64
}

fn heuristic(lights: &[bool], goal: &[bool]) -> usize {
    lights
        .iter()
        .zip(goal.iter())
        .filter(|(a, b)| a != b)
        .count()
}

fn successors<'a>(
    lights: Vec<bool>,
    buttons: &'a [Vec<usize>],
) -> impl Iterator<Item = (Vec<bool>, usize)> + use<'a> {
    buttons
        .iter()
        .map(move |button| (push_button(&lights, button), 1))
}

fn push_button(lights: &[bool], button: &[usize]) -> Vec<bool> {
    let mut new_state = lights.to_owned();
    for step in button {
        new_state[*step] ^= true;
    }
    new_state
}

fn parse_file(file: &str) -> Vec<Machine> {
    let file = read_file(file);
    file.lines()
        .map(|line| {
            let mut machine = Machine {
                lights: Vec::new(),
                buttons: Vec::new(),
                joltage: Vec::new(),
            };
            line.split(' ').for_each(|item| match item.chars().next() {
                Some('[') => {
                    machine.lights = item
                        .trim_matches(['[', ']'])
                        .chars()
                        .filter_map(|ch| match ch {
                            '.' => Some(false),
                            '#' => Some(true),
                            _ => None,
                        })
                        .collect::<Vec<_>>();
                }
                Some('(') => {
                    machine.buttons.push(
                        item.trim_matches(['(', ')'])
                            .split(',')
                            .filter_map(|num| num.parse().ok())
                            .collect::<Vec<_>>(),
                    );
                }
                Some('{') => {
                    machine.joltage = item
                        .trim_matches(['{', '}'])
                        .split(',')
                        .filter_map(|num| num.parse().ok())
                        .collect::<Vec<_>>();
                }
                _ => unreachable!(),
            });
            machine
        })
        .collect::<Vec<_>>()
}

struct Machine {
    lights: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<u8>,
}
