
#[doc = "`CollectionCreationResult`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct CollectionCreationResult {
    #[serde(
        rename = "Id",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub id: Option<uuid::Uuid>,
}

impl Default for CollectionCreationResult {
    fn default() -> Self {
        Self {
            id: Default::default(),
        }
    }
}

