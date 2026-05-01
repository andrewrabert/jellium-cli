use crate::types;
use crate::error::Error;
use crate::Client;

impl Client {
    #[doc = "Completes the startup wizard\n\nSends a `POST` request to `/Startup/Complete`\n\n"]
    pub async fn complete_wizard(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Startup/Complete".into())
            .send_no_content()
            .await
    }

    #[doc = "Gets the initial startup wizard configuration\n\nSends a `GET` request to `/Startup/Configuration`\n\n"]
    pub async fn get_startup_configuration(
        &self,
    ) -> Result<types::StartupConfigurationDto, Error> {
        self.request(reqwest::Method::GET, "/Startup/Configuration".into())
            .send()
            .await
    }

    #[doc = "Sets the initial startup wizard configuration\n\nSends a `POST` request to `/Startup/Configuration`\n\nArguments:\n- `body`: The updated startup configuration.\n"]
    pub async fn update_initial_configuration(
        &self,
        body: &types::StartupConfigurationDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Startup/Configuration".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets the first user\n\nSends a `GET` request to `/Startup/FirstUser`\n\n"]
    pub async fn get_first_user_2(
        &self,
    ) -> Result<types::StartupUserDto, Error> {
        self.request(reqwest::Method::GET, "/Startup/FirstUser".into())
            .send()
            .await
    }

    #[doc = "Sets remote access and UPnP\n\nSends a `POST` request to `/Startup/RemoteAccess`\n\nArguments:\n- `body`: The startup remote access dto.\n"]
    pub async fn set_remote_access(
        &self,
        body: &types::StartupRemoteAccessDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Startup/RemoteAccess".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets the first user\n\nSends a `GET` request to `/Startup/User`\n\n"]
    pub async fn get_first_user(
        &self,
    ) -> Result<types::StartupUserDto, Error> {
        self.request(reqwest::Method::GET, "/Startup/User".into())
            .send()
            .await
    }

    #[doc = "Sets the user name and password\n\nSends a `POST` request to `/Startup/User`\n\nArguments:\n- `body`: The DTO containing username and password.\n"]
    pub async fn update_startup_user(
        &self,
        body: &types::StartupUserDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Startup/User".into())
            .json_body(body)
            .send_no_content()
            .await
    }
}
