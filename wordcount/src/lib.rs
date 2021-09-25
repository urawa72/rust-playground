//! wordcount is simple counter which counts the frequency of characteres, words, and lines.
//! see [`count`](fn.count.html) for detail
use std::{collections::HashMap, io::BufRead};

use regex::Regex;

/// [`count`](fn.count.html) options.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// frequency of characteres
    Char,
    /// frequency of words
    Word,
    /// frequency of lines
    Line,
}

/// option default is [`Word`](enum.CountOption.html#wariant.word).
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// read th UTF-8 string line by line from input and count the frequency.
///
/// the target for countintg the frequency is controlled by the option.
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Unicode character by character
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): For each word which matches the regex \w+
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): For each line separated by \n or \r\n
///
/// # Panics
///
/// Panic if input is not formatted in UTF-8
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
            Line => *freqs.entry(line.to_string()).or_insert(0) += 1,
        }
    }
    freqs
}

#[test]
fn word_count_works() {
    use std::io::Cursor;

    let mut exp = HashMap::new();
    exp.insert("aa".to_string(), 1);
    exp.insert("bb".to_string(), 2);
    exp.insert("cc".to_string(), 1);

    assert_eq!(count(Cursor::new("aa bb cc bb"), CountOption::Word), exp);
}
