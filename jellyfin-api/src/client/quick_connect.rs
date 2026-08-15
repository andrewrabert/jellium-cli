use crate::Client;
use crate::error::Error;
use crate::types;

impl Client {
    #[doc = "Authorizes a pending quick connect request\n\nSends a `POST` request to `/QuickConnect/Authorize`\n\nArguments:\n- `code`: Quick connect code to authorize.\n- `user_id`: The user the authorize. Access to the requested user is required.\n"]
    pub async fn authorize_quick_connect(
        &self,
        code: &str,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<bool, Error> {
        self.request(reqwest::Method::POST, "/QuickConnect/Authorize".into())
            .query("code", code)
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Attempts to retrieve authentication information\n\nSends a `GET` request to `/QuickConnect/Connect`\n\nArguments:\n- `secret`: Secret previously returned from the Initiate endpoint.\n"]
    pub async fn get_quick_connect_state(
        &self,
        secret: &str,
    ) -> Result<types::QuickConnectResult, Error> {
        self.request(reqwest::Method::GET, "/QuickConnect/Connect".into())
            .query("secret", secret)
            .send()
            .await
    }

    #[doc = "Gets the current quick connect state\n\nSends a `GET` request to `/QuickConnect/Enabled`\n\n"]
    pub async fn get_quick_connect_enabled(&self) -> Result<bool, Error> {
        self.request(reqwest::Method::GET, "/QuickConnect/Enabled".into())
            .send()
            .await
    }

    #[doc = "Initiate a new quick connect request\n\nSends a `POST` request to `/QuickConnect/Initiate`\n\n"]
    pub async fn initiate_quick_connect(&self) -> Result<types::QuickConnectResult, Error> {
        self.request(reqwest::Method::POST, "/QuickConnect/Initiate".into())
            .send()
            .await
    }
}
