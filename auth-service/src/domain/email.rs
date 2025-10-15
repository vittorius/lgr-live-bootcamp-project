use email_address::{EmailAddress, Options};

#[derive(PartialEq, Eq, Clone, Debug, Hash)]
pub struct Email(String);

#[derive(Debug)]
pub enum EmailError {
    Invalid,
}

impl Email {
    pub fn parse(email: &str) -> Result<Self, EmailError> {
        if let Ok(email_address) =
            EmailAddress::parse_with_options(email, Options::default().with_required_tld())
        {
            Ok(Email(String::from(email_address.as_str())))
        } else {
            Err(EmailError::Invalid)
        }
    }
}

impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_valid_email() {
        let email = Email::parse("test@example.com");
        assert!(email.is_ok());
    }

    #[test]
    fn test_parse_invalid_email() {
        let email = Email::parse("invalid-email");
        assert!(email.is_err());
    }

    #[test]
    fn test_parse_missing_required_tld() {
        let email = Email::parse("test@example");
        assert!(email.is_err());
    }

    #[test]
    fn test_parse_missing_tld_after_dot() {
        let email = Email::parse("test@example.");
        assert!(email.is_err());
    }

    #[test]
    fn test_parse_missing_local_part_and_delimiter() {
        let email = Email::parse("example.com");
        assert!(email.is_err());
    }

    #[test]
    fn test_parse_missing_local_part() {
        let email = Email::parse("@example.com");
        assert!(email.is_err());
    }
}
