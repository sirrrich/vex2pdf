use super::*;
use tracking_info::TrackingInfo;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DocumentMetadata {
    pub id: String,
    pub version: String,
    pub author: String,
    pub timestamp: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking: Option<TrackingInfo>,
}
