use std::collections::HashMap;

use crate::domain::{user::User, data_stores::UserStoreError, UserStore};

// Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<String, User>,
}

impl HashmapUserStore {
    pub fn new() -> HashmapUserStore {
        HashmapUserStore { users: HashMap::new() }
    }
}

#[async_trait::async_trait]
impl UserStore for HashmapUserStore {

   async fn add_user(&mut self, user: User) -> Result<(), UserStoreError> {
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
    async fn get_user(&self, email: &str) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(User::new(user.email.clone(), user.password.clone(), user.requires_2fa)),
            None => return Err(UserStoreError::UserNotFound)
        }
    }

   async fn validate_user(&self, email: &str, password: &str) -> Result<(), UserStoreError> {
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
    async fn user_store_should_add_user_successfully() {
        
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
        let result1 = user_store.add_user(user1).await;
        let result2 = user_store.add_user(user2).await;

        // Then
        assert!(result1.is_ok());
        assert!(result2.is_err());

    }
    
    #[tokio::test]
    async fn user_store_should_return_user_successfully() {

        // Given
        let email = String::from("johnwick@gmail.com");
        let user = User::new(
            email.clone(),
            String::from("********"),
            false
        );
        let mut user_store: HashmapUserStore = HashmapUserStore::new();
        user_store.users.insert(email.clone(), user.clone());
        // When-Then
        let result = user_store.get_user(&email).await;
        assert_eq!(result, Ok(user));

    }

    #[tokio::test]
    async fn test_validate_user() {
        // Given
        let email = String::from("johnwick@gmail.com");
        let password = String::from("********");
        let user = User::new(
            email.clone(),
            password.clone(),
            false
        );
        let mut user_store: HashmapUserStore = HashmapUserStore::new();
        let result1 = user_store.add_user(user).await;
        let result2 = user_store.validate_user(email.as_str(), password.as_str()).await;
        assert!(result2.is_ok())
    }

}