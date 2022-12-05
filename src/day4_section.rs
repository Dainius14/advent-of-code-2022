use std::{str::FromStr, fmt::Error};

pub struct Section {
    from: i32,
    to: i32,
}

impl Section {
    pub fn overlaps_with(&self, other: &Section) -> bool {
        self.from >= other.from && self.from <= other.to
            && self.to >= other.from && self.to <= other.to 
    }

    pub fn partially_overlaps_with(&self, other: &Section) -> bool {
        self.from >= other.from && self.from <= other.to
            || self.to >= other.from && self.to <= other.to 
    }
}

impl FromStr for Section {
    type Err = Error;

    fn from_str(str: &str) -> Result<Self, Self::Err> {
        let sep_index = str.find('-').unwrap();
        let (from_str, to_str) = str.split_at(sep_index);

        let from: i32 = from_str.parse().unwrap();
        let to: i32 = to_str[1..].parse().unwrap();

        Ok(Section { from, to })
    }

}