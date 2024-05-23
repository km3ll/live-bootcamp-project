use std::collections::HashMap;

use crate::domain::User;

#[derive(Debug, PartialEq)]
pub enum UserStoreError {
    UserAlreadyExists,
    UserNotFound,
    InvalidCredentials,
    UnexpectedError,
}

// Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
struct HashmapUserStore {
    users: HashMap<String, User>,
}


impl HashmapUserStore {
    
    pub fn new() -> HashmapUserStore {
        HashmapUserStore { users: hashmap::new() }
    }

    pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(user.password) {
            return Err(UserStoreError::UserAlreadyExists);
        } else {
            self.users.insert(user.password, user);
        }
        
    }

    // Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(self, email: &str) -> Result<User, UserStoreError> {
        match selft.users.get(email) {
            Some(user) => ok(user),
            None => return Error(UserStoreError::UserNotFound)
        }
    }

    pub fn validate_user(self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match selft.users.get(email) {
            Some(user) => {
                if user.password == passworsd {
                    return Ok(())
                } else {
                    return Err(UserStoreError::InvalidCredentials)
                }
            },
            None => return Error(UserStoreError::UserNotFound)
        }
    }

}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    fn test_add_user() {
        // Given
        let user = User::new("");
        let user_store = HashmapUserStore::new();

        todo!()
    }
    /*
    #[tokio::test]
    fn test_get_user() {
        todo!()
    }

    #[tokio::test]
    fn test_validate_user() {
        todo!()
    }
    */

}