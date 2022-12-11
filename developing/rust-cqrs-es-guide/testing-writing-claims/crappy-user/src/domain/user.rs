use uuid::Uuid;
use crate::domain::*;
use chrono::{DateTime, Utc};

pub struct User {
    id: Uuid,
    email: String,
    password: String,
    is_registered: bool,
}

impl Default for User {
    fn default() -> Self {
        User::new(Uuid::new_v4(), "init_user@gmail.com".to_string(), "defaultPassword".to_string())
    }
}

impl User {
    pub fn new(id: Uuid, email: String, password: String) -> Self {
        User {
            id,
            email,
            password,
            is_registered: false,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn email_as_ref(&self) -> &str {
        &self.email
    }

    pub fn password_as_ref(&self) -> &str {
        &self.password
    }

    pub fn is_registered(&self) -> bool {
        self.is_registered
    }

    pub fn register(
        mut self,
        id: Uuid,
        email: String,
        password: String,
    ) -> Result<Self, UserDomainError> {
        if self.is_registered {
            return Err(UserDomainError::UserAlreadyRegistered(self.email));
        }
        let user_registered_event = UserRegisteredDomainEvent {
            id,
            email,
            password_hash: password,
            occurred_at: Utc::now(),
        };
        self.apply_user_registered_event(user_registered_event.clone());
        Ok(self)
    }

    fn apply_user_registered_event(&mut self, user_registered_event: UserRegisteredDomainEvent) {
        self.id = user_registered_event.id;
        self.is_registered = true;
        self.email = user_registered_event.email;
        self.password = user_registered_event.password_hash;
    }
}
