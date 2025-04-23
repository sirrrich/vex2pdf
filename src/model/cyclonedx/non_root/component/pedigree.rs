use super::*;
use commit::Commit;
use component::Component;
use patch::Patch;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Pedigree {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ancestors: Option<Vec<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub descendants: Option<Vec<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub variants: Option<Vec<Component>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub commits: Option<Vec<Commit>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub patches: Option<Vec<Patch>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub notes: Option<String>,
}
