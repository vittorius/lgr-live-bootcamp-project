use crate::domain::{email::Email, password::Password};

#[derive(Clone, Debug, PartialEq)]
pub struct User {
    pub(crate) email: Email,
    pub(crate) password: Password,
    pub(crate) requires_2fa: bool,
}

impl User {
    pub fn new(email: Email, password: Password, requires_2fa: bool) -> Self {
        Self {
            email,
            password,
            requires_2fa,
        }
    }
}
