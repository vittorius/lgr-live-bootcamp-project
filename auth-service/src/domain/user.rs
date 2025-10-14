// The User struct should contain 3 fields. email, which is a String;
// password, which is also a String; and requires_2fa, which is a boolean.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
    pub(crate) email: String,
    pub(crate) password: String,
    requires_2fa: bool,
}

impl User {
    pub fn new(email: &str, password: &str, requires_2fa: bool) -> Self {
        Self {
            email: email.to_owned(),
            password: password.to_owned(),
            requires_2fa,
        }
    }
}
