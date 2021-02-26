mod tests;

const ALPHABET_LOWER: &str = "abcdefghijklmnopqrstuvwxyz";
const ALPHABET_UPPER: &str = "ABCDEFGHIJKLMNOPQRSTUWXYZ";
const ALPHABET_LENGTH: usize = ALPHABET_LOWER.len();
/// Error type for Caesar cipher encryption and decryption.
#[derive(Debug)]
enum CaesarError {
    // The string being operated on isn't valid ASCII.
    NonAscii,
}
/// Encrypt a string of text with the given key.
fn encrypt(text: &str, key: u32) -> Result<String, CaesarError> {
    if text.chars().find(|c| !c.is_ascii()).is_some() {
        return Err(CaesarError::NonAscii);
    }
    unimplemented!()
}
