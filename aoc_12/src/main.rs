mod moon;
mod moon_part;

use std::fs;
use std::hash::{Hash, Hasher};

use num::integer::lcm;

use crate::moon::Moon;
use crate::moon_part::MoonPartial;

struct PointAxis {
    pos: Vec<i32>,
    vel: Vec<i32>
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

fn step_parts(moon_parts: &mut Vec<MoonPartial>) {
    for i in 0..(moon_parts.len()-1) {
        let (moon1, rest) = moon_parts[i..].split_at_mut(1);
        for moon2 in rest {
            MoonPartial::apply_gravity(&mut moon1[0], moon2);
        }
    }

    for m in moon_parts {
        m.apply_forces();
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

fn get_steps_until_repeat(moon_parts: &mut Vec<MoonPartial>) -> i64 {
    let start = moon_parts.clone();
    let mut i = 0;
    loop {
        step_parts(moon_parts);
        i += 1;
        if *moon_parts == start {
            break;
        }
        if i % 1000 == 0 {
            println!("{i}");
        }
    }
    println!("Finished part with {i} steps");
    i
}

fn solve2() {
    // split up by coordinate => lcm
    let content = fs::read_to_string("input.txt").expect("Failed to read input");

    let mut mx = Vec::new();
    let mut my = Vec::new();
    let mut mz = Vec::new();

    for line in content.lines() {
        let (px, py, pz) = MoonPartial::from_str(line);
        mx.push(px);
        my.push(py);
        mz.push(pz);
    }

    let (ix, iy, iz) = (
        get_steps_until_repeat(&mut mx),
        get_steps_until_repeat(&mut my),
        get_steps_until_repeat(&mut mz)
    );

    let repeat = lcm(lcm(ix, iy), iz);

    println!("Answer 2: {repeat}");
}

fn main() {
    solve2();
}
