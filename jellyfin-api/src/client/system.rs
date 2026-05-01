use crate::types;
use crate::error::Error;
use crate::util::encode_path;
use crate::Client;

impl Client {
    #[doc = "Gets activity log entries\n\nSends a `GET` request to `/System/ActivityLog/Entries`\n\nArguments:\n- `has_user_id`: Optional. Filter log entries if it has user id, or not.\n- `limit`: Optional. The maximum number of records to return.\n- `min_date`: Optional. The minimum date. Format = ISO.\n- `start_index`: Optional. The record index to start at. All items with a lower index will be dropped from the results.\n"]
    pub async fn get_log_entries(
        &self,
        has_user_id: Option<bool>,
        limit: Option<i32>,
        min_date: Option<&chrono::DateTime<chrono::Utc>>,
        start_index: Option<i32>,
    ) -> Result<types::ActivityLogEntryQueryResult, Error> {
        self.request(reqwest::Method::GET, "/System/ActivityLog/Entries".into())
            .query_opt("hasUserId", has_user_id)
            .query_opt("limit", limit)
            .query_opt("minDate", min_date)
            .query_opt("startIndex", start_index)
            .send()
            .await
    }

    #[doc = "Upload a document\n\nSends a `POST` request to `/ClientLog/Document`\n\n"]
    pub async fn log_file<B: Into<reqwest::Body>>(
        &self,
        body: B,
    ) -> Result<types::ClientLogDocumentResponseDto, Error> {
        self.request(reqwest::Method::POST, "/ClientLog/Document".into())
            .raw_body(body, "application/octet-stream")
            .send()
            .await
    }

    #[doc = "Creates a new collection\n\nSends a `POST` request to `/Collections`\n\nArguments:\n- `ids`: Item Ids to add to the collection.\n- `is_locked`: Whether or not to lock the new collection.\n- `name`: The name of the collection.\n- `parent_id`: Optional. Create the collection within a specific folder.\n"]
    pub async fn create_collection(
        &self,
        ids: Option<&Vec<String>>,
        is_locked: Option<bool>,
        name: Option<&str>,
        parent_id: Option<&uuid::Uuid>,
    ) -> Result<types::CollectionCreationResult, Error> {
        self.request(reqwest::Method::POST, "/Collections".into())
            .query_list_opt("ids", ids)
            .query_opt("isLocked", is_locked)
            .query_opt("name", name)
            .query_opt("parentId", parent_id)
            .send()
            .await
    }

    #[doc = "Gets application configuration\n\nSends a `GET` request to `/System/Configuration`\n\n"]
    pub async fn get_configuration(
        &self,
    ) -> Result<types::ServerConfiguration, Error> {
        self.request(reqwest::Method::GET, "/System/Configuration".into())
            .send()
            .await
    }

    #[doc = "Updates application configuration\n\nSends a `POST` request to `/System/Configuration`\n\nArguments:\n- `body`: Configuration.\n"]
    pub async fn update_configuration(
        &self,
        body: &types::ServerConfiguration,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/System/Configuration".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets a named configuration\n\nSends a `GET` request to `/System/Configuration/{key}`\n\nArguments:\n- `key`: Configuration key.\n"]
    pub async fn get_named_configuration(
        &self,
        key: &str,
    ) -> Result<String, Error> {
        self.request(reqwest::Method::GET, format!("/System/Configuration/{}", encode_path(key)))
            .send()
            .await
    }

