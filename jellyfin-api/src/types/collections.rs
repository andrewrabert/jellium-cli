#[doc = "`CollectionCreationResult`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct CollectionCreationResult {
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
}
