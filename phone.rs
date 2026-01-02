use anyhow::{anyhow, Result};
use regex::Regex;

pub struct PhoneFormatter {
    area: String,
    prefix: String,
    line: String,
    country: String,
}

impl PhoneFormatter {
    pub fn new(phone_number: &str) -> Result<Self> {
        // Strip non-digit characters
        let digits: String = phone_number.chars().filter(|c| c.is_numeric()).collect();

        let (area, prefix, line, country) = if digits.len() == 11 && digits.starts_with('1') {
            (
                digits[1..4].to_string(),
                digits[4..7].to_string(),
                digits[7..11].to_string(),
                "1".to_string(),
            )
        } else if digits.len() == 10 {
            (
                digits[0..3].to_string(),
                digits[3..6].to_string(),
                digits[6..10].to_string(),
                "1".to_string(),
            )
        } else {
            return Err(anyhow!("Error: Phone number must be 10 or 11 digits"));
        };

        Ok(PhoneFormatter {
            area,
            prefix,
            line,
            country,
        })
    }

    pub fn generate_formats(&self) -> Vec<String> {
        vec![
            // Format 1: 555-555-1212
            format!("{}-{}-{}", self.area, self.prefix, self.line),
            // Format 2: (555) 555-1212
            format!("({}) {}-{}", self.area, self.prefix, self.line),
            // Format 3: 5555551212
            format!("{}{}{}", self.area, self.prefix, self.line),
            // Format 4: 1 555-555-1212
            format!("{} {}-{}-{}", self.country, self.area, self.prefix, self.line),
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_10_digit_number() {
        let formatter = PhoneFormatter::new("5555551212").unwrap();
        let formats = formatter.generate_formats();
        assert_eq!(formats.len(), 4);
        assert_eq!(formats[0], "555-555-1212");
        assert_eq!(formats[1], "(555) 555-1212");
        assert_eq!(formats[2], "5555551212");
        assert_eq!(formats[3], "1 555-555-1212");
    }

    #[test]
    fn test_11_digit_number() {
        let formatter = PhoneFormatter::new("15555551212").unwrap();
        let formats = formatter.generate_formats();
        assert_eq!(formats[0], "555-555-1212");
    }

    #[test]
    fn test_formatted_input() {
        let formatter = PhoneFormatter::new("(555) 555-1212").unwrap();
        let formats = formatter.generate_formats();
        assert_eq!(formats[0], "555-555-1212");
    }

    #[test]
    fn test_invalid_length() {
        let result = PhoneFormatter::new("123");
        assert!(result.is_err());
    }
}
