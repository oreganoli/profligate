#[cfg(test)]
mod tests;

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
