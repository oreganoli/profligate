#[cfg(test)]
use super::*;
#[cfg(test)]
use super::{analysis::ENGLISH_FREQ_TABLE, validation::*};

#[test]
fn auto_with_crib() {
    let mut text = "The quick brown fox jumps over the lazy dog.".to_owned();
    encrypt(&mut text, 22).unwrap();
    let mut crib = CribValidator::new("quick");
    let res = auto_decrypt_caesar(&mut text, &ENGLISH_FREQ_TABLE, &crib);
    assert_eq!(res, Ok(4));

    crib = CribValidator::new("sesquipedalian");
    text = "The quick brown fox jumps over the lazy dog.".to_owned();
    encrypt(&mut text, -4).unwrap();
    let res = auto_decrypt_caesar(&mut text, &ENGLISH_FREQ_TABLE, &crib);
    assert_eq!(res, Err(CaesarError::PlaintextInvalid));
}

#[test]
fn auto_with_list() {
    // most of these words actually are in the English dictionary we're using
    let mut text =
        "Haec verba sunt tam in dictionario linguae anglicae, quam in linguae Latinae".to_owned();
    // the default threshold should not recognize this as English
    let eng = WordListValidator::new(WordList::new(ENGLISH_WORDS));
    encrypt(&mut text, 1).unwrap();
    let res = auto_decrypt_caesar(&mut text, &ENGLISH_FREQ_TABLE, &eng);
    assert_eq!(res, Err(CaesarError::PlaintextInvalid));
    // a lower one might
    let eng = WordListValidator::with_threshold(WordList::new(ENGLISH_WORDS), 0.5);
    let res = auto_decrypt_caesar(&mut text, &ENGLISH_FREQ_TABLE, &eng);
    assert!(res.is_ok());
}
