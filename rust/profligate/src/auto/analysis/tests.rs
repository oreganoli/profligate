#[cfg(test)]
use super::*;

#[test]
fn most_frequent() {
    let text_1 = "The eagle flew high.";
    let text_2 = "THIS IS A test StRING wiTh mixed case for tesTing";
    let text_3 = "BLOOD! GORE! SHOUTS! LOUD!";
    let text_4 = "aaabbbbbbbbbaaaa";
    let text_5 = "abba";
    assert_eq!(most_frequent_char(&text_1), b'e');
    assert_eq!(most_frequent_char(&text_2), b't');
    assert_eq!(most_frequent_char(&text_3), b'o');
    assert_eq!(most_frequent_char(&text_4), b'b');
    assert_eq!(most_frequent_char(&text_5), b'a');
}
