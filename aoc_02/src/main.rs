use std::fs;
use std::io::Error;

#[derive(Clone)]
struct Computer {
    ints: Vec<i32>,
    program_counter: i32
}

impl Computer {
    fn from_file(filepath: &str) -> Result<Computer, Error> {
        let content = fs::read_to_string(filepath)?;
        let values = content.split(',')
            .map(|s| s.parse().unwrap_or_else(|_| panic!("Unexpected value {s}")))
            .collect();

        Ok(Computer {
            ints: values,
            program_counter: 0
        })
    }

    fn new(state: Vec<i32>) -> Computer {
        Computer {
            ints: state.clone(),
            program_counter: 0
        }
    }

    fn execute_add(&mut self) {
        let in_addr1 = self.get_relative(1);
        let in_addr2 = self.get_relative(2);
        let out_addr = self.get_relative(3);

        let in_val1 = self.get_safe(in_addr1);
        let in_val2 = self.get_safe(in_addr2);

        self.set(out_addr, in_val1 + in_val2);

        self.program_counter += 4;
    }

    fn execute_mult(&mut self) {
        let in_addr1 = self.get_relative(1);
        let in_addr2 = self.get_relative(2);
        let out_addr = self.get_relative(3);

        let in_val1 = self.get_safe(in_addr1);
        let in_val2 = self.get_safe(in_addr2);

        self.set(out_addr, in_val1 * in_val2);

        self.program_counter += 4;
    }

    fn set(&mut self, index: i32, value: i32) {
        self.ints[index as usize] = value;
    }

    fn get_safe(&self, index: i32) -> i32 {
        *self.ints.get(index as usize).unwrap_or_else(|| panic!("No value at {index}"))
    }

    fn get_relative(&self, offset: i32) -> i32 {
        self.get_safe(self.program_counter + offset)
    }

    fn execute(&mut self) -> Result<(), String> {
        self.program_counter = 0;

        loop {
            let op = self.get_relative(0);
            match op {
                1 => self.execute_add(),
                2 => self.execute_mult(),
                //3 => {},
                //4 => {},
                99 => {
                    return Ok(());
                },
                _ => {
                    return Err(format!("Unexpected op-code {op}"));
                }
            }
        }

    }
}

fn solve1() {
    let mut com = Computer::from_file("input.txt").expect("Failed to init computer");
    com.set(1, 12);
    com.set(2, 2);
    com.execute().expect("Error during execution");
    println!("Answer 1: {}", com.get_safe(0));
}

fn solve2() {
    let mut com = Computer::from_file("input.txt").expect("Failed to init computer");
    for a in 0..100 {
        for b in 0..100 {
            let mut com_cpy = com.clone();
            com_cpy.set(1, a);
            com_cpy.set(2, b);
            match com_cpy.execute() {
                Ok(()) => if com_cpy.get_safe(0) == 19690720 {
                    println!("Answer 2: {}", 100 * a + b);
                    return;
                },
                Err(_) => continue
            }
        }
    }
}

fn main() {
    solve2();
}
