use std::collections::VecDeque;
use std::fs;
use std::io::Error;

#[derive(Clone)]
pub struct Computer2 {
    ints: Vec<i32>,
    program_counter: i32,
    inputs: VecDeque<i32>,
    outputs: VecDeque<i32>,
    waiting: bool
}

impl Computer2 {
    pub fn from_file(filepath: &str) -> Result<Computer2, Error> {
        let content = fs::read_to_string(filepath)?;
        let values = content.split(',')
            .map(|s| s.parse().unwrap_or_else(|_| panic!("Unexpected value {s}")))
            .collect();

        Ok(Computer2 {
            ints: values,
            program_counter: 0,
            inputs: VecDeque::new(),
            outputs: VecDeque::new(),
            waiting: false
        })
    }

    fn execute_add(&mut self, mut modes: i32) {
        let in1 = self.get_param(1, modes % 10);
        modes /= 10;
        let in2 = self.get_param(2, modes % 10);

        let out_addr = self.get_relative(3);

        self.set(out_addr, in1 + in2);

        self.program_counter += 4;
    }

    fn execute_mult(&mut self, mut modes: i32) {
        let in1 = self.get_param(1, modes % 10);
        modes /= 10;
        let in2 = self.get_param(2, modes % 10);

        let out_addr = self.get_relative(3);

        self.set(out_addr, in1 * in2);

        self.program_counter += 4;
    }

    fn execute_input(&mut self) {
        let wr_addr = self.get_relative(1);

        let in_val = match self.inputs.pop_front() {
            Some(val) => val,
            None => panic!("Expected more input values")
        };

        self.set(wr_addr, in_val);

        self.program_counter += 2;
    }

    fn execute_output(&mut self, modes: i32) {
        let value = self.get_param(1, modes % 10);

        self.outputs.push_back(value);

        self.program_counter += 2;
    }

    fn execute_jump_true(&mut self, mut modes: i32) {
        let val = self.get_param(1, modes % 10);
        modes /= 10;
        let target = self.get_param(2, modes % 10);

        if val != 0 {
            self.program_counter = target;
        } else {
            self.program_counter += 3;
        }
    }

    fn execute_jump_false(&mut self, mut modes: i32) {
        let val = self.get_param(1, modes % 10);
        modes /= 10;
        let target = self.get_param(2, modes % 10);

        if val == 0 {
            self.program_counter = target;
        } else {
            self.program_counter += 3;
        }
    }

    fn execute_less(&mut self, mut modes: i32) {
        let val1 = self.get_param(1, modes % 10);
        modes /= 10;
        let val2 = self.get_param(2, modes % 10);
        let target = self.get_relative(3);

        self.set(target, if val1 < val2 {1} else {0});

        self.program_counter += 4;
    }

    fn execute_equals(&mut self, mut modes: i32) {
        let val1 = self.get_param(1, modes % 10);
        modes /= 10;
        let val2 = self.get_param(2, modes % 10);
        let target = self.get_relative(3);

        self.set(target, if val1 == val2 {1} else {0});

        self.program_counter += 4;
    }

    fn set(&mut self, index: i32, value: i32) {
        self.ints[index as usize] = value;
    }

    fn get_param(&self, offset: i32, mode: i32) -> i32 {
        let p = self.get_relative(offset);
        if mode == 1 {
            return p;
        }
        return self.get_safe(p);
    }

    fn get_safe(&self, index: i32) -> i32 {
        *self.ints.get(index as usize).unwrap_or_else(|| panic!("No value at {index}"))
    }

    fn get_relative(&self, offset: i32) -> i32 {
        self.get_safe(self.program_counter + offset)
    }

    pub fn push_next_input(&mut self, val: i32) {
        self.inputs.push_back(val);
    }

    pub fn pop_next_output(&mut self) -> Option<i32> {
        self.outputs.pop_front()
    }

    pub fn is_waiting(&self) -> bool {
        self.waiting
    }

    pub fn has_output(&self) -> bool {
        !self.outputs.is_empty()
    }

    pub fn execute_until_input(&mut self) {
        loop {
            let op = self.get_relative(0);
            match op % 100 {
                1 => self.execute_add(op / 100),
                2 => self.execute_mult(op / 100),
                3 => {
                    if !self.inputs.is_empty() {
                        self.execute_input()
                    } else {
                        self.waiting = true;
                        return;
                    }
                },
                4 => self.execute_output(op / 100),
                5 => self.execute_jump_true(op / 100),
                6 => self.execute_jump_false(op / 100),
                7 => self.execute_less(op / 100),
                8 => self.execute_equals(op / 100),
                99 => {
                    return;
                },
                _ => {
                    panic!("Unexpected op-code {op}");
                }
            }
        }
    }
}