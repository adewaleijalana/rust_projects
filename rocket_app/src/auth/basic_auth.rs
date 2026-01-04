use base64::{Engine, prelude::BASE64_STANDARD};
use dotenv::dotenv;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome, Request},
};
use std::env;

pub struct BasicAuth {
    username: String,
    password: String,
}

impl BasicAuth {
    pub fn new(username: String, password: String) -> Self {
        Self { username, password }
    }

    pub fn decode_authorization_header(header: &str) -> Option<BasicAuth> {
        let split_str = header.split_whitespace().collect::<Vec<_>>();
        if split_str.len() != 2 {
            return None;
        }

        if split_str[0] != "Basic" {
            return None;
        }

        Self::from_base64_encoded(split_str[1])
    }

    fn from_base64_encoded(base64_string: &str) -> Option<BasicAuth> {
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

#[rocket::async_trait]
impl<'r> FromRequest<'r> for BasicAuth {
    type Error = ();

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = req.headers().get_one("Authorization");

        dotenv().ok();

        if let Some(auth_header) = auth_header {
            println!("Authorization: {}", auth_header);
            if let Some(auth) = Self::decode_authorization_header(auth_header) {
                let username = env::var("TEST_USERNAME").expect("Error getting username");
                let password = env::var("TEST_PASSWORD").expect("Error getting password");
                if auth.username == username && auth.password == password {
                    return Outcome::Success(auth);
                }
            }
        }

        Outcome::Error((Status::Unauthorized, ()))
    }
}
