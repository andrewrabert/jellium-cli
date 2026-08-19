#[doc = "Manifest type for backups internal structure."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BackupManifestDto {
    #[doc = "Gets or sets the backup engine version this backup was created with."]
    #[serde(
        rename = "BackupEngineVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backup_engine_version: Option<String>,
    #[doc = "Gets or sets the date this backup was created with."]
    #[serde(
        rename = "DateCreated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub date_created: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the contents of the backup archive."]
    #[serde(rename = "Options", default, skip_serializing_if = "Option::is_none")]
    pub options: Option<BackupOptionsDto>,
    #[doc = "Gets or sets the path to the backup on the system."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[doc = "Gets or sets the jellyfin version this backup was created with."]
    #[serde(
        rename = "ServerVersion",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub server_version: Option<String>,
}

#[doc = "Defines the optional contents of the backup archive."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BackupOptionsDto {
    #[doc = "Gets or sets a value indicating whether the archive contains the Database contents."]
    #[serde(rename = "Database", default, skip_serializing_if = "Option::is_none")]
    pub database: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the archive contains the Metadata contents."]
    #[serde(rename = "Metadata", default, skip_serializing_if = "Option::is_none")]
    pub metadata: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the archive contains the Subtitle contents."]
    #[serde(rename = "Subtitles", default, skip_serializing_if = "Option::is_none")]
    pub subtitles: Option<bool>,
    #[doc = "Gets or sets a value indicating whether the archive contains the Trickplay contents."]
    #[serde(rename = "Trickplay", default, skip_serializing_if = "Option::is_none")]
    pub trickplay: Option<bool>,
}

#[doc = "Defines properties used to start a restore process."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
pub struct BackupRestoreRequestDto {
    #[doc = "Gets or Sets the name of the backup archive to restore from. Must be present in MediaBrowser.Common.Configuration.IApplicationPaths.BackupPath."]
    #[serde(
        rename = "ArchiveFileName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub archive_file_name: Option<String>,
}
