mod com;

use std::io;
use std::io::{stdin, Write};
use std::time::Instant;
use crate::com::Computer;

fn solve() {
    let mut com = Computer::from_file("input.txt").expect("Failed to init computer");
    com.push_next_input(2);

    while !com.is_finished() {
        match com.execute_until_input() {
            Ok(()) => {},
            Err(e) => panic!("Failed to execute program: {}", e)
        };

        while com.has_output() {
            println!("{}", com.pop_next_output().unwrap());
        }

        if !com.is_finished() {
            print!("input: ");
            io::stdout().flush().expect("Failed to flush");

            let mut input = String::new();
            stdin().read_line(&mut input).expect("Failed to read input");
            let parsed_num = input.trim().parse().expect("Failed to parse number from input");

            com.push_next_input(parsed_num);
        }
    }
}

fn main() {
    let now = Instant::now();
    for i in 0..1000 {
        solve();
    }
    let full_time = now.elapsed().as_nanos();
    println!("Finished after {} ms", full_time as f64 / 1.0E6);
    println!("One execution took {} ms", full_time as f64 / 1.0E6 / 1000.0);
}
