use serde::{Deserialize, Serialize};

// === Requests ===

#[derive(Deserialize, Debug)]
pub struct PingRequest {
    pub message: String
}


// === Responses ===

#[derive(Serialize)]
pub struct PongResponse {
    pub message: String
}

impl PongResponse {
    pub fn new() -> Self {
        PongResponse {
            message: String::from("pong")
        }
    }
}