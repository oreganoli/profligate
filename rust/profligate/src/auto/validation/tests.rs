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
