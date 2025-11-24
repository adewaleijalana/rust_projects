
pub struct User {
    id: i32,
    name: String,
    email: String,
    password: String,
}

impl User {
    pub fn new(id: i32, name: String, email: String, password: String) -> Self {
        User {
            id,
            name,
            email,
            password,
        }
    }
    

    pub fn get_id(&self) -> i32 {
        self.id
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }

    pub fn set_name(&mut self, n: &str) {
        self.name = n.to_string();
    }
    
}