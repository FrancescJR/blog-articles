use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UserRegisteredDomainEvent {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub occurred_at: DateTime<Utc>
}
