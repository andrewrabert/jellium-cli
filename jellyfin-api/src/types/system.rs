use super::*;

#[doc = "The cast receiver application model."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct CastReceiverApplication {
    #[doc = "Gets or sets the cast receiver application id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the cast receiver application name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for CastReceiverApplication {
    fn default() -> Self {
        Self {
            id: Default::default(),
            name: Default::default(),
        }
    }
}

#[doc = "The configuration page info."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ConfigurationPageInfo {
    #[doc = "Gets or sets the display name."]
    #[serde(
        rename = "DisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the configurations page is enabled in the main menu."]
    #[serde(
        rename = "EnableInMainMenu",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_in_main_menu: Option<bool>,
    #[doc = "Gets or sets the menu icon."]
    #[serde(rename = "MenuIcon", default, skip_serializing_if = "Option::is_none")]
    pub menu_icon: Option<String>,
    #[doc = "Gets or sets the menu section."]
    #[serde(
        rename = "MenuSection",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub menu_section: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the plugin id."]
    #[serde(rename = "PluginId", default, skip_serializing_if = "Option::is_none")]
    pub plugin_id: Option<uuid::Uuid>,
}

impl Default for ConfigurationPageInfo {
    fn default() -> Self {
        Self {
            display_name: Default::default(),
            enable_in_main_menu: Default::default(),
            menu_icon: Default::default(),
            menu_section: Default::default(),
            name: Default::default(),
            plugin_id: Default::default(),
        }
    }
}

#[doc = "Options to configure jellyfins managed database."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct DatabaseConfigurationOptions {
    #[doc = "Gets or sets the options required to use a custom database provider."]
    #[serde(
        rename = "CustomProviderOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub custom_provider_options: Option<CustomDatabaseOptions>,
    #[doc = "Gets or Sets the type of database jellyfin should use."]
    #[serde(
        rename = "DatabaseType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub database_type: Option<String>,
    #[serde(
        rename = "LockingBehavior",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub locking_behavior: Option<DatabaseLockingBehaviorTypes>,
}

impl Default for DatabaseConfigurationOptions {
    fn default() -> Self {
        Self {
            custom_provider_options: Default::default(),
            database_type: Default::default(),
            locking_behavior: Default::default(),
        }
    }
}

#[doc = "`EndPointInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct EndPointInfo {
    #[serde(
        rename = "IsInNetwork",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_in_network: Option<bool>,
    #[serde(rename = "IsLocal", default, skip_serializing_if = "Option::is_none")]
    pub is_local: Option<bool>,
}

impl Default for EndPointInfo {
    fn default() -> Self {
        Self {
            is_in_network: Default::default(),
            is_local: Default::default(),
        }
    }
}

#[doc = "Class FileSystemEntryInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct FileSystemEntryInfo {
    #[doc = "Gets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<FileSystemEntryType>,
}

impl Default for FileSystemEntryInfo {
    fn default() -> Self {
        Self {
            name: Default::default(),
            path: Default::default(),
            type_: Default::default(),
        }
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum FileSystemEntryType {
    File,
    Directory,
    NetworkComputer,
    NetworkShare,
}

impl std::fmt::Display for FileSystemEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::File => f.write_str("File"),
            Self::Directory => f.write_str("Directory"),
            Self::NetworkComputer => f.write_str("NetworkComputer"),
            Self::NetworkShare => f.write_str("NetworkShare"),
        }
    }
}

