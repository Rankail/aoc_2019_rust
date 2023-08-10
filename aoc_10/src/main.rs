use std::collections::{HashSet, HashMap};
use std::fs;

fn gcd(first: i32, second: i32) -> i32 {
    let mut max: u32 = first.abs() as u32;
    let mut min: u32 = second.abs() as u32;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min as i32;
        }

        max = min;
        min = res;
    }
}

fn solve1() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut ast: HashSet<(i32, i32)> = HashSet::new();

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {},
                '#' => {ast.insert((x as i32,y as i32));},
                _ => panic!("Unexpected symbol {c}")
            };
        }
    }

    let mut max_asts = 0;
    let mut max_a = None;

    for a in ast.clone() {
        let mut rel_ast = HashSet::new();
        for b in ast.clone() {
            if a == b {
                continue;
            }

            let rel = (b.0 - a.0, b.1 - a.1);

            if rel.0 == 0 {
                rel_ast.insert((0, 1 * rel.1.signum()));
            } else if rel.1 == 0 {
                rel_ast.insert((1 * rel.0.signum(), 0));
            } else {
                let div = gcd(rel.0, rel.1);
                rel_ast.insert((rel.0 / div, rel.1 / div));
            }
        }

        if rel_ast.len() > max_asts {
            max_asts = rel_ast.len();
            max_a = Some(a.clone());
        }
    }

    match max_a {
        Some(a) => println!("Answer 1: {} at {:?}", max_asts, a),
        None => {}
    };
}

fn get_shortened(x: i32, y: i32) -> (i32, i32) {
    return if x == 0 {
        (0, 1 * y.signum())
    } else if y == 0 {
        (1 * x.signum(), 0)
    } else {
        let div = gcd(x, y);
        (x / div, y / div)
    }
}

fn solve2() {
    let content = fs::read_to_string("input.txt").expect("Failed to read input file");

    let mut asts: HashMap<(i32, i32), Vec<(i32, i32)>> = HashMap::new();

    let pos = (26, 36);

    for (y, line) in content.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            match c {
                '.' => {},
                '#' => {
                    let rel = (x as i32 - pos.0, y as i32 - pos.1);
                    let short = get_shortened(rel.0, rel.1);
                    asts.entry(short)
                        .and_modify(|v| v.push(rel))
                        .or_insert(vec![rel]);
                },
                _ => panic!("Unexpected symbol {c}")
            };
        }
    }

    for (_, v) in asts.iter_mut() {
        v.sort_by(|a, b| (b.0 + b.1).cmp(&(a.0 + a.1)));
    }

    let mut idx = 0;

    while idx + asts.len() < 200 {
        idx += asts.len();
        let _ = asts.iter_mut().for_each(|(k, v)| {
            v.pop();
        });
        asts = asts.iter().filter(|(k, v)| !v.is_empty()).collect();
    }

    let mut dirs: Vec<(i32, i32)> = asts.keys().cloned().collect();

    dirs.sort_by(|a, b| {
        let a_angle = (a.0 as f64).atan2(a.1 as f64);
        let b_angle = (b.0 as f64).atan2(b.1 as f64);
        a_angle.total_cmp(&b_angle)
    });

    loop {
        for dir in dirs {
            if idx == 200
        }
    }


}

fn main() {
    solve2();
}
