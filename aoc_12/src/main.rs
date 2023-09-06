mod moon;

use std::fs;
use std::hash::{Hash, Hasher};

use num::integer::lcm;
use regex::{Match, Regex};

use crate::moon::Moon;

fn parse_match(m: &Match) -> i128 {
    m.as_str().parse::<i128>().expect("Failed to parse capture group")
}

fn get_moon_coordinates(line: &str) -> (i128, i128, i128) {
    let re = Regex::new(r"^<x=(.*?), y=(.*?), z=(.*?)>$").unwrap();

    match re.captures(line) {
        Some(c) => (
            parse_match(&c.get(1).unwrap()),
            parse_match(&c.get(2).unwrap()),
            parse_match(&c.get(3).unwrap())
        ),
        None => panic!("Invalid input '{}'", line)
    }
}

fn step(moons: &mut Vec<Moon>) {
    for i in 0..(moons.len()-1) {
        let (moon1, rest) = moons[i..].split_at_mut(1);
        for moon2 in rest {
            Moon::apply_gravity(&mut moon1[0], moon2);
        }
    }

    for m in moons {
        m.apply_forces();
    }
}

fn step_parts(positions: &mut Vec<i128>, velocities: &mut Vec<i128>) {
    for i in 0..(positions.len()-1) {
        for j in (i+1)..positions.len() {
            let diff = (positions[j] - positions[i]).signum();
            velocities[i] += diff;
            velocities[j] -= diff;
        }
    }

    for i in 0..positions.len() {
        positions[i] += velocities[i];
    }
}

fn solve1() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input");
    let mut moons = content.lines().map(|l| Moon::from_string(l)).collect::<Vec<Moon>>();

    for i in 0..1000 {
        step(&mut moons);
    }
    for moon in &moons {
        moon.print_full();
    }

    let enery_sum = moons.iter().fold(0, |acc, m| acc + m.get_total_energy());
    println!("Answer 1: {}", enery_sum);
}

fn get_steps_until_repeat(start: Vec<i128>) -> i64 {
    let mut positions = start.clone();
    let zero_vec = vec![0i128; positions.len()];
    let mut velocities = zero_vec.clone();
    let mut i = 0;
    loop {
        step_parts(&mut positions, &mut velocities);
        i += 1;

        if positions == start && velocities == zero_vec {
            break;
        }

        if i % 1000 == 0 {
            println!("{i}");
        }
    }
    println!("Finished part with {i} steps");
    i
}

fn read_data_splitted(path: &str) -> Vec<Vec<i128>> {
    let mut all: Vec<Vec<i128>> = Vec::new();
    for _ in 0..3 {
        all.push(Vec::new());
    }

    let content = fs::read_to_string(path).expect("Failed to read input");
    for line in content.lines() {
        let (px, py, pz) = get_moon_coordinates(line);
        all[0].push(px);
        all[1].push(py);
        all[2].push(pz);
    }

    all
}

fn solve2() {
    // split up by coordinate => lcm
    let data = read_data_splitted("input.txt");

    let (ix, iy, iz) = (
        get_steps_until_repeat(data[0].clone()),
        get_steps_until_repeat(data[1].clone()),
        get_steps_until_repeat(data[2].clone())
    );

    let repeat = lcm(lcm(ix, iy), iz);

    println!("Answer 2: {repeat}");
}

fn main() {
    solve2();
}
