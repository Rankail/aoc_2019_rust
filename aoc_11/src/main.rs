mod com;

use std::collections::HashMap;
use std::io::Write;
use std::time::Instant;
use crate::com::Computer;

enum Direction {
    LEFT,
    RIGHT,
    DOWN,
    UP
}

impl Direction {
    fn as_string(&self) -> &'static str {
        match self {
            Direction::UP => "Up",
            Direction::DOWN => "Down",
            Direction::RIGHT => "Right",
            Direction::LEFT => "Left"
        }
    }
}

fn get_dir_left(dir: &Direction) -> Direction {
    match dir {
        Direction::LEFT => Direction::DOWN,
        Direction::RIGHT => Direction::UP,
        Direction::DOWN => Direction::RIGHT,
        Direction::UP => Direction::LEFT
    }
}

fn get_dir_right(dir: &Direction) -> Direction {
    match dir {
        Direction::LEFT => Direction::UP,
        Direction::RIGHT => Direction::DOWN,
        Direction::DOWN => Direction::LEFT,
        Direction::UP => Direction::RIGHT
    }
}

fn step(pos: (i32, i32), dir: &Direction) -> (i32, i32) {
    match dir {
        Direction::LEFT     => (pos.0 - 1, pos.1),
        Direction::RIGHT    => (pos.0 + 1, pos.1),
        Direction::DOWN     => (pos.0, pos.1 - 1),
        Direction::UP       => (pos.0, pos.1 + 1)
    }
}

fn solve1() {
    let mut com = Computer::from_file("input.txt").expect("Failed to init computer");

    let mut board: HashMap<(i32, i32), u8> = HashMap::new();

    board.insert((0,0), 1);

    let mut pos = (0, 0);
    let mut dir: Direction = Direction::UP;

    while !com.is_finished() {
        com.push_next_input(*board.get(&pos).unwrap_or(&0) as i128);
        match com.execute_until_input() {
            Ok(()) => {},
            Err(e) => panic!("Failed to execute program: {}", e)
        };

        if com.is_finished() {
            break;
        }

        if com.get_output_count() != 2 {
            panic!("Unexpected output count {}", com.get_output_count());
        }

        let color = match com.pop_next_output().unwrap() {
            0 => 0,
            1 => 1,
            c => panic!("Unexpected color {c}")
        };
        board.insert(pos, color);

        dir = match com.pop_next_output().unwrap() {
            0 => get_dir_left(&dir),
            1 => get_dir_right(&dir),
            d => panic!("Unexpected tunr instruction {d}")
        };

        println!("c: {} ({},{}) {}", color, pos.0, pos.1, dir.as_string());

        pos = step(pos, &dir);
    }

    println!("Answer 1: {}", board.len());

    board.retain(|_, v| *v == 1);

    let mut min_x = i32::MAX;
    let mut min_y = i32::MAX;
    let mut max_x = -i32::MAX;
    let mut max_y = -i32::MAX;

    for (x, y) in board.keys() {
        min_x = min_x.min(*x);
        min_y = min_y.min(*y);
        max_x = max_x.max(*x);
        max_y = max_y.max(*y);
    }

    println!("x: {}..{} y: {}..{}", min_x, max_x, min_y, max_y);

    for y in (min_y..=max_y).rev() {
        for x in min_x..=max_x {
            print!("{}", match board.get(&(x, y)).unwrap_or(&0) {
                0 => ' ',
                1 => '#',
                _ => '/'
            });
        }
        println!();
    }


}

fn main() {
    let now = Instant::now();
    //for i in 0..1000 {
    solve1();
    //}
    let full_time = now.elapsed().as_nanos();
    println!("Finished after {} ms", full_time as f64 / 1.0E6);
    println!("One execution took {} ms", full_time as f64 / 1.0E6 / 1000.0);
}
