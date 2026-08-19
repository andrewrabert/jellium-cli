use crate::Client;
use crate::error::Error;
use crate::types;

impl Client {
    #[doc = "Get user profile image\n\nSends a `GET` request to `/UserImage`\n\nArguments:\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `user_id`: User id.\n"]
    pub async fn get_user_image(
        &self,
        format: Option<types::ImageFormat>,
        tag: Option<&str>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, "/UserImage".into())
            .query_opt("format", format)
            .query_opt("tag", tag)
            .query_opt("userId", user_id)
            .send_response()
            .await
    }

    #[doc = "Sets the user image\n\nSends a `POST` request to `/UserImage`\n\nArguments:\n- `user_id`: User Id.\n- `body`\n"]
    pub async fn post_user_image<B: Into<reqwest::Body>>(
        &self,
        user_id: Option<&uuid::Uuid>,
        body: B,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/UserImage".into())
            .query_opt("userId", user_id)
            .raw_body(body, "application/octet-stream")
            .send_no_content()
            .await
    }

    #[doc = "Delete the user's image\n\nSends a `DELETE` request to `/UserImage`\n\nArguments:\n- `user_id`: User Id.\n"]
    pub async fn delete_user_image(&self, user_id: Option<&uuid::Uuid>) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, "/UserImage".into())
            .query_opt("userId", user_id)
            .send_no_content()
            .await
    }

    #[doc = "Get user profile image\n\nSends a `HEAD` request to `/UserImage`\n\nArguments:\n- `format`: Determines the output format of the image - original,gif,jpg,png.\n- `tag`: Optional. Supply the cache tag from the item object to receive strong caching headers.\n- `user_id`: User id.\n"]
    pub async fn head_user_image(
        &self,
        format: Option<types::ImageFormat>,
        tag: Option<&str>,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::HEAD, "/UserImage".into())
            .query_opt("format", format)
            .query_opt("tag", tag)
            .query_opt("userId", user_id)
            .send_response()
            .await
    }
}
