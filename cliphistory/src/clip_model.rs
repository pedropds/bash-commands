use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ClipEntry {
    pub timestamp: String,
    pub content: String,
}