impl std::str::FromStr for FileSystemEntryType {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "File" => Ok(Self::File),
            "Directory" => Ok(Self::Directory),
            "NetworkComputer" => Ok(Self::NetworkComputer),
            "NetworkShare" => Ok(Self::NetworkShare),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for FileSystemEntryType {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for FileSystemEntryType {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for FileSystemEntryType {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "`LogFile`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct LogFile {
    #[doc = "Gets or sets the date created."]
    #[serde(
        rename = "DateCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the date modified."]
    #[serde(
        rename = "DateModified",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_modified: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the size."]
    #[serde(rename = "Size", default, skip_serializing_if = "Option::is_none")]
    pub size: Option<i64>,
}

impl Default for LogFile {
    fn default() -> Self {
        Self {
            date_created: Default::default(),
            date_modified: Default::default(),
            name: Default::default(),
            size: Default::default(),
        }
    }
}

#[doc = "`MetadataConfiguration`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MetadataConfiguration {
    #[serde(
        rename = "UseFileCreationTimeForDateAdded",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub use_file_creation_time_for_date_added: Option<bool>,
}

impl Default for MetadataConfiguration {
    fn default() -> Self {
        Self {
            use_file_creation_time_for_date_added: Default::default(),
        }
    }
}

#[doc = "Defines the MediaBrowser.Common.Net.NetworkConfiguration."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct NetworkConfiguration {
    #[doc = "Gets or sets a value indicating whether Autodiscovery is enabled."]
    #[serde(
        rename = "AutoDiscovery",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub auto_discovery: Option<bool>,
    #[doc = "Gets or sets a value used to specify the URL prefix that your Jellyfin instance can be accessed at."]
    #[serde(rename = "BaseUrl", default, skip_serializing_if = "Option::is_none")]
    pub base_url: Option<String>,
    #[doc = "Gets or sets the password required to access the X.509 certificate data in the file specified by MediaBrowser.Common.Net.NetworkConfiguration.CertificatePath."]
    #[serde(
        rename = "CertificatePassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_password: Option<String>,
    #[doc = "Gets or sets the filesystem path of an X.509 certificate to use for SSL."]
    #[serde(
        rename = "CertificatePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub certificate_path: Option<String>,
    #[doc = "Gets or sets a value indicating whether to use HTTPS."]
    #[serde(
        rename = "EnableHttps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_https: Option<bool>,
    #[doc = "Gets or sets a value indicating whether IPv6 is enabled."]
    #[serde(
        rename = "EnableIPv4",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_i_pv4: Option<bool>,
    #[doc = "Gets or sets a value indicating whether IPv6 is enabled."]
    #[serde(
        rename = "EnableIPv6",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_i_pv6: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the published server uri is based on information in HTTP requests."]
    #[serde(
        rename = "EnablePublishedServerUriByRequest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_published_server_uri_by_request: Option<bool>,
    #[doc = "Gets or sets a value indicating whether access from outside of the LAN is permitted."]
    #[serde(
        rename = "EnableRemoteAccess",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_remote_access: Option<bool>,
    #[doc = "Gets or sets a value indicating whether to enable automatic port forwarding."]
    #[serde(
        rename = "EnableUPnP",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_u_pn_p: Option<bool>,
    #[doc = "Gets or sets a value indicating whether address names that match MediaBrowser.Common.Net.NetworkConfiguration.VirtualInterfaceNames should be ignored for the purposes of binding."]
    #[serde(
        rename = "IgnoreVirtualInterfaces",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub ignore_virtual_interfaces: Option<bool>,
    #[doc = "Gets or sets the internal HTTP server port."]
    #[serde(
        rename = "InternalHttpPort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_http_port: Option<i32>,
    #[doc = "Gets or sets the internal HTTPS server port."]
    #[serde(
        rename = "InternalHttpsPort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_https_port: Option<i32>,
    #[doc = "Gets or sets a value indicating whether <seealso cref=\"P:MediaBrowser.Common.Net.NetworkConfiguration.RemoteIPFilter\" /> contains a blacklist or a whitelist. Default is a whitelist."]
    #[serde(
        rename = "IsRemoteIPFilterBlacklist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_remote_ip_filter_blacklist: Option<bool>,
    #[doc = "Gets or sets the known proxies."]
    #[serde(
        rename = "KnownProxies",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub known_proxies: Vec<String>,
    #[doc = "Gets or sets the interface addresses which Jellyfin will bind to. If empty, all interfaces will be used."]
    #[serde(
        rename = "LocalNetworkAddresses",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub local_network_addresses: Vec<String>,
    #[doc = "Gets or sets the subnets that are deemed to make up the LAN."]
    #[serde(
        rename = "LocalNetworkSubnets",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub local_network_subnets: Vec<String>,
    #[doc = "Gets or sets the public HTTP port."]
    #[serde(
        rename = "PublicHttpPort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_http_port: Option<i32>,
    #[doc = "Gets or sets the public HTTPS port."]
    #[serde(
        rename = "PublicHttpsPort",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub public_https_port: Option<i32>,
    #[doc = "Gets or sets the PublishedServerUriBySubnet\r\nGets or sets PublishedServerUri to advertise for specific subnets."]
    #[serde(
        rename = "PublishedServerUriBySubnet",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub published_server_uri_by_subnet: Vec<String>,
    #[doc = "Gets or sets the filter for remote IP connectivity. Used in conjunction with <seealso cref=\"P:MediaBrowser.Common.Net.NetworkConfiguration.IsRemoteIPFilterBlacklist\" />."]
    #[serde(
        rename = "RemoteIPFilter",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub remote_ip_filter: Vec<String>,
    #[doc = "Gets or sets a value indicating whether the server should force connections over HTTPS."]
    #[serde(
        rename = "RequireHttps",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub require_https: Option<bool>,
    #[doc = "Gets or sets a value indicating the interface name prefixes that should be ignored. The list can be comma separated and values are case-insensitive. <seealso cref=\"P:MediaBrowser.Common.Net.NetworkConfiguration.IgnoreVirtualInterfaces\" />."]
    #[serde(
        rename = "VirtualInterfaceNames",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub virtual_interface_names: Vec<String>,
}

