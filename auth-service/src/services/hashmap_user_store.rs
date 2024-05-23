use std::collections::HashMap;

use crate::domain::user::User;

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
        HashmapUserStore { users: HashMap::new() }
    }
   pub fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
        // Return `UserStoreError::UserAlreadyExists` if the user already exists,
        // otherwise insert the user into the hashmap and return `Ok(())`.
        if self.users.contains_key(&user.email) {
            return Err(UserStoreError::UserAlreadyExists);
        } else {
            self.users.insert(user.email.clone(), user);
            return Ok(())
        }
        
    }

    // Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    pub fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(User::new(user.email.clone(), user.password.clone(), user.requires_2fa)),
            None => return Err(UserStoreError::UserNotFound)
        }
    }

    pub fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password == password {
                    return Ok(())
                } else {
                    return Err(UserStoreError::InvalidCredentials)
                }
            },
            None => return Err(UserStoreError::UserNotFound)
        }
    }

}

// TODO: Add unit tests for your `HashmapUserStore` implementation
#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn test_add_user() {
        
        // Given
        let user1 = User::new(
            String::from("johnwick@gmail.com"),
            String::from("********"),
            false
        );
        let user2 = User::new(
            String::from("johnwick@gmail.com"),
            String::from("********"),
            false
        );
        
        // When
        let mut user_store: HashmapUserStore = HashmapUserStore::new();
        let result1 = user_store.add_user(user1);
        let result2 = user_store.add_user(user2);

        // Then
        assert_eq!(result1.is_ok(), true);
        assert_eq!(result2.is_err(), true);

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