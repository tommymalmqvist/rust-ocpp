use std::sync::OnceLock;

use regex::Regex;
use rust_decimal::Decimal;
use validator::ValidationError;

static REGEX: OnceLock<Regex> = OnceLock::new();

/// Helper function to validate identifierString
///
/// # identfierString
/// This is a case-insensitive dataType and can only contain characters from the following
/// character set: `a-z`, `A-Z`, `0-9`, `'*'`, `'-'`, `'_'`, `'='`, `':'`, `'+'`, `'|'`, `'@'`, `'.'`
pub fn validate_identifier_string(s: &str) -> Result<(), ValidationError> {
    // regex for identifierString as defined by the specification
    let res = REGEX
        .get_or_init(|| Regex::new(r"^[a-zA-Z0-9*+=:|@._-]*$").unwrap())
        .is_match(s);

    match res {
        true => Ok(()),
        false => Err(ValidationError::new("Not a valid identifierString")),
    }
}

/// Validates that a discharge limit is non-positive (less than or equal to zero).
///
/// # Arguments
///
/// * `value` - The Decimal value to validate
///
/// # Returns
///
/// Returns Ok(()) if the value is less than or equal to zero, otherwise returns Err
pub fn validate_discharge_limit(value: &Decimal) -> Result<(), ValidationError> {
    if *value > Decimal::ZERO {
        let mut error = ValidationError::new("discharge_limit_must_be_non_positive");
        error.message = Some("Discharge limit must be less than or equal to zero".into());
        return Err(error);
    }

    Ok(())
}

#[cfg(test)]
mod test {
    use super::validate_identifier_string;

    #[test]
    fn good_case() {
        let good_cases = ["abc123", "A*C_|..", "||||", "ABCabc123:==@"];

        for case in good_cases.iter() {
            dbg!(case);
            validate_identifier_string(case).unwrap();
        }
    }

    #[test]
    fn bad_case() {
        let bad_cases = [
            "abc123/",
            "https://",
            "ABC#123",
            ",,,,",
            "Test test",
            "123 Prøve",
            "123 Test?",
        ];

        for case in bad_cases.iter() {
            dbg!(case);
            validate_identifier_string(case).unwrap_err();
        }
    }
}
