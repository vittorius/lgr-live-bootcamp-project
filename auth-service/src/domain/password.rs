#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: &str) -> Result<Self, String> {
        if password.len() >= 8 {
            Ok(Self(String::from(password)))
        } else {
            Err("Failed to parse string to a Password type".to_owned())
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_password() {
        let password = "password123";
        let result = Password::parse(password);
        assert!(result.is_ok());
        let parsed_password = result.unwrap();
        assert_eq!(parsed_password.as_ref(), "password123");
    }

    #[test]
    fn test_parse_minimum_valid_password() {
        let password = "12345678";
        let result = Password::parse(password);
        assert!(result.is_ok());
        let parsed_password = result.unwrap();
        assert_eq!(parsed_password.as_ref(), "12345678");
    }

    #[test]
    fn test_parse_too_short_password() {
        let password = "1234567";
        let result = Password::parse(password);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_empty_password() {
        let password = "";
        let result = Password::parse(password);
        assert!(result.is_err());
    }
}
