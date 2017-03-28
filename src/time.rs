use std::cmp::Ordering;
use std::str::FromStr;
use std::num::ParseIntError;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Time {
    pub hour: u32,
    pub min: u32,
}

impl Ord for Time {
    fn cmp(&self, other: &Time) -> Ordering {
        let selfmins = &self.hour * 60 + &self.min;
        let othermins = other.hour * 60 + other.min;
        selfmins.cmp(&othermins)
    }
}

impl PartialOrd for Time {
    fn partial_cmp(&self, other: &Time) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl FromStr for Time {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.parse::<u32>().map(|v|
            Time { hour: v / 100, min: v % 100 }
        )
    }
}
