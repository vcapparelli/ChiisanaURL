use serde_derive::Deserialize;

#[derive(Deserialize)]
pub struct Request {
    pub url: String,
    pub custom_path: String,
}

#[derive(Deserialize)]
pub struct PasswordRequest {
    pub password: String,
}


