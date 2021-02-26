mod tests;

const ALPHABET_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_LENGTH: u8 = ALPHABET_LOWER.len() as u8;
/// Error type for Caesar cipher encryption and decryption.
#[derive(Debug)]
pub enum CaesarError {
    // The string being operated on isn't valid ASCII.
    NonAscii,
}
/// Encrypt a string of text with the given key. This operation happens in place to cut down on allocations.
pub fn encrypt(text: &mut str, key: i8) -> Result<(), CaesarError> {
    if text.chars().find(|c| !c.is_ascii()).is_some() {
        return Err(CaesarError::NonAscii);
    }
    let bytes = unsafe {
        // We've already errored out if the text isn't ASCII, so manipulating its bytes in place like a C *char should be safe.
        text.as_bytes_mut()
    };
    let shift = key % ALPHABET_LENGTH as i8;
    dbg!(shift);
    for byte in bytes {
        match byte {
            b'a'..=b'z' => {
                *byte = ((*byte as i8 + shift) as u8 % b'a') + b'a';
            }
            b'A'..=b'Z' => {
                *byte = ((*byte as i8 + shift) as u8 % b'A') + b'A';
            }
            _ => (),
        };
    }
    Ok(())
}
