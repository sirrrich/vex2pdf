use super::*;
use copyright_evidence::CopyrightEvidence;
use license_evidence::LicenseEvidence;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Evidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub licenses: Option<Vec<LicenseEvidence>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub copyright: Option<Vec<CopyrightEvidence>>,
}
