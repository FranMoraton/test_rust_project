pub mod authentication {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::Hasher;

    pub struct User {
        username: String,
        pub password_hash: u64,
    }

    fn hash_password(input: &[u8]) -> u64 {
        let mut s = DefaultHasher::new();
        s.write(input);
        s.finish()
    }

    impl User {
        pub fn new(username: &str, password: &str) -> User {
            User {
                username: username.to_string(),
                password_hash: hash_password(password.as_bytes()),
            }
        }

        pub fn get_username(&self) -> &String {
            &self.username
        }

        pub fn set_password(&mut self, new_password: &str) {
            self.password_hash = hash_password(new_password.as_bytes())
        }
    }
}

pub fn main() {
    let user = authentication::User::new("jeremy", "super-secret");

    println!("The username is: {}", user.get_username());
    println!("The username is: {}", user.password_hash);
}
