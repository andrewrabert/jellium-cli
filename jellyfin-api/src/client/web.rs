use crate::types;
use crate::error::Error;
use crate::Client;

impl Client {
    #[doc = "Gets a dashboard configuration page\n\nSends a `GET` request to `/web/ConfigurationPage`\n\nArguments:\n- `name`: The name of the page.\n"]
    pub async fn get_dashboard_configuration_page(
        &self,
        name: Option<&str>,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, "/web/ConfigurationPage".into())
            .query_opt("name", name)
            .send_response()
            .await
    }

    #[doc = "Gets the configuration pages\n\nSends a `GET` request to `/web/ConfigurationPages`\n\nArguments:\n- `enable_in_main_menu`: Whether to enable in the main menu.\n"]
    pub async fn get_configuration_pages(
        &self,
        enable_in_main_menu: Option<bool>,
    ) -> Result<Vec<types::ConfigurationPageInfo>, Error> {
        self.request(reqwest::Method::GET, "/web/ConfigurationPages".into())
            .query_opt("enableInMainMenu", enable_in_main_menu)
            .send()
            .await
    }
}
