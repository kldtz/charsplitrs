//! Improved Rust implementation of Don Tuggener's [CharSplit](https://github.com/dtuggener/CharSplit).
mod utils;
pub use crate::utils::CharString;
use serde::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fs;

/// CharSplitter with prefix, infix and suffix probabilities.
#[derive(Deserialize, Debug)]
pub struct CharSplitter {
    pub prefix: HashMap<String, f64>,
    pub infix: HashMap<String, f64>,
    pub suffix: HashMap<String, f64>,
}

impl CharSplitter {
    /// Creates new CharSplitter. Loads probabilities from JSON under specified path.
    pub fn new(prob_path: &str) -> Result<CharSplitter, Box<dyn Error>> {
        let content = fs::read_to_string(prob_path)?;
        let ngram_probs: CharSplitter = serde_json::from_str(&content)?;
        Ok(ngram_probs)
    }

    fn suffix_prob(&self, word: &str) -> Option<&f64> {
        self.suffix.get(word)
    }

    fn prefix_prob(&self, word: &str) -> Option<&f64> {
        self.prefix.get(word)
    }

    fn infix_prob(&self, word: &str) -> Option<&f64> {
        self.infix.get(word)
    }

    /// Splits compound into two parts based on provided ngram probabilities.
    /// 
    /// ```
    /// use charsplitrs::CharSplitter;
    /// 
    /// let splitter = CharSplitter::new("data/ngram_probs.json").unwrap();
    /// 
    /// let (left, right) = splitter.split("Haustür");
    /// assert_eq!(left, "Haus");
    /// assert_eq!(right, "tür");
    /// ```
    pub fn split<'a>(&self, word: &'a str) -> (&'a str, &'a str) {
        let (_prob, left, right) = self.find_split_indices(word)[0];
        (&word[..left], &word[right..])
    }

    fn find_split_indices(&self, word: &str) -> Vec<(f64, usize, usize)> {
        // word contains hyphen: split on last hyphen
        let idx = word.rfind('-');
        if let Some(i) = idx {
            return vec![(1f64, i, i + 1)];
        }

        let word = word.to_lowercase();
        let char_string = CharString::new(&word);

        let mut scores: Vec<(f64, usize, usize)> = Vec::new();

        for n in 3..char_string.len() - 2 {
            // likelihood of left part being suffix (independent word)
            let mut left_slice = char_string.prefix(n);
            if n > 3 {
                left_slice = cut_off_fugen_s(left_slice);
            }
            let left_slice_prob = self.suffix_prob(left_slice).unwrap_or(&-1f64);
            // likelihood of right part being prefix (independent word)
            let mut right_slice = char_string.suffix(n);
            if char_string.len() - n > 3 {
                right_slice = cut_off_fugen_s(right_slice);
            }
            let right_slice_prob = self.prefix_prob(right_slice).unwrap_or(&-1f64);
            // likelihood of compound suffix being infix (not an independent word)
            let in_slice_prob = self.compute_in_slice_prob(&char_string, n);

            let score = right_slice_prob - in_slice_prob + left_slice_prob;
            let i = char_string.char2byte(n);
            scores.push((score, i, i));
        }
        // no split candidates (for short words)
        if scores.len() == 0 {
            scores.push((0f64, char_string.num_bytes(), 0));
            return scores;
        }
        // sort scores in reverse order (highest score == best)
        scores.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
        scores
    }

    fn compute_in_slice_prob(&self, char_string: &CharString, n: usize) -> f64 {
        let mut in_slice_probs: Vec<&f64> = Vec::new();
        // iterate over all substrings of right slice with min. length 3
        for k in (n + 3)..char_string.len() + 1 {
            let in_slice = char_string.substr(n, k);
            in_slice_probs.push(self.infix_prob(in_slice).unwrap_or(&1f64));
        }
        in_slice_probs.iter().fold(1f64, |a, &b| a.min(*b))
    }
}

fn cut_off_fugen_s(word: &str) -> &str {
    if word.ends_with("ts")
        || word.ends_with("gs")
        || word.ends_with("ks")
        || word.ends_with("hls")
        || word.ends_with("ns")
    {
        // -1 because 's' is a single-byte char
        return &word[..word.len() - 1];
    }
    return &word;
}
