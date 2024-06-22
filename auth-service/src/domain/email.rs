use color_eyre::eyre::{eyre, Result};
use secrecy::{ExposeSecret, Secret};
use std::hash::Hash;
use validator::validate_email;

#[derive(Debug, Clone)]
pub struct Email(Secret<String>);

impl PartialEq for Email {
    fn eq(&self, other: &Self) -> bool {
        self.0.expose_secret() == other.0.expose_secret()
    }
}

impl Eq for Email {}

impl Email {
    pub fn parse(s: Secret<String>) -> Result<Email> {
        if validate_email(s.expose_secret()) {
            Ok(Self(s))
        } else {
            Err(eyre!(format!("{} is not a valid email.", s.expose_secret())))
        }
    }
}

impl AsRef<Secret<String>> for Email {
    fn as_ref(&self) -> &Secret<String> {
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
    use secrecy::Secret;

    #[test]
    fn should_reject_empty_email() {
        let email = Secret::new("".to_string());
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn should_reject_email_without_at_character() {
        let email = Secret::new("usergmail.com".to_string());
        assert!(Email::parse(email).is_err());
    }

    #[test]
    fn should_reject_email_without_user() {
        let email = Secret::new("@protonmail.com".to_string());
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
        Email::parse(Secret::new(valid_email.0)).is_ok()
    }

}