impl Default for NetworkConfiguration {
    fn default() -> Self {
        Self {
            auto_discovery: Default::default(),
            base_url: Default::default(),
            certificate_password: Default::default(),
            certificate_path: Default::default(),
            enable_https: Default::default(),
            enable_i_pv4: Default::default(),
            enable_i_pv6: Default::default(),
            enable_published_server_uri_by_request: Default::default(),
            enable_remote_access: Default::default(),
            enable_u_pn_p: Default::default(),
            ignore_virtual_interfaces: Default::default(),
            internal_http_port: Default::default(),
            internal_https_port: Default::default(),
            is_remote_ip_filter_blacklist: Default::default(),
            known_proxies: Default::default(),
            local_network_addresses: Default::default(),
            local_network_subnets: Default::default(),
            public_http_port: Default::default(),
            public_https_port: Default::default(),
            published_server_uri_by_subnet: Default::default(),
            remote_ip_filter: Default::default(),
            require_https: Default::default(),
            virtual_interface_names: Default::default(),
        }
    }
}

#[doc = "Defines the MediaBrowser.Model.Configuration.PathSubstitution."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PathSubstitution {
    #[doc = "Gets or sets the value to substitute."]
    #[serde(rename = "From", default, skip_serializing_if = "Option::is_none")]
    pub from: Option<String>,
    #[doc = "Gets or sets the value to substitution with."]
    #[serde(rename = "To", default, skip_serializing_if = "Option::is_none")]
    pub to: Option<String>,
}

impl Default for PathSubstitution {
    fn default() -> Self {
        Self {
            from: Default::default(),
            to: Default::default(),
        }
    }
}

#[doc = "`PublicSystemInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PublicSystemInfo {
    #[doc = "Gets or sets the id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the local address."]
    #[serde(
        rename = "LocalAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_address: Option<String>,
    #[doc = "Gets or sets the operating system."]
    #[serde(
        rename = "OperatingSystem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operating_system: Option<String>,
    #[doc = "Gets or sets the product name. This is the AssemblyProduct name."]
    #[serde(
        rename = "ProductName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_name: Option<String>,
    #[doc = "Gets or sets the name of the server."]
    #[serde(
        rename = "ServerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the startup wizard is completed."]
    #[serde(
        rename = "StartupWizardCompleted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub startup_wizard_completed: Option<bool>,
    #[doc = "Gets or sets the server version."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

impl Default for PublicSystemInfo {
    fn default() -> Self {
        Self {
            id: Default::default(),
            local_address: Default::default(),
            operating_system: Default::default(),
            product_name: Default::default(),
            server_name: Default::default(),
            startup_wizard_completed: Default::default(),
            version: Default::default(),
        }
    }
}

