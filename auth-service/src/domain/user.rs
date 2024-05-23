struct User {
    email: String,
    password: String,
    requires_2fa: bool,
}

impl User {
    fn new(email: String, password: String, requires_2fa: String ) -> User {
        User { email, password, required_2fa }
    }
}