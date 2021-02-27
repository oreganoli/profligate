#[cfg(test)]
use super::*;

#[test]
fn most_frequent() {
    let text_1 = "The eagle flew high.";
    let text_2 = "THIS IS A test StRING wiTh mixed case for tesTing";
    assert_eq!(most_frequent_char(&text_1), b'e');
    assert_eq!(most_frequent_char(&text_2), b't');
}
