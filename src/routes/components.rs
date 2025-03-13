use serde::{Deserialize, Serialize};

// === Requests ===

#[derive(Deserialize, Debug)]
pub struct DebugRequest {
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

#[derive(Serialize)]
pub struct SimpleIPResponse {
    pub ip: String
}

#[derive(Serialize)]
pub struct IdResponse {
    pub ip: String,
    pub city: String,
    pub postal: String,
    pub region: String,
    pub country: String,
    pub loc: String,
    pub timezone: String,
    pub hostname: String,
    pub org: String,
}