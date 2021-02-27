#[cfg(test)]
mod tests;
/// This trait is responsible for judging whether or not a possible decrypted plaintext is correct.
pub trait Validator {
    fn validate(&self, text: &str) -> bool;
}
/// This type of validator checks if the decrypted plaintext contains a known phrase, ie. a crib.
pub struct CribValidator {
    crib: String,
}
impl CribValidator {
    pub fn new<S: Into<String>>(crib: S) -> Self {
        Self { crib: crib.into() }
    }
}
impl Validator for CribValidator {
    fn validate(&self, text: &str) -> bool {
        text.contains(&self.crib)
    }
}
