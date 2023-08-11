use std::hash::{Hash, Hasher};
use std::iter::zip;
use regex::{Match, Regex};

#[derive(Clone)]
pub struct Moon {
    pos: Vec<i32>,
    vel: Vec<i32>
}


impl PartialEq for Moon {
    fn eq(&self, other: &Moon) -> bool {
        self.pos == other.pos
    }
}

impl Eq for Moon {}

impl Hash for Moon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.vel.hash(state);
    }
}

fn parse_match(m: &Match) -> i32 {
    m.as_str().parse::<i32>().expect("Failed to parse capture group")
}

fn get_vec3_named_formatted(name: &str, vec3: &Vec<i32>) -> String {
    format!("{}=<x={:>2}, y={:>2}, z={:>2}>", name, vec3[0], vec3[1], vec3[2])
}

impl Moon {

    pub fn from_string(s: &str) -> Moon {
        let re = Regex::new(r"^<x=(.*?), y=(.*?), z=(.*?)>$").unwrap();
        let pos = match re.captures(s) {
            Some(c) => vec![
                parse_match(&c.get(1).unwrap()),
                parse_match(&c.get(2).unwrap()),
                parse_match(&c.get(3).unwrap())
            ],
            None => panic!("Invalid input '{}'", s)
        };
        Moon {
            pos: pos,
            vel: vec![0, 0, 0]
        }
    }

    pub fn apply_gravity(moon1: &mut Moon, moon2: &mut Moon) {
        let mut diffs = Vec::new();
        for (p1, p2) in zip(&moon1.pos, &moon2.pos) {
            diffs.push((p2 - p1).signum());
        }

        for (idx, diff) in diffs.iter().enumerate() {
            moon1.vel[idx] += diff;
            moon2.vel[idx] -= diff;
        }
    }

    pub fn apply_forces(&mut self) {
        for (idx, v) in self.vel.iter().enumerate() {
            self.pos[idx] += v;
        }
    }

    fn get_pot_energy(&self) -> u32 {
        self.pos.iter().fold(0u32, |acc, e| acc + e.abs() as u32)
    }

    fn get_kin_energy(&self) -> u32 {
        self.vel.iter().fold(0u32, |acc, e| acc + e.abs() as u32)
    }

    pub fn get_total_energy(&self) -> u32 {
        self.get_pot_energy() * self.get_kin_energy()
    }

    pub fn print_full(&self) {
        println!("{:<25}    {:<25}",
                 get_vec3_named_formatted("pos", &self.pos),
                 get_vec3_named_formatted("vel", &self.vel));
    }
}

