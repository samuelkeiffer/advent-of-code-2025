use crate::*;

pub fn run() {
    assert_eq!(part1("assets/day11_test.txt"), 5);
    dbg!(part1("assets/day11.txt"));
    assert_eq!(part2("assets/day11_test2.txt"), 2);
    dbg!(part2("assets/day11.txt"));
}

fn part2(file: &str) -> u64 {
    let devices = parse_file(file);
    let mut cache = HashMap::new();
    let path = vec![String::from("svr")];
    test_path(path, &devices, &mut cache).0
}

// (both, fft, dac, none)
fn test_path(
    path: Vec<String>,
    devices: &HashMap<String, Vec<String>>,
    cache: &mut HashMap<String, (u64, u64, u64, u64)>,
) -> (u64, u64, u64, u64) {
    if let Some(device) = path.last() {
        if let Some(stored_count) = cache.get(device) {
            return *stored_count;
        } else if *device == "out" {
            return (0, 0, 0, 1);
        } else if let Some(dests) = devices.get(device) {
            let mut both = 0;
            let mut fft = 0;
            let mut dac = 0;
            let mut none = 0;
            dests.iter().for_each(|dest| {
                let mut new_path = path.clone();
                new_path.push(dest.to_owned());
                let (a, b, c, d) = test_path(new_path, devices, cache);
                match device.as_str() {
                    "dac" => {
                        dac += d + c;
                        both += b + a;
                    }
                    "fft" => {
                        fft += d + b;
                        both += c + a;
                    }
                    _ => {
                        both += a;
                        fft += b;
                        dac += c;
                        none += d;
                    }
                }
            });
            cache.insert(device.to_owned(), (both, fft, dac, none));
            return (both, fft, dac, none);
        }
    }
    (0, 0, 0, 0)
}

fn part1(file: &str) -> u64 {
    let devices = parse_file(file);
    let mut paths = vec!["you"];
    let mut count = 0;
    loop {
        if paths.is_empty() {
            break;
        }
        let test = paths.remove(0);
        if test == "out" {
            count += 1
        } else if let Some(test) = devices.get(test) {
            paths.extend(test.iter().map(|s| s.as_str()));
        }
    }
    count
}

fn parse_file(file: &str) -> HashMap<String, Vec<String>> {
    let file = read_file(file);
    file.lines()
        .filter_map(|line| {
            if let Some((source, destinations)) = line.split_once(": ") {
                Some((
                    source.to_owned(),
                    destinations
                        .split(' ')
                        .map(|dest| dest.to_owned())
                        .collect::<Vec<_>>(),
                ))
            } else {
                None
            }
        })
        .collect()
}
