use super::{encrypt, CaesarError};

const HORSE_FABLE: &str = "This having heard, the sheep fled into the plains.";
/// This string contains UTF-8 characters that should trigger an error.
const HORSE_FABLE_UTF: &str = "To usłyszawszy, owca uciekła na równinę.";
#[test]
fn shift_by_one() {
    let mut text = HORSE_FABLE.to_owned();
    encrypt(&mut text, 1).unwrap();
    assert_eq!(text, "Uijt ibwjoh ifbse, uif tiffq gmfe joup uif qmbjot.")
}
#[test]
fn non_ascii_error() {
    match encrypt(&mut HORSE_FABLE_UTF.to_owned(), 8) {
        Err(CaesarError::NonAscii) => (),
        _ => panic!(),
    }
}
