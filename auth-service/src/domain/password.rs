use validator::HasLen;

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
pub struct Password(String);

impl Password {
    pub fn parse(password: String) -> Result<Password, String> {
        if password.length() >= 8 {
            Ok(Self(password))
        } else {
            Err(format!("{} is not a valid password.", password))
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}