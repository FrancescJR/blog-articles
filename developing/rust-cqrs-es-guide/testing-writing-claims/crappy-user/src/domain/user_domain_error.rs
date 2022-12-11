#[derive(Debug)]
pub enum UserDomainError {
    UserAlreadyRegistered(String),
}
