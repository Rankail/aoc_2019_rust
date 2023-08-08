use std::collections::HashSet;
use std::collections::HashMap;
use std::fs;

fn get_visited_nodes(input: &str) -> Result<HashSet<(i32, i32)>, &str> {
    let mut visited = HashSet::new();

    let mut pos = (0, 0);

    for d in input.split(",") {
        let dist = &d[1..].parse().expect("Failed to parse dist to travel {d}");
        match d.chars().next().unwrap() {
            'R' => {
                for i in (pos.0)..=(pos.0+dist) {
                    visited.insert((i, pos.1));
                }
                pos.0 += dist;
            },
            'L' => {
                for i in (pos.0-dist)..=(pos.0) {
                    visited.insert((i, pos.1));
                }
                pos.0 -= dist;
            },
            'U' => {
                for i in (pos.1)..=(pos.1+dist) {
                    visited.insert((pos.0, i));
                }
                pos.1 += dist;
            },
            'D' => {
                for i in (pos.1-dist)..=(pos.1+dist) {
                    visited.insert((pos.0, i));
                }

                pos.1 -= dist;
            }
            _ => {
                return Err("Unknown direction {d[0]}");
            }
        }
    }

    visited.remove(&(0, 0));

    Ok(visited)
}

fn get_visited_nodes_step(input: &str) -> Result<HashMap<(i32, i32), i32>, &str> {
    let mut visited = HashMap::new();

    let mut pos = (0, 0);

    let mut steps = 0;
    for d in input.split(",") {
        let dist = &d[1..].parse().expect("Failed to parse dist to travel {d}");
        match d.chars().next().unwrap() {
            'R' => {
                for i in (pos.0+1)..=(pos.0+dist) {
                    steps += 1;
                    visited.entry((i, pos.1)).or_insert(steps);
                }
                pos.0 += dist;
            },
            'L' => {
                for i in ((pos.0-dist)..=(pos.0-1)).rev() {
                    steps += 1;
                    visited.entry((i, pos.1)).or_insert(steps);
                }
                pos.0 -= dist;
            },
            'U' => {
                for i in (pos.1+1)..=(pos.1+dist) {
                    steps += 1;
                    visited.entry((pos.0, i)).or_insert(steps);
                }
                pos.1 += dist;
            },
            'D' => {
                for i in ((pos.1-dist)..=(pos.1-1)).rev() {
                    steps += 1;
                    visited.entry((pos.0, i)).or_insert(steps);
                }

                pos.1 -= dist;
            }
            _ => {
                return Err("Unknown direction {d[0]}");
            }
        }
    }

    visited.remove(&(0, 0));

    Ok(visited)
}

fn solve1() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input file.");
    let lines: Vec<&str> = content.lines().collect();

    let sets: Vec<HashSet<(i32, i32)>> = lines.iter().map(|l| get_visited_nodes(l).expect("Failed to parse a line")).collect();

    let cross = sets[0].intersection(&sets[1]);

    let min_pos = cross.min_by_key(|pos| pos.0.abs() + pos.1.abs()).expect("No cross found");

    let min_dist = min_pos.0.abs() + min_pos.1.abs();
    println!("Answer 1: {min_dist} at ({},{})", min_pos.0, min_pos.1);
}

fn solve2() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input file.");
    let lines: Vec<&str> = content.lines().collect();

    let maps: Vec<HashMap<(i32, i32), i32>> = lines.iter().map(|l| get_visited_nodes_step(l).expect("Failed to parse a line")).collect();

    let mut min = i32::MAX;
    let mut min_pos = (0, 0);
    for (pos, step1) in maps[0].clone() {
        match maps[1].get(&pos) {
            Some(step2) => {
                if step1 + step2 < min {
                    min = step1 + step2;
                    min_pos = pos.clone();
                }
            },
            None => ()
        };
    }

    println!("Answer 2: {} at ({}, {})", min, min_pos.0, min_pos.1);
}

fn main() {
    solve1();

    solve2();
}
