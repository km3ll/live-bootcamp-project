use std::collections::HashMap;

use crate::domain::{user::User, data_stores::UserStoreError, UserStore, Email, Password};

// Create a new struct called `HashmapUserStore` containing a `users` field
// which stores a `HashMap`` of email `String`s mapped to `User` objects.
// Derive the `Default` trait for `HashmapUserStore`.
#[derive(Default)]
pub struct HashmapUserStore {
    users: HashMap<Email, User>,
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
        }
        self.users.insert(user.email.clone(), user);
        return Ok(())

    }

    // Implement a public method called `get_user`, which takes an
    // immutable reference to self and an email string slice as arguments.
    // This function should return a `Result` type containing either a
    // `User` object or a `UserStoreError`.
    // Return `UserStoreError::UserNotFound` if the user can not be found.
    async fn get_user(&self, email: &Email) -> Result<User, UserStoreError> {
        match self.users.get(email) {
            Some(user) => Ok(user.clone()),
            None => Err(UserStoreError::UserNotFound),
        }
    }

   async fn validate_user(
       &self,
       email: &Email,
       password: &Password
   ) -> Result<(), UserStoreError> {
        match self.users.get(email) {
            Some(user) => {
                if user.password.eq(password) {
                    Ok(())
                } else {
                    Err(UserStoreError::InvalidCredentials)
                }
            },
            None => Err(UserStoreError::UserNotFound)
        }
   }

}

#[cfg(test)]
mod tests {

    use super::*;

    #[tokio::test]
    async fn user_store_should_add_user() {
        
        // Given
        let mut user_store: HashmapUserStore = HashmapUserStore::default();
        let user = User {
            email: Email::parse("johnwick@gmail.com".to_owned()).unwrap(),
            password: Password::parse("********".to_owned()).unwrap(),
            requires_2fa: false
        };

        // When-Then
        let result = user_store.add_user(user.clone()).await;
        assert!(result.is_ok());

        let result = user_store.add_user(user.clone()).await;
        assert_eq!(result, Err(UserStoreError::UserAlreadyExists));

    }
    
    #[tokio::test]
    async fn user_store_should_return_user() {

        // Given
        let mut user_store: HashmapUserStore = HashmapUserStore::default();
        let email = Email::parse("johnwick@gmail.com".to_owned()).unwrap();

        let user = User {
            email: email.clone(),
            password: Password::parse("********".to_owned()).unwrap(),
            requires_2fa: false
        };

        user_store.users.insert(email.clone(), user.clone());

        // When-Then
        let result = user_store.get_user(&email).await;
        assert_eq!(result, Ok(user));

        let random_email = Email::parse("test@gmail.com".to_owned()).unwrap();
        let result = user_store.get_user(&random_email).await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));

    }

    #[tokio::test]
    async fn test_validate_user() {

        // Given
        let mut user_store: HashmapUserStore = HashmapUserStore::default();
        let email = Email::parse("johnwick@gmail.com".to_owned()).unwrap();
        let password = Password::parse("********".to_owned()).unwrap();

        let user = User {
            email: email.clone(),
            password: password.clone(),
            requires_2fa: true
        };

        // When-Then
        // User that exists with correct password
        user_store.users.insert(email.clone(), user.clone());
        let result = user_store.validate_user(&email, &password).await;
        assert_eq!(result, Ok(()));

        // User that exists with incorrect password
        let wrong_password = Password::parse("wrongpassword".to_owned()).unwrap();
        let result = user_store.validate_user(&email, &wrong_password).await;
        assert_eq!(result, Err(UserStoreError::InvalidCredentials));

        // User that doesn't exist
        let result = user_store
            .validate_user(
                &Email::parse("nonexistent@example.com".to_string()).unwrap(),
                &password,
            )
            .await;
        assert_eq!(result, Err(UserStoreError::UserNotFound));

    }

}