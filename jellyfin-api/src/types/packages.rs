use super::*;

#[doc = "Class BasePluginConfiguration."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BasePluginConfiguration {}
#[doc = "Defines the MediaBrowser.Common.Plugins.IPlugin."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct IPlugin {
    #[doc = "Gets the path to the assembly file."]
    #[serde(
        rename = "AssemblyFilePath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub assembly_file_path: Option<String>,
    #[doc = "Gets a value indicating whether the plugin can be uninstalled."]
    #[serde(
        rename = "CanUninstall",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_uninstall: Option<bool>,
    #[doc = "Gets the full path to the data folder, where the plugin can store any miscellaneous files needed."]
    #[serde(
        rename = "DataFolderPath",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub data_folder_path: Option<String>,
    #[doc = "Gets the Description."]
    #[serde(
        rename = "Description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[doc = "Gets the unique id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets the name of the plugin."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets the plugin version."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[doc = "Class InstallationInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct InstallationInfo {
    #[doc = "Gets or sets the changelog for this version."]
    #[serde(rename = "Changelog", default, skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    #[doc = "Gets or sets a checksum for the binary."]
    #[serde(rename = "Checksum", default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[doc = "Gets or sets the Id."]
    #[serde(rename = "Guid", default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<uuid::Uuid>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets package information for the installation."]
    #[serde(
        rename = "PackageInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub package_info: Option<PackageInfo>,
    #[doc = "Gets or sets the source URL."]
    #[serde(rename = "SourceUrl", default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[doc = "Gets or sets the version."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[doc = "Class PackageInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PackageInfo {
    #[doc = "Gets or sets the category."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[doc = "Gets or sets a long description of the plugin containing features or helpful explanations."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[doc = "Gets or sets the guid of the assembly associated with this plugin.\r\nThis is used to identify the proper item for automatic updates."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub guid: Option<uuid::Uuid>,
    #[doc = "Gets or sets the image url for the package."]
    #[serde(rename = "imageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets a short overview of what the plugin does."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[doc = "Gets or sets the owner."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    #[doc = "Gets or sets the versions."]
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub versions: Vec<VersionInfo>,
}

#[doc = "This is a serializable stub class that is used by the api to provide information about installed plugins."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PluginInfo {
    #[doc = "Gets or sets a value indicating whether the plugin can be uninstalled."]
    #[serde(
        rename = "CanUninstall",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub can_uninstall: Option<bool>,
    #[doc = "Gets or sets the name of the configuration file."]
    #[serde(
        rename = "ConfigurationFileName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub configuration_file_name: Option<String>,
    #[doc = "Gets or sets the description."]
    #[serde(
        rename = "Description",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub description: Option<String>,
    #[doc = "Gets or sets a value indicating whether this plugin has a valid image."]
    #[serde(rename = "HasImage", default, skip_serializing_if = "Option::is_none")]
    pub has_image: Option<bool>,
    #[doc = "Gets or sets the unique id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<PluginStatus>,
    #[doc = "Gets or sets the version."]
    #[serde(rename = "Version", default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
}

#[doc = "Plugin installation cancelled message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PluginInstallationCancelledMessage {
    #[doc = "Class InstallationInfo."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<InstallationInfo>,
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

#[doc = "Plugin installation completed message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PluginInstallationCompletedMessage {
    #[doc = "Class InstallationInfo."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<InstallationInfo>,
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

#[doc = "Plugin installation failed message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PluginInstallationFailedMessage {
    #[doc = "Class InstallationInfo."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<InstallationInfo>,
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

#[doc = "Package installing message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PluginInstallingMessage {
    #[doc = "Class InstallationInfo."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<InstallationInfo>,
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

#[derive(
    serde::Deserialize, serde::Serialize, Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd,
)]
pub enum PluginStatus {
    Active,
    Restart,
    Deleted,
    Superseded,
    Superceded,
    Malfunctioned,
    NotSupported,
    Disabled,
}

impl std::fmt::Display for PluginStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Active => f.write_str("Active"),
            Self::Restart => f.write_str("Restart"),
            Self::Deleted => f.write_str("Deleted"),
            Self::Superseded => f.write_str("Superseded"),
            Self::Superceded => f.write_str("Superceded"),
            Self::Malfunctioned => f.write_str("Malfunctioned"),
            Self::NotSupported => f.write_str("NotSupported"),
            Self::Disabled => f.write_str("Disabled"),
        }
    }
}

impl std::str::FromStr for PluginStatus {
    type Err = super::error::ConversionError;
    fn from_str(value: &str) -> Result<Self, super::error::ConversionError> {
        match value {
            "Active" => Ok(Self::Active),
            "Restart" => Ok(Self::Restart),
            "Deleted" => Ok(Self::Deleted),
            "Superseded" => Ok(Self::Superseded),
            "Superceded" => Ok(Self::Superceded),
            "Malfunctioned" => Ok(Self::Malfunctioned),
            "NotSupported" => Ok(Self::NotSupported),
            "Disabled" => Ok(Self::Disabled),
            _ => Err("invalid value".into()),
        }
    }
}

impl TryFrom<&str> for PluginStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: &str) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<&String> for PluginStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: &String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

impl TryFrom<String> for PluginStatus {
    type Error = super::error::ConversionError;
    fn try_from(value: String) -> Result<Self, super::error::ConversionError> {
        value.parse()
    }
}

#[doc = "Plugin uninstalled message."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct PluginUninstalledMessage {
    #[doc = "This is a serializable stub class that is used by the api to provide information about installed plugins."]
    #[serde(rename = "Data", default, skip_serializing_if = "Option::is_none")]
    pub data: Option<PluginInfo>,
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

#[doc = "Class RepositoryInfo."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct RepositoryInfo {
    #[doc = "Gets or sets a value indicating whether the repository is enabled."]
    #[serde(rename = "Enabled", default, skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the URL."]
    #[serde(rename = "Url", default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

#[doc = "Defines the MediaBrowser.Model.Updates.VersionInfo class."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct VersionInfo {
    #[doc = "Gets or sets the changelog for this version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub changelog: Option<String>,
    #[doc = "Gets or sets a checksum for the binary."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub checksum: Option<String>,
    #[doc = "Gets or sets the repository name."]
    #[serde(
        rename = "repositoryName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_name: Option<String>,
    #[doc = "Gets or sets the repository url."]
    #[serde(
        rename = "repositoryUrl",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub repository_url: Option<String>,
    #[doc = "Gets or sets the source URL."]
    #[serde(rename = "sourceUrl", default, skip_serializing_if = "Option::is_none")]
    pub source_url: Option<String>,
    #[doc = "Gets or sets the ABI that this version was built against."]
    #[serde(rename = "targetAbi", default, skip_serializing_if = "Option::is_none")]
    pub target_abi: Option<String>,
    #[doc = "Gets or sets a timestamp of when the binary was built."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<String>,
    #[doc = "Gets or sets the version."]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub version: Option<String>,
    #[doc = "Gets the version as a System.Version."]
    #[serde(
        rename = "VersionNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub version_number: Option<String>,
}
