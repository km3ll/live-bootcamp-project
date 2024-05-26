use validator::validate_email;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Email(String);

impl Email {
    pub fn parse(email: String) -> Result<Email, String> {
        if email.is_empty() {
            Err("Email must not be empty".to_owned())
        } else if validate_email(&email) {
            Ok(Self(email))
        } else {
            Err(format!("{} is not a valid email.", email))
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

    use super::Email;
    use fake::{
        faker::internet::en::SafeEmail,
        Fake
    };
    use quickcheck::Gen;

    #[test]
    fn should_reject_empty_email() {
        let email = "".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn should_reject_email_without_at_character() {
        let email = "usergmail.com".to_string();
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn should_reject_email_without_user() {
        let email = "@protonmail.com".to_string();
        assert!(Email::parse(email).is_err());
    }

    // Bonus
    #[derive(Debug, Clone)]
    struct ValidEmailFixture(pub String);

    impl quickcheck::Arbitrary for ValidEmailFixture {
        fn arbitrary<G: Gen>(generator: &mut G) -> Self {
            let email = SafeEmail().fake_with_rng(generator);
            Self(email)
        }
    }

    #[quickcheck_macros::quickcheck]
    fn valid_emails_are_parsed(valid_email: ValidEmailFixture) -> bool {
        Email::parse(valid_email.0).is_ok()
    }

}