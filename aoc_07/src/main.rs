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
        let mut coms = vec![original.clone(), original.clone(), original.clone(), original.clone(), original.clone()];

        for (idx, (com, phase)) in zip(&mut coms, perm.clone()).enumerate() {
            com.push_next_input(phase.clone());
            com.execute_until_input();
            if idx == 0 {
                com.push_next_input(0 as i32);
            }
        }

        let mut last_e_out = -1;

        let mut signals_to_push = Vec::new();
        while coms.last().unwrap().is_waiting() {
            let mut signal_changed = false;
            for (idx, com) in coms.iter_mut().enumerate() {
                for signal in &signals_to_push {
                    com.push_next_input(*signal);
                    signal_changed = true;
                }
                signals_to_push.clear();

                if !com.is_waiting() {
                    com.execute_until_input();
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
            if !signal_changed {
                panic!("No more signals. Program got stuck");
            }
        }

        if last_e_out > max_signal {
            max_signal = last_e_out;
            best_perm = Some(perm.clone());
        }
    }

    println!("Answer 2: {} with perm {:?}", max_signal, best_perm);
}

fn main() {
    solve2();
}
