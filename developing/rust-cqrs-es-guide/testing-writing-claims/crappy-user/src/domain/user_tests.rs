#[cfg(test)]
pub(crate) mod tests {
    #[test]
    fn register_user() {
        //we are deciding here that the user is going to have only those 3 pieces of information:
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();
        let password = "mySecretPassword".to_string();

        // we would like to "create" a user by registering:
        let user = User::register(id, email, password);

        // then we have some claims:
        // but we won't write that yet, there are architectural bits that I want to
        // deeply analyze first with you here.
    }
}
