mod com;
mod com_a;

use std::collections::VecDeque;
use std::iter::zip;
use itertools::Itertools;
use crate::com::Computer;
use crate::com_a::Computer2;

fn solve1() {
    let com = Computer::from_file("input.txt").expect("Failed to init computer");
    let phases = vec![0, 1, 2, 3, 4];

    let mut max_signal = -1;
    let mut best_perm = None;
    for perm in phases.iter().permutations(phases.len()) {
        let mut com_cpy = com.clone();
        let mut signal = 0;

        for phase in perm.clone() {
            com_cpy.add_next_input(phase.clone());
            com_cpy.add_next_input(signal);
            let _ = com_cpy.execute();
            match com_cpy.get_last_output() {
                Some(result) => {
                    signal = result;
                },
                None => {
                    signal = -1;
                    break;
                }
            }
        }

        if signal > max_signal {
            max_signal = signal;
            best_perm = Some(perm.clone());
        }

    }

    println!("Answer 1: {} with perm {:?}", max_signal, best_perm);
}

fn solve2() {
    let mut v = VecDeque::new();
    v.push_back(5);
    v.push_back(0);
    println!("{:?}", v);

    let original = Computer2::from_file("input.txt").expect("Failed to init computer");


    let phases = vec![5, 6, 7, 8, 9];

    let mut max_signal = -1;
    let mut best_perm = None;

    for perm in phases.iter().permutations(phases.len()) {
        match run_perm(&original, &perm) {
            Ok(output) => {
                if output > max_signal {
                    max_signal = output;
                    best_perm = Some(perm.clone());
                }
            },
            Err(_) => {}
        }
    }

    println!("Answer 2: {} with perm {:?}", max_signal, best_perm);
}

fn run_perm(original: &Computer2, perm: &Vec<&i32>) -> Result<i32, &'static str> {
    let mut coms = vec![original.clone(), original.clone(), original.clone(), original.clone(), original.clone()];

    for (idx, (com, phase)) in zip(&mut coms, perm.clone()).enumerate() {
        com.push_next_input(phase.clone());
        com.execute_until_input();
        if idx == 0 {
            com.push_next_input(0);
        }
    }

    let mut last_e_out = -1;

    let mut signals_to_push = Vec::new();
    loop {
        let mut signal_changed = false;
        let mut all_finished = true;
        for (idx, com) in coms.iter_mut().enumerate() {
            for signal in &signals_to_push {
                com.push_next_input(*signal);
                signal_changed = true;
            }
            signals_to_push.clear();

            com.execute_until_input()?;
            if !com.is_finished() {
                all_finished = false;
            }

            while com.has_output() {
                signal_changed = true;
                let out = com.pop_next_output().unwrap();
                if idx == 4 {
                    last_e_out = out;
                }
                signals_to_push.push(out);
            }
        }
        if all_finished {
            return Ok(last_e_out);
        }
        if !signal_changed {
            return Err("No more signals. Program got stuck");
        }
    }
}

fn main() {
    solve2();
}
