use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::Error;

#[derive(Clone)]
pub struct Computer {
    memory: HashMap<u32, i128>,
    program_counter: u32,
    relative_base: i128,
    inputs: VecDeque<i128>,
    outputs: VecDeque<i128>,
    finished: bool
}

impl Computer {
    pub fn from_file(filepath: &str) -> Result<Computer, Error> {
        let content = fs::read_to_string(filepath)?;
        let values = content.split(',').enumerate()
            .map(|(idx, s)| {
                (idx as u32, s.trim().parse()
                    .unwrap_or_else(|_| panic!("Unexpected value {s}")))
            })
            .collect();

        Ok(Computer {
            memory: values,
            program_counter: 0,
            relative_base: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            finished: false
        })
    }

    fn execute_add(&mut self, mut modes: i128) {
        let in1 = self.get_param(1, modes % 10);
        modes /= 10;
        let in2 = self.get_param(2, modes % 10);
        modes /= 10;
        let out_addr = self.get_addr(3, modes % 10) as u32;

        self.set(out_addr, in1 + in2);

        self.program_counter += 4;
    }

    fn execute_mult(&mut self, mut modes: i128) {
        let in1 = self.get_param(1, modes % 10);
        modes /= 10;
        let in2 = self.get_param(2, modes % 10);
        modes /= 10;
        let out_addr = self.get_addr(3, modes % 10);

        self.set(out_addr, in1 * in2);

        self.program_counter += 4;
    }

    fn execute_input(&mut self, modes: i128) {
        let wr_addr = self.get_addr(1, modes % 10);

        let in_val = match self.inputs.pop_front() {
            Some(val) => val,
            None => panic!("Expected more input values")
        };

        self.set(wr_addr, in_val);

        self.program_counter += 2;
    }

    fn execute_output(&mut self, modes: i128) {
        let value = self.get_param(1, modes % 10);

        self.outputs.push_back(value);

        self.program_counter += 2;
    }

    fn execute_jump_true(&mut self, mut modes: i128) {
        let val = self.get_param(1, modes % 10);
        modes /= 10;
        let target = self.get_param(2, modes % 10) as u32;

        if val != 0 {
            self.program_counter = target;
        } else {
            self.program_counter += 3;
        }
    }

    fn execute_jump_false(&mut self, mut modes: i128) {
        let val = self.get_param(1, modes % 10);
        modes /= 10;
        let target = self.get_param(2, modes % 10) as u32;

        if val == 0 {
            self.program_counter = target;
        } else {
            self.program_counter += 3;
        }
    }

    fn execute_less(&mut self, mut modes: i128) {
        let val1 = self.get_param(1, modes % 10);
        modes /= 10;
        let val2 = self.get_param(2, modes % 10);
        modes /= 10;
        let target = self.get_addr(3, modes % 10);

        self.set(target, if val1 < val2 {1} else {0});

        self.program_counter += 4;
    }

    fn execute_equals(&mut self, mut modes: i128) {
        let val1 = self.get_param(1, modes % 10);
        modes /= 10;
        let val2 = self.get_param(2, modes % 10);
        modes /= 10;
        let target = self.get_addr(3, modes % 10);

        self.set(target, if val1 == val2 {1} else {0});

        self.program_counter += 4;
    }

    fn execute_adjust_relative_base(&mut self, modes: i128) {
        let val = self.get_param(1, modes % 10);

        self.relative_base += val;

        self.program_counter += 2;
    }

    pub fn set(&mut self, index: u32, value: i128) {
        self.memory.insert(index, value);
    }

    fn get_addr(&self, offset: u32, mode: i128) -> u32 {
        return match mode {
            0 => self.get_relative(offset),
            2 => self.relative_base + self.get_relative(offset),
            _ => panic!("Unknown mode for addr {mode}")
        } as u32
    }

    fn get_param(&self, offset: u32, mode: i128) -> i128 {
        return match mode {
            0 => self.get_safe(self.get_relative(offset) as u32),
            1 => self.get_relative(offset),
            2 => self.get_safe((self.relative_base + self.get_relative(offset)) as u32),
            _ => panic!("Unknown mode for value {mode}")
        };
    }

    fn get_safe(&self, index: u32) -> i128 {
        self.memory.get(&index).cloned().unwrap_or(0)
    }

    fn get_relative(&self, offset: u32) -> i128 {
        self.get_safe(self.program_counter + offset)
    }

    pub fn push_next_input(&mut self, val: i128) {
        self.inputs.push_back(val);
    }

    pub fn pop_next_output(&mut self) -> Option<i128> {
        self.outputs.pop_front()
    }

    pub fn is_finished(&self) -> bool {
        self.finished
    }

    pub fn has_output(&self) -> bool {
        !self.outputs.is_empty()
    }

    pub fn get_output_count(&self) -> usize {
        self.outputs.len()
    }

    pub fn execute_until_input(&mut self) -> Result<(), &'static str> {
        loop {
            let op = self.get_relative(0);
            match op % 100 {
                1 => self.execute_add(op / 100),
                2 => self.execute_mult(op / 100),
                3 => {
                    if !self.inputs.is_empty() {
                        self.execute_input(op / 100)
                    } else {
                        return Ok(());
                    }
                },
                4 => self.execute_output(op / 100),
                5 => self.execute_jump_true(op / 100),
                6 => self.execute_jump_false(op / 100),
                7 => self.execute_less(op / 100),
                8 => self.execute_equals(op / 100),
                9 => self.execute_adjust_relative_base(op / 100),
                99 => {
                    self.finished = true;
                    return Ok(());
                },
                _ => {
                    return Err("Unexpected op-code {op}");
                }
            }
        }
    }
}