#[cfg(test)]
mod tests;

/// Letter frequency analysis.
pub mod analysis;
/// Guess validation.
pub mod validation;

use crate::caesar::*;
use analysis::*;
use validation::*;

/// Attempt to automatically decrypt a Caesar-ciphered string of text given a language's letter frequency table and a validator.
/// In the successful case, returns the amount of attempts.
pub fn auto_decrypt_caesar(
    text: &mut str,
    freq_table: &FreqTable,
    validator: &dyn Validator,
) -> Result<u8, CaesarError> {
    let most_frequent = most_frequent_char(text);
    let shifts = compute_shifts(most_frequent, freq_table);
    let mut iters = 0;
    for shift in shifts {
        iters += 1;
        decrypt(text, shift as i16)?;
        if validator.validate(text) {
            return Ok(iters);
        } else {
            encrypt(text, shift as i16)?; // Undo the decryption if the deciphered plaintext is invalid.
        }
    }
    Err(CaesarError::PlaintextInvalid)
}
