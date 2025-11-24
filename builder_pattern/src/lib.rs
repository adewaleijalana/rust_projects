#[derive(Debug)]
#[allow(dead_code)]
pub struct User {
    username: String,
    email: String,
    age: u32,
}

pub struct UserBuilder {
    username: String,
    email: String,
    age: u32,
}

impl UserBuilder {
    pub fn new() -> Self {
        UserBuilder {
            username: String::new(),
            email: String::new(),
            age: 0,
        }
    }

    pub fn username(mut self, username: &str) -> Self {
        self.username = username.to_string();
        self
    }

    pub fn email(mut self, email: &str) -> Self {
        self.email = email.to_string();
        self
    }

    pub fn age(mut self, age: u32) -> Self {
        self.age = age;
        self
    }

    pub fn build(self) -> User {
        User {
            username: self.username,
            email: self.email,
            age: self.age,
        }
    }
}