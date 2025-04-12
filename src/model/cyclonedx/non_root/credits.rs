use super::*;
use individual::Individual;
use organization::Organization;
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Credits {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub individuals: Option<Vec<Individual>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub organizations: Option<Vec<Organization>>,
}

