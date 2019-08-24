//! wordcount provides a simple count function for the appearance frequency of characters, words and lines.
//! See the [`count`] (fn.count.html) function documentation for details.

use regex::Regex;
use std::collections::HashMap;
use std::io::BufRead;

/// Options used in [`count`](fn.count.html)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CountOption {
    /// count frequency for each character
    Char,
    /// count frequency for each word
    Word,
    /// count frequency per line
    Line,
}

/// The default option is [`Word`] (enum.CountOption.html # variant.Word)
impl Default for CountOption {
    fn default() -> Self {
        CountOption::Word
    }
}

/// Read UTF-8 string line by line from `input` and count frequency.
///
/// The frequency counting target is controlled by options.
/// * [`CountOption::Char`](enum.CountOption.html#variant.Char): Count frequency for each Unicode character
/// * [`CountOption::Word`](enum.CountOption.html#variant.Word): Count frequency for each word that matches the regular expression` \ w + `
/// * [`CountOption::Line`](enum.CountOption.html#variant.Line): Count frequency per line separated by` \ n` or `\ r \ n`
///
/// # Examples
/// Example of counting the frequency of occurrence of words during input
///
/// ```
/// use std::io::Cursor;
/// use bicycle_book_wordcount::{count, CountOption};
///
/// let mut input = Cursor::new("aa bb cc bb");
/// let freq = count(input, CountOption::Word);
///
/// assert_eq!(freq["aa"], 1);
/// assert_eq!(freq["bb"], 2);
/// assert_eq!(freq["cc"], 1);
/// ```
///
/// # Panics
///
/// Panic if input is not formatted in UTF-8.
pub fn count(input: impl BufRead, option: CountOption) -> HashMap<String, usize> {
    let re = Regex::new(r"\w+").unwrap();
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