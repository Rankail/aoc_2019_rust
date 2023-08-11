use std::hash::{Hash, Hasher};
use regex::{Match, Regex};

#[derive(Clone)]
pub struct MoonPartial {
    pos: i32,
    vel: i32
}

impl PartialEq for MoonPartial {
    fn eq(&self, other: &MoonPartial) -> bool {
        self.pos == other.pos
    }
}

impl Eq for MoonPartial {}

impl Hash for MoonPartial {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pos.hash(state);
        self.vel.hash(state);
    }
}

fn parse_match(m: &Match) -> i32 {
    m.as_str().parse::<i32>().expect("Failed to parse capture group")
}

impl MoonPartial {

    pub fn from_str(s: &str) -> (MoonPartial, MoonPartial, MoonPartial) {
        let re = Regex::new(r"^<x=(.*?), y=(.*?), z=(.*?)>$").unwrap();
        let (x, y, z) = match re.captures(s) {
            Some(c) => (
                parse_match(&c.get(1).unwrap()),
                parse_match(&c.get(2).unwrap()),
                parse_match(&c.get(3).unwrap())
            ),
            None => panic!("Invalid input '{}'", s)
        };

        (
            MoonPartial{ pos: x, vel: 0 },
            MoonPartial{ pos: y, vel: 0 },
            MoonPartial{ pos: z, vel: 0 }
        )
    }

    pub fn apply_gravity(m1: &mut MoonPartial, m2: &mut MoonPartial) {
        let diff = (m2.pos - m1.pos).signum();
        m1.vel += diff;
        m2.vel -= diff;
    }

    pub fn apply_forces(&mut self) {
        self.pos += self.vel;
    }

}