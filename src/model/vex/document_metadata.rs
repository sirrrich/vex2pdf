use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub id: String,
    pub version: String,
    pub author: String,
    pub timestamp: String,
    // Other metadata fields as needed
}