    #[doc = "Updates named configuration\n\nSends a `POST` request to `/System/Configuration/{key}`\n\nArguments:\n- `key`: Configuration key.\n- `body`: Configuration.\n"]
    pub async fn update_named_configuration(
        &self,
        key: &str,
        body: &::serde_json::Value,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, format!("/System/Configuration/{}", encode_path(key)))
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Updates branding configuration\n\nSends a `POST` request to `/System/Configuration/Branding`\n\nArguments:\n- `body`: Branding configuration.\n"]
    pub async fn update_branding_configuration(
        &self,
        body: &types::BrandingOptionsDto,
    ) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/System/Configuration/Branding".into())
            .json_body(body)
            .send_no_content()
            .await
    }

    #[doc = "Gets a default MetadataOptions object\n\nSends a `GET` request to `/System/Configuration/MetadataOptions/Default`\n\n"]
    pub async fn get_default_metadata_options(
        &self,
    ) -> Result<types::MetadataOptions, Error> {
        self.request(reqwest::Method::GET, "/System/Configuration/MetadataOptions/Default".into())
            .send()
            .await
    }

    #[doc = "Gets a list of available fallback font files\n\nSends a `GET` request to `/FallbackFont/Fonts`\n\n"]
    pub async fn get_fallback_font_list(
        &self,
    ) -> Result<Vec<types::FontFile>, Error> {
        self.request(reqwest::Method::GET, "/FallbackFont/Fonts".into())
            .send()
            .await
    }

    #[doc = "Gets a fallback font file\n\nSends a `GET` request to `/FallbackFont/Fonts/{name}`\n\nArguments:\n- `name`: The name of the fallback font file to get.\n"]
    pub async fn get_fallback_font(
        &self,
        name: &str,
    ) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, format!("/FallbackFont/Fonts/{}", encode_path(name)))
            .send_response()
            .await
    }

    #[doc = "Gets information about the request endpoint\n\nSends a `GET` request to `/System/Endpoint`\n\n"]
    pub async fn get_endpoint_info(
        &self,
    ) -> Result<types::EndPointInfo, Error> {
        self.request(reqwest::Method::GET, "/System/Endpoint".into())
            .send()
            .await
    }

    #[doc = "Gets information about the server\n\nSends a `GET` request to `/System/Info`\n\n"]
    pub async fn get_system_info(
        &self,
    ) -> Result<types::SystemInfo, Error> {
        self.request(reqwest::Method::GET, "/System/Info".into())
            .send()
            .await
    }

    #[doc = "Gets public information about the server\n\nSends a `GET` request to `/System/Info/Public`\n\n"]
    pub async fn get_public_system_info(
        &self,
    ) -> Result<types::PublicSystemInfo, Error> {
        self.request(reqwest::Method::GET, "/System/Info/Public".into())
            .send()
            .await
    }

    #[doc = "Gets information about the server\n\nSends a `GET` request to `/System/Info/Storage`\n\n"]
    pub async fn get_system_storage(
        &self,
    ) -> Result<types::SystemStorageDto, Error> {
        self.request(reqwest::Method::GET, "/System/Info/Storage".into())
            .send()
            .await
    }

    #[doc = "Gets a list of available server log files\n\nSends a `GET` request to `/System/Logs`\n\n"]
    pub async fn get_server_logs(
        &self,
    ) -> Result<Vec<types::LogFile>, Error> {
        self.request(reqwest::Method::GET, "/System/Logs".into())
            .send()
            .await
    }

    #[doc = "Gets a log file\n\nSends a `GET` request to `/System/Logs/Log`\n\nArguments:\n- `name`: The name of the log file to get.\n"]
    pub async fn get_log_file(&self, name: &str) -> Result<reqwest::Response, Error> {
        self.request(reqwest::Method::GET, "/System/Logs/Log".into())
            .query("name", name)
            .send_response()
            .await
    }

    #[doc = "Pings the system\n\nSends a `GET` request to `/System/Ping`\n\n"]
    pub async fn get_ping_system(
        &self,
    ) -> Result<String, Error> {
        self.request(reqwest::Method::GET, "/System/Ping".into())
            .send()
            .await
    }

    #[doc = "Pings the system\n\nSends a `POST` request to `/System/Ping`\n\n"]
    pub async fn post_ping_system(
        &self,
    ) -> Result<String, Error> {
        self.request(reqwest::Method::POST, "/System/Ping".into())
            .send()
            .await
    }

    #[doc = "Restarts the application\n\nSends a `POST` request to `/System/Restart`\n\n"]
    pub async fn restart_application(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/System/Restart".into())
            .send_no_content()
            .await
    }

    #[doc = "Shuts down the application\n\nSends a `POST` request to `/System/Shutdown`\n\n"]
    pub async fn shutdown_application(&self) -> Result<(), Error> {
        self.request(reqwest::Method::POST, "/System/Shutdown".into())
            .send_no_content()
            .await
    }

    #[doc = "Gets the current UTC time\n\nSends a `GET` request to `/GetUtcTime`\n\n"]
    pub async fn get_utc_time(
        &self,
    ) -> Result<types::UtcTimeResponse, Error> {
        self.request(reqwest::Method::GET, "/GetUtcTime".into())
            .send()
            .await
    }

    #[doc = "Gets the TMDb image configuration options\n\nSends a `GET` request to `/Tmdb/ClientConfiguration`\n\n"]
    pub async fn tmdb_client_configuration(
        &self,
    ) -> Result<types::ConfigImageTypes, Error> {
        self.request(reqwest::Method::GET, "/Tmdb/ClientConfiguration".into())
            .send()
            .await
    }
}
