use std::collections::HashMap;
use std::fs;
use std::iter::zip;
use std::time::Instant;

fn sum_leaf_depths(orbits: &HashMap<&str, Vec<&str>>, depth: i32, cur: &str) -> i32 {
    let mut sum = depth;
    match orbits.get(cur) {
        Some(childs) => {
            for child in childs {
                sum += sum_leaf_depths(orbits, depth + 1, child);
            }
        },
        None => {}
    }

    sum
}

fn solve1() {
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();

    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    for line in content.lines() {
        let (main, orbit) = match line.split_once(')'){
            Some((v1, v2)) => (v1, v2),
            None => panic!("Invalid input {line}")
        };

        orbits.entry(main)
            .and_modify(|v| v.push(orbit))
            .or_insert(vec![orbit]);
    }

    let all_orbits = sum_leaf_depths(&orbits, 0, "COM");

    println!("Answer 1: {all_orbits}");
}

fn get_path<'a>(orbits: &HashMap<&'a str, Vec<&'a str>>, path: &mut Vec<&'a str>, target: &str) -> Option<Vec<&'a str>> {
    match orbits.get(path.last().expect("Unexpected empty path")) {
        None => {},
        Some(childs) => {
            for child in childs {
                if child == &target {
                    return Some(path.clone());
                }
                path.push(child);
                match get_path(orbits, path, target) {
                    Some(p) => return Some(p),
                    None => {}
                }
                path.pop();
            }
        }
    };

    return None;
}

fn solve2() {
    let mut orbits: HashMap<&str, Vec<&str>> = HashMap::new();

    let content = fs::read_to_string("input.txt")
        .expect("Failed to read input file");

    for line in content.lines() {
        let (main, orbit) = match line.split_once(')'){
            Some((v1, v2)) => (v1, v2),
            None => panic!("Invalid input {line}")
        };

        orbits.entry(main)
            .and_modify(|v| v.push(orbit))
            .or_insert(vec![orbit]);
    }

    let path1 = get_path(&orbits, &mut vec!["COM"], "SAN").expect("Failed to find SAN");
    let path2 = get_path(&orbits, &mut vec!["COM"], "YOU").expect("Failed to find YOU");
    for (i, (a, b)) in zip(path1.clone(), path2.clone()).enumerate() {
        if a != b {
            let dist = path1.len() - i + path2.len() - i;
            println!("Answer 2: {dist}");
            return;
        }
    }
}

fn main() {
    let now = Instant::now();

    solve1();
    solve2();

    let elapsed = (now.elapsed().as_nanos() as f64) / 1.0e6;
    println!("Elapsed: {} ms", elapsed);
}
