use crate::Client;
use crate::error::Error;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Get Display Preferences\n\nSends a `GET` request to `/DisplayPreferences/{displayPreferencesId}`\n\nArguments:\n- `display_preferences_id`: Display preferences id.\n- `client`: Client.\n- `user_id`: User id.\n"]
    pub async fn get_display_preferences(
        &self,
        display_preferences_id: &str,
        client: &str,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::DisplayPreferencesDto, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/DisplayPreferences/{}",
                encode_path(display_preferences_id)
            ),
        )
        .query("client", client)
        .query_opt("userId", user_id)
        .send()
        .await
    }

    #[doc = "Update Display Preferences\n\nSends a `POST` request to `/DisplayPreferences/{displayPreferencesId}`\n\nArguments:\n- `display_preferences_id`: Display preferences id.\n- `client`: Client.\n- `user_id`: User Id.\n- `body`: New Display Preferences object.\n"]
    pub async fn update_display_preferences(
        &self,
        display_preferences_id: &str,
        client: &str,
        user_id: Option<&uuid::Uuid>,
        body: &types::DisplayPreferencesDto,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/DisplayPreferences/{}",
                encode_path(display_preferences_id)
            ),
        )
        .query("client", client)
        .query_opt("userId", user_id)
        .json_body(body)
        .send_no_content()
        .await
    }
}
