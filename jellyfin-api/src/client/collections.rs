use crate::Client;
use crate::error::Error;
use crate::util::encode_path;

impl Client {
    #[doc = "Adds items to a collection\n\nSends a `POST` request to `/Collections/{collectionId}/Items`\n\nArguments:\n- `collection_id`: The collection id.\n- `ids`: Item ids, comma delimited.\n"]
    pub async fn add_to_collection(
        &self,
        collection_id: &uuid::Uuid,
        ids: &[uuid::Uuid],
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Collections/{}/Items",
                encode_path(&collection_id.to_string())
            ),
        )
        .query_list("ids", ids)
        .send_no_content()
        .await
    }

    #[doc = "Removes items from a collection\n\nSends a `DELETE` request to `/Collections/{collectionId}/Items`\n\nArguments:\n- `collection_id`: The collection id.\n- `ids`: Item ids, comma delimited.\n"]
    pub async fn remove_from_collection(
        &self,
        collection_id: &uuid::Uuid,
        ids: &[uuid::Uuid],
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!(
                "/Collections/{}/Items",
                encode_path(&collection_id.to_string())
            ),
        )
        .query_list("ids", ids)
        .send_no_content()
        .await
    }
}
