use serde::{Deserialize, Serialize};

pub const SWH_HTTP_API_URL: &str = "http://localhost:50761";

#[derive(Debug, Deserialize, Serialize)]
pub struct SwhResponse {
    pub success: bool,
    pub data: Vec<HostsMeta>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HostsMeta {
    pub id: String,
    pub title: Option<String>,
    pub on: Option<bool>,
}
