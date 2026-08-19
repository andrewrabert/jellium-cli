use crate::Client;
use crate::error::Error;
use crate::types;
use crate::util::encode_path;

impl Client {
    #[doc = "Gets a list of currently installed plugins\n\nSends a `GET` request to `/Plugins`\n\n"]
    pub async fn get_plugins(&self) -> Result<Vec<types::PluginInfo>, Error> {
        self.request(reqwest::Method::GET, "/Plugins".into())
            .send()
            .await
    }

    #[doc = "Uninstalls a plugin\n\nSends a `DELETE` request to `/Plugins/{pluginId}`\n\nArguments:\n- `plugin_id`: Plugin id.\n"]
    pub async fn uninstall_plugin(&self, plugin_id: &uuid::Uuid) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!("/Plugins/{}", encode_path(&plugin_id.to_string())),
        )
        .send_no_content()
        .await
    }

    #[doc = "Uninstalls a plugin by version\n\nSends a `DELETE` request to `/Plugins/{pluginId}/{version}`\n\nArguments:\n- `plugin_id`: Plugin id.\n- `version`: Plugin version.\n"]
    pub async fn uninstall_plugin_by_version(
        &self,
        plugin_id: &uuid::Uuid,
        version: &str,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::DELETE,
            format!(
                "/Plugins/{}/{}",
                encode_path(&plugin_id.to_string()),
                encode_path(version)
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Disable a plugin\n\nSends a `POST` request to `/Plugins/{pluginId}/{version}/Disable`\n\nArguments:\n- `plugin_id`: Plugin id.\n- `version`: Plugin version.\n"]
    pub async fn disable_plugin(&self, plugin_id: &uuid::Uuid, version: &str) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Plugins/{}/{}/Disable",
                encode_path(&plugin_id.to_string()),
                encode_path(version)
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Enables a disabled plugin\n\nSends a `POST` request to `/Plugins/{pluginId}/{version}/Enable`\n\nArguments:\n- `plugin_id`: Plugin id.\n- `version`: Plugin version.\n"]
    pub async fn enable_plugin(&self, plugin_id: &uuid::Uuid, version: &str) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Plugins/{}/{}/Enable",
                encode_path(&plugin_id.to_string()),
                encode_path(version)
            ),
        )
        .send_no_content()
        .await
    }

    #[doc = "Gets a plugin's image\n\nSends a `GET` request to `/Plugins/{pluginId}/{version}/Image`\n\nArguments:\n- `plugin_id`: Plugin id.\n- `version`: Plugin version.\n"]
    pub async fn get_plugin_image(
        &self,
        plugin_id: &uuid::Uuid,
        version: &str,
    ) -> Result<reqwest::Response, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Plugins/{}/{}/Image",
                encode_path(&plugin_id.to_string()),
                encode_path(version)
            ),
        )
        .send_response()
        .await
    }

    #[doc = "Gets plugin configuration\n\nSends a `GET` request to `/Plugins/{pluginId}/Configuration`\n\nArguments:\n- `plugin_id`: Plugin id.\n"]
    pub async fn get_plugin_configuration(
        &self,
        plugin_id: &uuid::Uuid,
    ) -> Result<serde_json::Value, Error> {
        self.request(
            reqwest::Method::GET,
            format!(
                "/Plugins/{}/Configuration",
                encode_path(&plugin_id.to_string())
            ),
        )
        .send()
        .await
    }

    #[doc = "Updates plugin configuration\n\nAccepts plugin configuration as JSON body.\n\nSends a `POST` request to `/Plugins/{pluginId}/Configuration`\n\nArguments:\n- `plugin_id`: Plugin id.\n"]
    pub async fn update_plugin_configuration(
        &self,
        plugin_id: &uuid::Uuid,
        body: &serde_json::Value,
    ) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!(
                "/Plugins/{}/Configuration",
                encode_path(&plugin_id.to_string())
            ),
        )
        .json_body(body)
        .send_no_content()
        .await
    }

    #[doc = "Gets a plugin's manifest\n\nSends a `POST` request to `/Plugins/{pluginId}/Manifest`\n\nArguments:\n- `plugin_id`: Plugin id.\n"]
    pub async fn get_plugin_manifest(&self, plugin_id: &uuid::Uuid) -> Result<(), Error> {
        self.request(
            reqwest::Method::POST,
            format!("/Plugins/{}/Manifest", encode_path(&plugin_id.to_string())),
        )
        .send_no_content()
        .await
    }
}
