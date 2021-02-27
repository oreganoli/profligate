#[cfg(test)]
use super::*;

#[test]
fn crib_validation() {
    let val_1 = CribValidator::new("This phrase turns up in the plaintext");
    let val_2 = CribValidator::new("This one doesn't");
    let plaintext = "Here's a string of plaintext for validation: This phrase turns up in the plaintext, the other one doesn't.";
    assert!(val_1.validate(plaintext));
    assert!(!val_2.validate(plaintext));
}

#[test]
fn english_validation() {
    let text_1 = "A quick brown fox jumps over the lazy dog";
    let text_2 = "In hac scriptura sunt tantum litterae anglicae, sed Latine scripta est.";
    let eng_validator = WordListValidator::new(WordList::new(ENGLISH_WORDS));
    assert!(eng_validator.validate(text_1));
    assert!(!eng_validator.validate(text_2));
}
