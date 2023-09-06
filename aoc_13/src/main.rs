use std::collections::HashMap;
use std::io;
use std::io::Read;
use crate::com::Computer;

mod com;

fn solve1() {
    let mut com = Computer::from_file("input.txt").unwrap();
    com.execute_until_input().unwrap();

    let mut tiles = HashMap::new();
    while com.has_output() {
        let x = com.pop_next_output().unwrap() as i32;
        let y = com.pop_next_output().unwrap() as i32;
        let typ = com.pop_next_output().unwrap() as u8;
        tiles.insert((x, y), typ);
    }

    let mut count = 0;
    let mut mx = 0;
    let mut my = 0;
    for ((x, y), typ) in tiles {
        if typ == 2 {
            count += 1;
        }
        mx = mx.max(x);
        my = my.max(y);
    }

    println!("{} {}", mx, my);
    println!("Answer 1: {}", count);
}

fn step(com: &mut Computer, tiles: &mut HashMap<(i32, i32), i32>, score: &mut i32) {
    com.execute_until_input().unwrap();
    while com.has_output() {
        let x = com.pop_next_output().unwrap() as i32;
        let y = com.pop_next_output().unwrap() as i32;
        let typ = com.pop_next_output().unwrap() as i32;

        if x == -1 && y == 0 {
            *score = typ;
        } else {
            tiles.insert((x, y), typ);
        }
    }
}

fn print_board(tiles: &HashMap<(i32, i32), i32>, score: &i32) {
    for y in 0..24 {
        for x in 0..42 {
            if tiles.contains_key(&(x, y)) {
                print!("{}", match tiles.get(&(x, y)).unwrap() {
                    0 => " ",
                    1 => "#",
                    2 => "=",
                    3 => "-",
                    4 => "0",
                    _ => "E"
                })
            } else {
                print!(" ")
            }
        }
        println!();
    }
    println!("{}", score);
}

fn move_paddle(tiles: &HashMap<(i32, i32), i32>, com: &mut Computer) {
    let ball_x = tiles.iter()
        .find_map(|((x, y), v)| if *v == 4 { Some(x) } else { None })
        .unwrap();

    let paddle_x = tiles.iter()
        .find_map(|((x, y), v)| if *v == 3 { Some(x) } else { None })
        .unwrap();

    let dir = (ball_x - paddle_x).signum();
    com.push_next_input(dir as i128);
}

fn play() {
    let mut com = Computer::from_file("input.txt").unwrap();

    com.set(0, 2);

    let mut tiles = HashMap::new();
    let mut score = 0;

    while !com.is_finished() {
        step(&mut com, &mut tiles, &mut score);

        //print_board(&tiles, &score);

        //let mut input = String::new();
        //io::stdin().read_line(&mut input).expect("Failed to wait");

        move_paddle(&tiles, &mut com);
    }

    println!("Answer 2: {}", score);
}

fn main() {
    //solve1();
    play();
}