#[doc = "Restart required."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RestartRequiredMessage {
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for RestartRequiredMessage {
    fn default() -> Self {
        Self {
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Scheduled task ended message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ScheduledTaskEndedMessage {
    #[doc = "Class TaskExecutionInfo."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<TaskResult>,
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for ScheduledTaskEndedMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Scheduled tasks info message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ScheduledTasksInfoMessage {
    #[doc = "Gets or sets the data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<Vec<TaskInfo>>,
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for ScheduledTasksInfoMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Scheduled tasks info start message.\r\nData is the timing data encoded as \"$initialDelay,$interval\" in ms."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ScheduledTasksInfoStartMessage {
    #[doc = "Gets or sets the data."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<String>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for ScheduledTasksInfoStartMessage {
    fn default() -> Self {
        Self {
            data: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Scheduled tasks info stop message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ScheduledTasksInfoStopMessage {
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for ScheduledTasksInfoStopMessage {
    fn default() -> Self {
        Self {
            message_type: Default::default(),
        }
    }
}

#[doc = "Represents the server configuration."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ServerConfiguration {
    #[doc = "Gets or sets the number of days we should retain activity logs."]
    #[serde(
        rename = "ActivityLogRetentionDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub activity_log_retention_days: Option<i32>,
    #[doc = "Gets or sets a value indicating whether clients should be allowed to upload logs."]
    #[serde(
        rename = "AllowClientLogUpload",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub allow_client_log_upload: Option<bool>,
    #[doc = "Gets or sets the cache path."]
    #[serde(rename = "CachePath", default, skip_serializing_if = "Option::is_none")]
    pub cache_path: Option<String>,
    #[doc = "Gets or sets the maximum amount of items to cache."]
    #[serde(rename = "CacheSize", default, skip_serializing_if = "Option::is_none")]
    pub cache_size: Option<i32>,
    #[doc = "Gets or sets the list of cast receiver applications."]
    #[serde(
        rename = "CastReceiverApplications",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub cast_receiver_applications: Vec<CastReceiverApplication>,
    #[serde(
        rename = "ChapterImageResolution",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub chapter_image_resolution: Option<ImageResolution>,
    #[serde(rename = "CodecsUsed", default, skip_serializing_if = "Vec::is_empty")]
    pub codecs_used: Vec<String>,
    #[serde(
        rename = "ContentTypes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub content_types: Vec<NameValuePair>,
    #[doc = "Gets or sets the cors hosts."]
    #[serde(rename = "CorsHosts", default, skip_serializing_if = "Vec::is_empty")]
    pub cors_hosts: Vec<String>,
    #[serde(
        rename = "DisableLiveTvChannelUserDataName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub disable_live_tv_channel_user_data_name: Option<bool>,
    #[serde(
        rename = "DisplaySpecialsWithinSeasons",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_specials_within_seasons: Option<bool>,
    #[doc = "Gets or sets the dummy chapter duration in seconds, use 0 (zero) or less to disable generation altogether."]
    #[serde(
        rename = "DummyChapterDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub dummy_chapter_duration: Option<i32>,
    #[doc = "Gets or sets a value indicating whether [enable case-sensitive item ids]."]
    #[serde(
        rename = "EnableCaseSensitiveItemIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_case_sensitive_item_ids: Option<bool>,
    #[serde(
        rename = "EnableExternalContentInSuggestions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_external_content_in_suggestions: Option<bool>,
    #[serde(
        rename = "EnableFolderView",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_folder_view: Option<bool>,
    #[serde(
        rename = "EnableGroupingMoviesIntoCollections",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_grouping_movies_into_collections: Option<bool>,
    #[serde(
        rename = "EnableGroupingShowsIntoCollections",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_grouping_shows_into_collections: Option<bool>,
    #[doc = "Gets or sets a value indicating whether old authorization methods are allowed."]
    #[serde(
        rename = "EnableLegacyAuthorization",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_legacy_authorization: Option<bool>,
    #[doc = "Gets or sets a value indicating whether to enable prometheus metrics exporting."]
    #[serde(
        rename = "EnableMetrics",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_metrics: Option<bool>,
    #[serde(
        rename = "EnableNormalizedItemByNameIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_normalized_item_by_name_ids: Option<bool>,
    #[doc = "Gets or sets a value indicating whether slow server responses should be logged as a warning."]
    #[serde(
        rename = "EnableSlowResponseWarning",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_slow_response_warning: Option<bool>,
    #[serde(
        rename = "ImageExtractionTimeoutMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_extraction_timeout_ms: Option<i32>,
    #[serde(
        rename = "ImageSavingConvention",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_saving_convention: Option<ImageSavingConvention>,
    #[doc = "Gets or sets the threshold in minutes after a inactive session gets closed automatically.\r\nIf set to 0 the check for inactive sessions gets disabled."]
    #[serde(
        rename = "InactiveSessionThreshold",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub inactive_session_threshold: Option<i32>,
    #[doc = "Gets or sets a value indicating whether this instance is port authorized."]
    #[serde(
        rename = "IsPortAuthorized",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_port_authorized: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance is first run."]
    #[serde(
        rename = "IsStartupWizardCompleted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_startup_wizard_completed: Option<bool>,
    #[doc = "Gets or sets the how many metadata refreshes can run concurrently."]
    #[serde(
        rename = "LibraryMetadataRefreshConcurrency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_metadata_refresh_concurrency: Option<i32>,
    #[doc = "Gets or sets the delay in seconds that we will wait after a file system change to try and discover what has been added/removed\r\nSome delay is necessary with some items because their creation is not atomic.  It involves the creation of several\r\ndifferent directories and files."]
    #[serde(
        rename = "LibraryMonitorDelay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_monitor_delay: Option<i32>,
    #[doc = "Gets or sets the how the library scan fans out."]
    #[serde(
        rename = "LibraryScanFanoutConcurrency",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_scan_fanout_concurrency: Option<i32>,
    #[doc = "Gets or sets the duration in seconds that we will wait after a library updated event before executing the library changed notification."]
    #[serde(
        rename = "LibraryUpdateDuration",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub library_update_duration: Option<i32>,
    #[doc = "Gets or sets the number of days we should retain log files."]
    #[serde(
        rename = "LogFileRetentionDays",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub log_file_retention_days: Option<i32>,
    #[doc = "Gets or sets the remaining minutes of a book that can be played while still saving playstate. If this percentage is crossed playstate will be reset to the beginning and the item will be marked watched."]
    #[serde(
        rename = "MaxAudiobookResume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_audiobook_resume: Option<i32>,
    #[doc = "Gets or sets the maximum percentage of an item that can be played while still saving playstate. If this percentage is crossed playstate will be reset to the beginning and the item will be marked watched."]
    #[serde(
        rename = "MaxResumePct",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub max_resume_pct: Option<i32>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[serde(
        rename = "MetadataOptions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub metadata_options: Vec<MetadataOptions>,
    #[doc = "Gets or sets the metadata path."]
    #[serde(
        rename = "MetadataPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_path: Option<String>,
    #[doc = "Gets or sets the minimum minutes of a book that must be played in order for playstate to be updated."]
    #[serde(
        rename = "MinAudiobookResume",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_audiobook_resume: Option<i32>,
    #[doc = "Gets or sets the minimum duration that an item must have in order to be eligible for playstate updates.."]
    #[serde(
        rename = "MinResumeDurationSeconds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_resume_duration_seconds: Option<i32>,
    #[doc = "Gets or sets the minimum percentage of an item that must be played in order for playstate to be updated."]
    #[serde(
        rename = "MinResumePct",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub min_resume_pct: Option<i32>,
    #[doc = "Gets or sets the limit for parallel image encoding."]
    #[serde(
        rename = "ParallelImageEncodingLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parallel_image_encoding_limit: Option<i32>,
    #[serde(
        rename = "PathSubstitutions",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub path_substitutions: Vec<PathSubstitution>,
    #[serde(
        rename = "PluginRepositories",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub plugin_repositories: Vec<RepositoryInfo>,
    #[doc = "Gets or sets the preferred metadata language."]
    #[serde(
        rename = "PreferredMetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_metadata_language: Option<String>,
    #[doc = "Gets or sets the last known version that was ran using the configuration."]
    #[serde(
        rename = "PreviousVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_version: Option<String>,
    #[doc = "Gets or sets the stringified PreviousVersion to be stored/loaded,\r\nbecause System.Version itself isn't xml-serializable."]
    #[serde(
        rename = "PreviousVersionStr",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub previous_version_str: Option<String>,
    #[doc = "Gets or sets a value indicating whether quick connect is available for use on this server."]
    #[serde(
        rename = "QuickConnectAvailable",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub quick_connect_available: Option<bool>,
    #[serde(
        rename = "RemoteClientBitrateLimit",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remote_client_bitrate_limit: Option<i32>,
    #[serde(
        rename = "SaveMetadataHidden",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub save_metadata_hidden: Option<bool>,
    #[serde(
        rename = "ServerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_name: Option<String>,
    #[serde(
        rename = "SkipDeserializationForBasicTypes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub skip_deserialization_for_basic_types: Option<bool>,
    #[doc = "Gets or sets the threshold for the slow response time warning in ms."]
    #[serde(
        rename = "SlowResponseThresholdMs",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub slow_response_threshold_ms: Option<i64>,
    #[doc = "Gets or sets characters to be removed from strings to create a sort name."]
    #[serde(
        rename = "SortRemoveCharacters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sort_remove_characters: Vec<String>,
    #[doc = "Gets or sets words to be removed from strings to create a sort name."]
    #[serde(
        rename = "SortRemoveWords",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sort_remove_words: Vec<String>,
    #[doc = "Gets or sets characters to be replaced with a ' ' in strings to create a sort name."]
    #[serde(
        rename = "SortReplaceCharacters",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub sort_replace_characters: Vec<String>,
    #[doc = "Gets or sets the trickplay options."]
    #[serde(
        rename = "TrickplayOptions",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub trickplay_options: Option<TrickplayOptions>,
    #[serde(rename = "UICulture", default, skip_serializing_if = "Option::is_none")]
    pub ui_culture: Option<String>,
}

impl Default for ServerConfiguration {
    fn default() -> Self {
        Self {
            activity_log_retention_days: Default::default(),
            allow_client_log_upload: Default::default(),
            cache_path: Default::default(),
            cache_size: Default::default(),
            cast_receiver_applications: Default::default(),
            chapter_image_resolution: Default::default(),
            codecs_used: Default::default(),
            content_types: Default::default(),
            cors_hosts: Default::default(),
            disable_live_tv_channel_user_data_name: Default::default(),
            display_specials_within_seasons: Default::default(),
            dummy_chapter_duration: Default::default(),
            enable_case_sensitive_item_ids: Default::default(),
            enable_external_content_in_suggestions: Default::default(),
            enable_folder_view: Default::default(),
            enable_grouping_movies_into_collections: Default::default(),
            enable_grouping_shows_into_collections: Default::default(),
            enable_legacy_authorization: Default::default(),
            enable_metrics: Default::default(),
            enable_normalized_item_by_name_ids: Default::default(),
            enable_slow_response_warning: Default::default(),
            image_extraction_timeout_ms: Default::default(),
            image_saving_convention: Default::default(),
            inactive_session_threshold: Default::default(),
            is_port_authorized: Default::default(),
            is_startup_wizard_completed: Default::default(),
            library_metadata_refresh_concurrency: Default::default(),
            library_monitor_delay: Default::default(),
            library_scan_fanout_concurrency: Default::default(),
            library_update_duration: Default::default(),
            log_file_retention_days: Default::default(),
            max_audiobook_resume: Default::default(),
            max_resume_pct: Default::default(),
            metadata_country_code: Default::default(),
            metadata_options: Default::default(),
            metadata_path: Default::default(),
            min_audiobook_resume: Default::default(),
            min_resume_duration_seconds: Default::default(),
            min_resume_pct: Default::default(),
            parallel_image_encoding_limit: Default::default(),
            path_substitutions: Default::default(),
            plugin_repositories: Default::default(),
            preferred_metadata_language: Default::default(),
            previous_version: Default::default(),
            previous_version_str: Default::default(),
            quick_connect_available: Default::default(),
            remote_client_bitrate_limit: Default::default(),
            save_metadata_hidden: Default::default(),
            server_name: Default::default(),
            skip_deserialization_for_basic_types: Default::default(),
            slow_response_threshold_ms: Default::default(),
            sort_remove_characters: Default::default(),
            sort_remove_words: Default::default(),
            sort_replace_characters: Default::default(),
            trickplay_options: Default::default(),
            ui_culture: Default::default(),
        }
    }
}

#[doc = "The server discovery info model."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ServerDiscoveryInfo {
    #[doc = "Gets the address."]
    #[serde(rename = "Address", default, skip_serializing_if = "Option::is_none")]
    pub address: Option<String>,
    #[doc = "Gets the endpoint address."]
    #[serde(
        rename = "EndpointAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub endpoint_address: Option<String>,
    #[doc = "Gets the server identifier."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

impl Default for ServerDiscoveryInfo {
    fn default() -> Self {
        Self {
            address: Default::default(),
            endpoint_address: Default::default(),
            id: Default::default(),
            name: Default::default(),
        }
    }
}

#[doc = "Server restarting down message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ServerRestartingMessage {
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for ServerRestartingMessage {
    fn default() -> Self {
        Self {
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "Server shutting down message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ServerShuttingDownMessage {
    #[doc = "Gets or sets the message id."]
    #[serde(rename = "MessageId", default, skip_serializing_if = "Option::is_none")]
    pub message_id: Option<uuid::Uuid>,
    #[serde(
        rename = "MessageType",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub message_type: Option<SessionMessageType>,
}

impl Default for ServerShuttingDownMessage {
    fn default() -> Self {
        Self {
            message_id: Default::default(),
            message_type: Default::default(),
        }
    }
}

#[doc = "The startup configuration DTO."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct StartupConfigurationDto {
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the preferred language for the metadata."]
    #[serde(
        rename = "PreferredMetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub preferred_metadata_language: Option<String>,
    #[doc = "Gets or sets the server name."]
    #[serde(
        rename = "ServerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_name: Option<String>,
    #[doc = "Gets or sets UI language culture."]
    #[serde(rename = "UICulture", default, skip_serializing_if = "Option::is_none")]
    pub ui_culture: Option<String>,
}

impl Default for StartupConfigurationDto {
    fn default() -> Self {
        Self {
            metadata_country_code: Default::default(),
            preferred_metadata_language: Default::default(),
            server_name: Default::default(),
            ui_culture: Default::default(),
        }
    }
}

#[doc = "Startup remote access dto."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct StartupRemoteAccessDto {
    #[doc = "Gets or sets a value indicating whether enable automatic port mapping."]
    #[serde(rename = "EnableAutomaticPortMapping")]
    pub enable_automatic_port_mapping: bool,
    #[doc = "Gets or sets a value indicating whether enable remote access."]
    #[serde(rename = "EnableRemoteAccess")]
    pub enable_remote_access: bool,
}

#[doc = "The startup user DTO."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct StartupUserDto {
    #[doc = "Gets or sets the username."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the user's password."]
    #[serde(rename = "Password", default, skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
}

impl Default for StartupUserDto {
    fn default() -> Self {
        Self {
            name: Default::default(),
            password: Default::default(),
        }
    }
}

#[doc = "Class SystemInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SystemInfo {
    #[doc = "Gets or sets the cache path."]
    #[serde(rename = "CachePath", default, skip_serializing_if = "Option::is_none")]
    pub cache_path: Option<String>,
    #[serde(rename = "CanLaunchWebBrowser", default)]
    pub can_launch_web_browser: bool,
    #[doc = "Gets or sets a value indicating whether this instance can self restart."]
    #[serde(
        rename = "CanSelfRestart",
        default = "crate::types::defaults::default_bool::<true>"
    )]
    pub can_self_restart: bool,
    #[doc = "Gets or sets the list of cast receiver applications."]
    #[serde(
        rename = "CastReceiverApplications",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cast_receiver_applications: Option<Vec<CastReceiverApplication>>,
    #[doc = "Gets or sets the completed installations."]
    #[serde(
        rename = "CompletedInstallations",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub completed_installations: Option<Vec<InstallationInfo>>,
    #[serde(
        rename = "EncoderLocation",
        default = "crate::types::defaults::system_info_encoder_location"
    )]
    pub encoder_location: Option<String>,
    #[doc = "Gets or sets a value indicating whether this instance has pending restart."]
    #[serde(
        rename = "HasPendingRestart",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub has_pending_restart: Option<bool>,
    #[doc = "Gets or sets a value indicating whether this instance has update available."]
    #[serde(rename = "HasUpdateAvailable", default)]
    pub has_update_available: bool,
    #[doc = "Gets or sets the id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[doc = "Gets or sets the internal metadata path."]
    #[serde(
        rename = "InternalMetadataPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_metadata_path: Option<String>,
    #[serde(
        rename = "IsShuttingDown",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_shutting_down: Option<bool>,
    #[doc = "Gets or sets the items by name path."]
    #[serde(
        rename = "ItemsByNamePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub items_by_name_path: Option<String>,
    #[doc = "Gets or sets the local address."]
    #[serde(
        rename = "LocalAddress",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub local_address: Option<String>,
    #[doc = "Gets or sets the log path."]
    #[serde(rename = "LogPath", default, skip_serializing_if = "Option::is_none")]
    pub log_path: Option<String>,
    #[doc = "Gets or sets the operating system."]
    #[serde(
        rename = "OperatingSystem",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operating_system: Option<String>,
    #[doc = "Gets or sets the display name of the operating system."]
    #[serde(
        rename = "OperatingSystemDisplayName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub operating_system_display_name: Option<String>,
    #[doc = "Gets or sets the package name."]
    #[serde(
        rename = "PackageName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_name: Option<String>,
    #[doc = "Gets or sets the product name. This is the AssemblyProduct name."]
    #[serde(
        rename = "ProductName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub product_name: Option<String>,
    #[doc = "Gets or sets the program data path."]
    #[serde(
        rename = "ProgramDataPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_data_path: Option<String>,
    #[doc = "Gets or sets the name of the server."]
    #[serde(
        rename = "ServerName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_name: Option<String>,
    #[doc = "Gets or sets a value indicating whether the startup wizard is completed."]
    #[serde(
        rename = "StartupWizardCompleted",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub startup_wizard_completed: Option<bool>,
    #[doc = "Gets or sets a value indicating whether [supports library monitor]."]
    #[serde(
        rename = "SupportsLibraryMonitor",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub supports_library_monitor: Option<bool>,
    #[serde(
        rename = "SystemArchitecture",
        default = "crate::types::defaults::system_info_system_architecture"
    )]
    pub system_architecture: Option<String>,
    #[doc = "Gets or sets the transcode path."]
    #[serde(
        rename = "TranscodingTempPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_temp_path: Option<String>,
    #[doc = "Gets or sets the server version."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets or sets the web UI resources path."]
    #[serde(rename = "WebPath", default, skip_serializing_if = "Option::is_none")]
    pub web_path: Option<String>,
    #[doc = "Gets or sets the web socket port number."]
    #[serde(
        rename = "WebSocketPortNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub web_socket_port_number: Option<i32>,
}

impl Default for SystemInfo {
    fn default() -> Self {
        Self {
            cache_path: Default::default(),
            can_launch_web_browser: Default::default(),
            can_self_restart: super::defaults::default_bool::<true>(),
            cast_receiver_applications: Default::default(),
            completed_installations: Default::default(),
            encoder_location: super::defaults::system_info_encoder_location(),
            has_pending_restart: Default::default(),
            has_update_available: Default::default(),
            id: Default::default(),
            internal_metadata_path: Default::default(),
            is_shutting_down: Default::default(),
            items_by_name_path: Default::default(),
            local_address: Default::default(),
            log_path: Default::default(),
            operating_system: Default::default(),
            operating_system_display_name: Default::default(),
            package_name: Default::default(),
            product_name: Default::default(),
            program_data_path: Default::default(),
            server_name: Default::default(),
            startup_wizard_completed: Default::default(),
            supports_library_monitor: Default::default(),
            system_architecture: super::defaults::system_info_system_architecture(),
            transcoding_temp_path: Default::default(),
            version: Default::default(),
            web_path: Default::default(),
            web_socket_port_number: Default::default(),
        }
    }
}

#[doc = "Contains informations about the systems storage."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SystemStorageDto {
    #[doc = "Gets or sets the Storage information of the cache folder."]
    #[serde(
        rename = "CacheFolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cache_folder: Option<FolderStorageDto>,
    #[doc = "Gets or sets the Storage information of the folder where images are cached."]
    #[serde(
        rename = "ImageCacheFolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub image_cache_folder: Option<FolderStorageDto>,
    #[doc = "Gets or sets the Storage information of the folder where metadata is stored."]
    #[serde(
        rename = "InternalMetadataFolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub internal_metadata_folder: Option<FolderStorageDto>,
    #[doc = "Gets or sets the storage informations of all libraries."]
    #[serde(rename = "Libraries", default, skip_serializing_if = "Vec::is_empty")]
    pub libraries: Vec<LibraryStorageDto>,
    #[doc = "Gets or sets the Storage information of the folder where logfiles are saved to."]
    #[serde(rename = "LogFolder", default, skip_serializing_if = "Option::is_none")]
    pub log_folder: Option<FolderStorageDto>,
    #[doc = "Gets or sets the Storage information of the program data folder."]
    #[serde(
        rename = "ProgramDataFolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub program_data_folder: Option<FolderStorageDto>,
    #[doc = "Gets or sets the Storage information of the transcoding cache."]
    #[serde(
        rename = "TranscodingTempFolder",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub transcoding_temp_folder: Option<FolderStorageDto>,
    #[doc = "Gets or sets the Storage information of the web UI resources folder."]
    #[serde(rename = "WebFolder", default, skip_serializing_if = "Option::is_none")]
    pub web_folder: Option<FolderStorageDto>,
}

impl Default for SystemStorageDto {
    fn default() -> Self {
        Self {
            cache_folder: Default::default(),
            image_cache_folder: Default::default(),
            internal_metadata_folder: Default::default(),
            libraries: Default::default(),
            log_folder: Default::default(),
            program_data_folder: Default::default(),
            transcoding_temp_folder: Default::default(),
            web_folder: Default::default(),
        }
    }
}

#[doc = "Class UserConfiguration."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct UserConfiguration {
    #[doc = "Gets or sets the audio language preference."]
    #[serde(
        rename = "AudioLanguagePreference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub audio_language_preference: Option<String>,
    #[doc = "Gets or sets the id of the selected cast receiver."]
    #[serde(
        rename = "CastReceiverId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub cast_receiver_id: Option<String>,
    #[serde(
        rename = "DisplayCollectionsView",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_collections_view: Option<bool>,
    #[serde(
        rename = "DisplayMissingEpisodes",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub display_missing_episodes: Option<bool>,
    #[serde(
        rename = "EnableLocalPassword",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_local_password: Option<bool>,
    #[serde(
        rename = "EnableNextEpisodeAutoPlay",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub enable_next_episode_auto_play: Option<bool>,
    #[serde(
        rename = "GroupedFolders",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub grouped_folders: Vec<uuid::Uuid>,
    #[serde(
        rename = "HidePlayedInLatest",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub hide_played_in_latest: Option<bool>,
    #[serde(
        rename = "LatestItemsExcludes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub latest_items_excludes: Vec<uuid::Uuid>,
    #[serde(
        rename = "MyMediaExcludes",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub my_media_excludes: Vec<uuid::Uuid>,
    #[serde(
        rename = "OrderedViews",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub ordered_views: Vec<uuid::Uuid>,
    #[doc = "Gets or sets a value indicating whether [play default audio track]."]
    #[serde(
        rename = "PlayDefaultAudioTrack",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub play_default_audio_track: Option<bool>,
    #[serde(
        rename = "RememberAudioSelections",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remember_audio_selections: Option<bool>,
    #[serde(
        rename = "RememberSubtitleSelections",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub remember_subtitle_selections: Option<bool>,
    #[doc = "Gets or sets the subtitle language preference."]
    #[serde(
        rename = "SubtitleLanguagePreference",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_language_preference: Option<String>,
    #[serde(
        rename = "SubtitleMode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub subtitle_mode: Option<SubtitlePlaybackMode>,
}

impl Default for UserConfiguration {
    fn default() -> Self {
        Self {
            audio_language_preference: Default::default(),
            cast_receiver_id: Default::default(),
            display_collections_view: Default::default(),
            display_missing_episodes: Default::default(),
            enable_local_password: Default::default(),
            enable_next_episode_auto_play: Default::default(),
            grouped_folders: Default::default(),
            hide_played_in_latest: Default::default(),
            latest_items_excludes: Default::default(),
            my_media_excludes: Default::default(),
            ordered_views: Default::default(),
            play_default_audio_track: Default::default(),
            remember_audio_selections: Default::default(),
            remember_subtitle_selections: Default::default(),
            subtitle_language_preference: Default::default(),
            subtitle_mode: Default::default(),
        }
    }
}
