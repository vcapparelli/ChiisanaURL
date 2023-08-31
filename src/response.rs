use serde_derive::Serialize;

#[derive(Serialize)]
pub struct Response {
    pub url: String,
    pub password: String,
}
