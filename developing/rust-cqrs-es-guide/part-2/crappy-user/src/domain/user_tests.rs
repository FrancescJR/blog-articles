#[cfg(test)]
pub(crate) mod tests {

    use crate::domain::*;
    use uuid::Uuid;

    #[test]
    fn register_user() {
        //we are deciding here that the user is going to have only those 3 pieces of information:
        let id = Uuid::new_v4();
        let email = "francesc.travesa@mymail.com".to_string();
        let password = "mySecretPassword".to_string();

        // we would like to "create" a user by registering:
        let mut user = User::default();
        user = user.register(id.clone(), email.clone(), password.clone()).unwrap();

        // then we have some claims:
        assert_eq!(user.id(), id);
        assert_eq!(email, user.email_as_ref());
        assert_eq!(true, user.is_registered());
    }
}
