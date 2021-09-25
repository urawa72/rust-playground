use std::{collections::HashMap, io::BufRead};

use regex::Regex;

pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").expect("");
    let mut freqs = HashMap::new();

    for line in input.lines() {
        let line = line.unwrap();
        use crate::CountOption::*;
        match option {
            Char => {
                for c in line.chars() {
                    *freqs.entry(c.to_string()).or_insert(0) += 1;
                }
            }
            Word => {
                for m in re.find_iter(&line) {
                    let word = m.as_str().to_string();
                    *freqs.entry(word).or_insert(0) += 1;
                }
            }
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1
        }
    }
    freqs
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    Char,
    Word,
    Line,
}

impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}
