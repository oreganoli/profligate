use std::collections::HashSet;
pub const ENGLISH_WORDS: &str = include_str!("validation/words_alpha.txt");

#[cfg(test)]
mod tests;
/// This trait is responsible for judging whether or not a possible decrypted plaintext is correct.
pub trait Validator {
    fn validate(&self, text: &str) -> bool;
}
/// This type of validator checks if the decrypted plaintext contains a known phrase, ie. a crib.
pub struct CribValidator {
    crib: String,
}
impl CribValidator {
    pub fn new<S: Into<String>>(crib: S) -> Self {
        Self { crib: crib.into() }
    }
    pub fn set_crib<S: Into<String>>(&mut self, crib: S) {
        self.crib = crib.into();
    }
}
impl Validator for CribValidator {
    fn validate(&self, text: &str) -> bool {
        text.contains(&self.crib)
    }
}

/// A validator that checks the plaintext against a word list with a given threshold.
pub struct WordListValidator<'a> {
    /// The word list.
    list: WordList<'a>,
    /// The minimum ratio of recognized words. Must be within (0; 1]. By default, plaintext is considered valid if 75% of its words are contained in the word list, ie. the threshold is 0.75.
    threshold: f32,
}
impl<'a> WordListValidator<'a> {
    pub fn new(list: WordList<'a>) -> Self {
        Self {
            list,
            threshold: 0.75,
        }
    }
    /// Constructor with custom threshold. If an invalid (not within (0; 1]) value of `threshold` is given, will default to 0.75.
    pub fn with_threshold(list: WordList<'a>, threshold: f32) -> Self {
        Self {
            list,
            threshold: if threshold > 0.0 && threshold <= 1.0 {
                threshold
            } else {
                0.75
            },
        }
    }
    /// Adjust threshold. Values outside of (0; 1] will be discarded.
    pub fn set_threshold(&mut self, threshold: f32) {
        if threshold > 0.0 && threshold <= 1.0 {
            self.threshold = threshold;
        }
    }
}
impl<'a> Validator for WordListValidator<'a> {
    fn validate(&self, text: &str) -> bool {
        let mut total_words = 0f32;
        let mut valid_words = 0f32;
        text.split(|char| match char {
            'a'..='z' | 'A'..='Z' => false,
            // split on characters that aren't ASCII letters
            _ => true,
        })
        .for_each(|word| {
            total_words += 1.0;
            if self.list.contains(word) {
                valid_words += 1.0;
            }
        });
        valid_words / total_words >= self.threshold
    }
}
/// A word list utilizing a hash set under the hood.
pub struct WordList<'a> {
    set: HashSet<&'a str>,
}
impl<'a> WordList<'a> {
    /// Creates a word list based on a newline-delimited string.
    pub fn new(raw_list: &'a str) -> Self {
        let mut set = HashSet::new();
        for word in raw_list.lines() {
            set.insert(word);
        }
        Self { set }
    }
    fn contains(&self, word: &str) -> bool {
        self.set.contains(word)
    }
}
