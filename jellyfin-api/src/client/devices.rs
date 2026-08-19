use crate::Client;
use crate::error::Error;
use crate::types;

impl Client {
    #[doc = "Get Devices\n\nSends a `GET` request to `/Devices`\n\nArguments:\n- `user_id`: Gets or sets the user identifier.\n"]
    pub async fn get_devices(
        &self,
        user_id: Option<&uuid::Uuid>,
    ) -> Result<types::DeviceInfoDtoQueryResult, Error> {
        self.request(reqwest::Method::GET, "/Devices".into())
            .query_opt("userId", user_id)
            .send()
            .await
    }

    #[doc = "Deletes a device\n\nSends a `DELETE` request to `/Devices`\n\nArguments:\n- `id`: Device Id.\n"]
    pub async fn delete_device(&self, id: &str) -> Result<(), Error> {
        self.request(reqwest::Method::DELETE, "/Devices".into())
            .query("id", id)
            .send_no_content()
            .await
    }

    #[doc = "Get info for a device\n\nSends a `GET` request to `/Devices/Info`\n\nArguments:\n- `id`: Device Id.\n"]
    pub async fn get_device_info(&self, id: &str) -> Result<types::DeviceInfoDto, Error> {
        self.request(reqwest::Method::GET, "/Devices/Info".into())
            .query("id", id)
            .send()
            .await
    }

    #[doc = "Get options for a device\n\nSends a `GET` request to `/Devices/Options`\n\nArguments:\n- `id`: Device Id.\n"]
    pub async fn get_device_options(&self, id: &str) -> Result<types::DeviceOptionsDto, Error> {
        self.request(reqwest::Method::GET, "/Devices/Options".into())
            .query("id", id)
            .send()
            .await
    }

    #[doc = "Update device options\n\nSends a `POST` request to `/Devices/Options`\n\nArguments:\n- `id`: Device Id.\n- `body`: Device Options.\n"]
    pub async fn update_device_options(
        &self,
        id: &str,
        body: &types::DeviceOptionsDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/Devices/Options".into())
            .query("id", id)
            .json_body(body)
            .send_no_content()
            .await
    }
}
