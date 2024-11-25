use serde::Deserialize;

#[derive(Deserialize)]
pub struct SessionResponse {
    pub token: String,
}
