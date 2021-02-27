mod tests;
const ALPHABET_LENGTH: i16 = 26;
/// Error type for Caesar cipher encryption and decryption.
#[derive(Debug)]
pub enum CaesarError {
    // The string being operated on isn't valid ASCII.
    NonAscii,
}
/// Encrypt a string of text with the given key. This operation happens in place to cut down on allocations.
pub fn encrypt(text: &mut str, key: i16) -> Result<(), CaesarError> {
    if text.chars().find(|c| !c.is_ascii()).is_some() {
        return Err(CaesarError::NonAscii);
    }
    let shift = key % ALPHABET_LENGTH;
    let bytes = unsafe {
        // We've already errored out if the text isn't ASCII, so manipulating its bytes in place like those of a C *char should be safe.
        text.as_bytes_mut()
    };
    for byte in bytes {
        match byte {
            b'a'..=b'z' => {
                let old_pos = (*byte - b'a') as i16 % ALPHABET_LENGTH;
                //dbg!(old_pos);
                let pos = (old_pos as i16 + shift) % ALPHABET_LENGTH;
                //dbg!(pos);
                if pos >= 0 {
                    *byte = pos as u8 + b'a';
                } else {
                    *byte = b'z' - (-pos) as u8 + 1;
                }
            }
            b'A'..=b'Z' => {
                let old_pos = (*byte - b'A') as i16 % ALPHABET_LENGTH;
                //dbg!(old_pos);
                let pos = (old_pos as i16 + shift) % ALPHABET_LENGTH;
                //dbg!(pos);
                if pos >= 0 {
                    *byte = pos as u8 + b'A';
                } else {
                    *byte = b'Z' - (-pos) as u8 + 1;
                }
            }
            _ => continue,
        }
    }
    Ok(())
}
/// This function applies a Caesar cipher to a raw ASCII byte within the English alphabet. It panics on non-letter bytes.
/// Note that the ASCII range is within the lower half of a `u8`, and thus also within the positive half of an `i8`.
fn shift_byte(byte: u8, shift: i8) -> u8 {
    match byte {
        b'a'..=b'z' => {
            if shift >= 0 {
                ((byte as i8 + shift) as u8 % b'a') + b'a'
            } else {
                (b'z' as i8 + shift + (byte - b'a') as i8) as u8
            }
        }
        b'A'..=b'Z' => {
            if shift >= 0 {
                ((byte as i8 + shift) as u8 % b'A') + b'A'
            } else {
                (b'Z' as i8 + shift + (byte - b'A') as i8) as u8
            }
        }
        _ => panic!("The byte being shifted isn't an ASCII letter."),
    }
}
