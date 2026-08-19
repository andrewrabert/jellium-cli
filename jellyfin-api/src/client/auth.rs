use crate::Client;
use crate::error::Error;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Get all keys\n\nSends a `GET` request to `/Auth/Keys`\n\n"]
    pub async fn get_keys(&self) -> Result<types::AuthenticationInfoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Auth/Keys".into())
            .send()
            .await
    }

    #[doc = "Create a new api key\n\nSends a `POST` request to `/Auth/Keys`\n\nArguments:\n- `app`: Name of the app using the authentication key.\n"]
    pub async fn create_key(&self, app: &str) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Auth/Keys".into())
            .query("app", app)
            .send_no_content()
            .await
    }

    #[doc = "Remove an api key\n\nSends a `DELETE` request to `/Auth/Keys/{key}`\n\nArguments:\n- `key`: The access token to delete.\n"]
    pub async fn revoke_key(&self, key: &str) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/Auth/Keys/{}", encode_path(key)),
        )
        .send_no_content()
        .await
    }

    #[doc = "Get all password reset providers\n\nSends a `GET` request to `/Auth/PasswordResetProviders`\n\n"]
    pub async fn get_password_reset_providers(&self) -> Result<Vec<types::NameIdPair>, Error> {
        self.request(reqwest::Method::GET, "/Auth/PasswordResetProviders".into())
            .send()
            .await
    }

    #[doc = "Get all auth providers\n\nSends a `GET` request to `/Auth/Providers`\n\n"]
    pub async fn get_auth_providers(&self) -> Result<Vec<types::NameIdPair>, Error> {
        self.request(reqwest::Method::GET, "/Auth/Providers".into())
            .send()
            .await
    }
}
