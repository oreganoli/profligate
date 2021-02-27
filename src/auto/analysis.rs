#[cfg(test)]
mod tests;
/// A list of the lowercase letters of a language, sorted by frequency in descending order.
pub struct FreqTable<'a>(pub &'a [u8]);
/// The letter frequency table for English, as given by Wikipedia.
pub const ENGLISH_FREQ_TABLE: FreqTable<'static> = FreqTable(b"etaoinshrdlcumwfgypbvkjxqz");

/// Returns the most frequently occurring character in the given string. Case-insensitive, returns a lowercase char; assumes the text is ASCII.
/// Behaves in a deterministic way - should two or more Latin letters be equally frequent, the first one will be assumed.
pub fn most_frequent_char(text: &str) -> u8 {
    let mut table = [0; 26];
    let bytes = text.as_bytes();
    for byte in bytes {
        match *byte {
            b'a'..=b'z' => table[(*byte - b'a') as usize] += 1,
            b'A'..=b'Z' => table[(*byte - b'A') as usize] += 1,
            _ => (),
        }
    }
    let mut max = 0;
    let mut max_index = 0;
    for i in 0..table.len() {
        if table[i] > max {
            max = table[i];
            max_index = i as u8;
        }
    }
    max_index + b'a'
}
/// For a given character of an alphabet, computes a table of Caesarean cipher shifts
/// that would make each corresponding character in the frequency table match the given character.
pub fn compute_shifts(char: u8, freq_table: &FreqTable) -> Vec<i8> {
    // All we really do here is subtraction.
    freq_table
        .0
        .iter()
        .map(|each| char as i8 - *each as i8)
        .collect()
}
