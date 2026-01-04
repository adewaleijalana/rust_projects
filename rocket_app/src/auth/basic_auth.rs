use base64::{Engine, prelude::BASE64_STANDARD};

pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
        let decoded_bytes = BASE64_STANDARD.decode(base64_string).ok()?;
        let decoded_string = String::from_utf8(decoded_bytes).ok()?;

        let split_str = decoded_string.split(":").collect::<Vec<_>>();

        if split_str.len() != 2 {
            return None;
        }

        Some(BasicAuth {
            username: split_str[0].to_string(),
            password: split_str[1].to_string(),
        })
    }
}
