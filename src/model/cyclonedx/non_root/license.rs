use super::*;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Licenses {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub license: Option<Vec<License>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expression: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct License {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<LicenseText>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct LicenseText {
    pub content: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contentType: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
}