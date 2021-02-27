#[cfg(test)]
use super::*;

#[cfg(test)]
const HORSE_FABLE: &str = "This having heard, the sheep fled into the plains.";
/// This string contains UTF-8 characters that should trigger an error.
#[cfg(test)]
const HORSE_FABLE_UTF: &str = "To usłyszawszy, owca uciekła na równinę.";
#[test]
fn shift_by_one() {
    let mut text = HORSE_FABLE.to_owned();
    encrypt(&mut text, 1).unwrap();
    assert_eq!(text, "Uijt ibwjoh ifbse, uif tiffq gmfe joup uif qmbjot.")
}
#[test]
fn shift_by_minus_four() {
    let mut text = HORSE_FABLE.to_owned();
    encrypt(&mut text, -4).unwrap();
    assert_eq!(text, "Pdeo dwrejc dawnz, pda odaal bhaz ejpk pda lhwejo.")
}

#[test]
fn wraparound() {
    let mut text_1 = HORSE_FABLE.to_owned();
    let mut text_2 = text_1.clone();
    encrypt(&mut text_1, -4).unwrap();
    encrypt(&mut text_2, 22).unwrap();
    assert_eq!(text_1, text_2);
    encrypt(&mut text_1, 34).unwrap();
    encrypt(&mut text_2, 8).unwrap();
    assert_eq!(text_1, text_2);
}

#[test]
fn decryption() {
    let mut text = HORSE_FABLE.to_owned();
    encrypt(&mut text, 8).unwrap();
    encrypt(&mut text, -8).unwrap();
    assert_eq!(text, HORSE_FABLE);
    encrypt(&mut text, 34).unwrap();
    assert_ne!(text, HORSE_FABLE);
    decrypt(&mut text, 34).unwrap();
    assert_eq!(text, HORSE_FABLE);
}

#[test]
fn non_ascii_error() {
    match encrypt(&mut HORSE_FABLE_UTF.to_owned(), 8) {
        Err(CaesarError::NonAscii) => (),
        _ => panic!(),
    }
}
