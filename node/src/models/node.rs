use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
pub struct Node {
    pub address: String,
    pub version: String,
}
