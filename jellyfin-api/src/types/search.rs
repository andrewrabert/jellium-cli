use super::*;

#[doc = "`AlbumInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AlbumInfo {
    #[doc = "Gets or sets the album artist."]
    #[serde(
        rename = "AlbumArtists",
        default,
        skip_serializing_if = "Vec::is_empty"
    )]
    pub album_artists: Vec<String>,
    #[doc = "Gets or sets the artist provider ids."]
    #[serde(
        rename = "ArtistProviderIds",
        default,
        skip_serializing_if = ":: std :: collections :: HashMap::is_empty"
    )]
    pub artist_provider_ids: std::collections::HashMap<String, Option<String>>,
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "SongInfos", default, skip_serializing_if = "Vec::is_empty")]
    pub song_infos: Vec<SongInfo>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for AlbumInfo {
    fn default() -> Self {
        Self {
            album_artists: Default::default(),
            artist_provider_ids: Default::default(),
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            song_infos: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`AlbumInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct AlbumInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<AlbumInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for AlbumInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "`ArtistInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ArtistInfo {
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(rename = "SongInfos", default, skip_serializing_if = "Vec::is_empty")]
    pub song_infos: Vec<SongInfo>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for ArtistInfo {
    fn default() -> Self {
        Self {
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            song_infos: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`ArtistInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct ArtistInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<ArtistInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for ArtistInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "`BookInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BookInfo {
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(
        rename = "SeriesName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub series_name: Option<String>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for BookInfo {
    fn default() -> Self {
        Self {
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            series_name: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`BookInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BookInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<BookInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for BookInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "`BoxSetInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BoxSetInfo {
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for BoxSetInfo {
    fn default() -> Self {
        Self {
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`BoxSetInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct BoxSetInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<BoxSetInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for BoxSetInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "`MovieInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MovieInfo {
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for MovieInfo {
    fn default() -> Self {
        Self {
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`MovieInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MovieInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<MovieInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for MovieInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "`MusicVideoInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MusicVideoInfo {
    #[serde(rename = "Artists", default, skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<String>>,
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for MusicVideoInfo {
    fn default() -> Self {
        Self {
            artists: Default::default(),
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`MusicVideoInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct MusicVideoInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<MusicVideoInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for MusicVideoInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "`PersonLookupInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PersonLookupInfo {
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for PersonLookupInfo {
    fn default() -> Self {
        Self {
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`PersonLookupInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct PersonLookupInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<PersonLookupInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for PersonLookupInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "`RemoteSearchResult`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct RemoteSearchResult {
    #[serde(
        rename = "AlbumArtist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_artist: Option<Box<RemoteSearchResult>>,
    #[serde(rename = "Artists", default, skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<RemoteSearchResult>>,
    #[serde(rename = "ImageUrl", default, skip_serializing_if = "Option::is_none")]
    pub image_url: Option<String>,
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IndexNumberEnd",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number_end: Option<i32>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "Overview", default, skip_serializing_if = "Option::is_none")]
    pub overview: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the year."]
    #[serde(
        rename = "ProductionYear",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_year: Option<i32>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for RemoteSearchResult {
    fn default() -> Self {
        Self {
            album_artist: Default::default(),
            artists: Default::default(),
            image_url: Default::default(),
            index_number: Default::default(),
            index_number_end: Default::default(),
            name: Default::default(),
            overview: Default::default(),
            parent_index_number: Default::default(),
            premiere_date: Default::default(),
            production_year: Default::default(),
            provider_ids: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "Class SearchHintResult."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SearchHint {
    #[doc = "Gets or sets the album."]
    #[serde(rename = "Album", default, skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[doc = "Gets or sets the album artist."]
    #[serde(
        rename = "AlbumArtist",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_artist: Option<String>,
    #[doc = "Gets or sets the album id."]
    #[serde(rename = "AlbumId", default, skip_serializing_if = "Option::is_none")]
    pub album_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the artists."]
    #[serde(rename = "Artists", default, skip_serializing_if = "Vec::is_empty")]
    pub artists: Vec<String>,
    #[doc = "Gets or sets the backdrop image item identifier."]
    #[serde(
        rename = "BackdropImageItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backdrop_image_item_id: Option<String>,
    #[doc = "Gets or sets the backdrop image tag."]
    #[serde(
        rename = "BackdropImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub backdrop_image_tag: Option<String>,
    #[doc = "Gets or sets the channel identifier."]
    #[serde(rename = "ChannelId", default, skip_serializing_if = "Option::is_none")]
    pub channel_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the name of the channel."]
    #[serde(
        rename = "ChannelName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub channel_name: Option<String>,
    #[doc = "Gets or sets the end date."]
    #[serde(rename = "EndDate", default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the episode count."]
    #[serde(
        rename = "EpisodeCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub episode_count: Option<i32>,
    #[doc = "Gets or sets the item id."]
    #[serde(rename = "Id", default, skip_serializing_if = "Option::is_none")]
    pub id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the index number."]
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[doc = "Gets or sets a value indicating whether this instance is folder."]
    #[serde(rename = "IsFolder", default, skip_serializing_if = "Option::is_none")]
    pub is_folder: Option<bool>,
    #[doc = "Gets or sets the item id."]
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[doc = "Gets or sets the matched term."]
    #[serde(
        rename = "MatchedTerm",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub matched_term: Option<String>,
    #[serde(rename = "MediaType", default, skip_serializing_if = "Option::is_none")]
    pub media_type: Option<MediaType>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the parent index number."]
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the primary image aspect ratio."]
    #[serde(
        rename = "PrimaryImageAspectRatio",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_aspect_ratio: Option<f64>,
    #[doc = "Gets or sets the image tag."]
    #[serde(
        rename = "PrimaryImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub primary_image_tag: Option<String>,
    #[doc = "Gets or sets the production year."]
    #[serde(
        rename = "ProductionYear",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub production_year: Option<i32>,
    #[doc = "Gets or sets the run time ticks."]
    #[serde(
        rename = "RunTimeTicks",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub run_time_ticks: Option<i64>,
    #[doc = "Gets or sets the series."]
    #[serde(rename = "Series", default, skip_serializing_if = "Option::is_none")]
    pub series: Option<String>,
    #[doc = "Gets or sets the song count."]
    #[serde(rename = "SongCount", default, skip_serializing_if = "Option::is_none")]
    pub song_count: Option<i32>,
    #[doc = "Gets or sets the start date."]
    #[serde(rename = "StartDate", default, skip_serializing_if = "Option::is_none")]
    pub start_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the status."]
    #[serde(rename = "Status", default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[doc = "Gets or sets the thumb image item identifier."]
    #[serde(
        rename = "ThumbImageItemId",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thumb_image_item_id: Option<String>,
    #[doc = "Gets or sets the thumb image tag."]
    #[serde(
        rename = "ThumbImageTag",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub thumb_image_tag: Option<String>,
    #[serde(rename = "Type", default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<BaseItemKind>,
}

impl Default for SearchHint {
    fn default() -> Self {
        Self {
            album: Default::default(),
            album_artist: Default::default(),
            album_id: Default::default(),
            artists: Default::default(),
            backdrop_image_item_id: Default::default(),
            backdrop_image_tag: Default::default(),
            channel_id: Default::default(),
            channel_name: Default::default(),
            end_date: Default::default(),
            episode_count: Default::default(),
            id: Default::default(),
            index_number: Default::default(),
            is_folder: Default::default(),
            item_id: Default::default(),
            matched_term: Default::default(),
            media_type: Default::default(),
            name: Default::default(),
            parent_index_number: Default::default(),
            primary_image_aspect_ratio: Default::default(),
            primary_image_tag: Default::default(),
            production_year: Default::default(),
            run_time_ticks: Default::default(),
            series: Default::default(),
            song_count: Default::default(),
            start_date: Default::default(),
            status: Default::default(),
            thumb_image_item_id: Default::default(),
            thumb_image_tag: Default::default(),
            type_: Default::default(),
        }
    }
}

#[doc = "Class SearchHintResult."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SearchHintResult {
    #[doc = "Gets the search hints."]
    #[serde(rename = "SearchHints", default, skip_serializing_if = "Vec::is_empty")]
    pub search_hints: Vec<SearchHint>,
    #[doc = "Gets the total record count."]
    #[serde(
        rename = "TotalRecordCount",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub total_record_count: Option<i32>,
}

impl Default for SearchHintResult {
    fn default() -> Self {
        Self {
            search_hints: Default::default(),
            total_record_count: Default::default(),
        }
    }
}

#[doc = "`SeriesInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SeriesInfo {
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for SeriesInfo {
    fn default() -> Self {
        Self {
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`SeriesInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SeriesInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<SeriesInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for SeriesInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}

#[doc = "`SongInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct SongInfo {
    #[serde(rename = "Album", default, skip_serializing_if = "Option::is_none")]
    pub album: Option<String>,
    #[serde(
        rename = "AlbumArtists",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub album_artists: Option<Vec<String>>,
    #[serde(rename = "Artists", default, skip_serializing_if = "Option::is_none")]
    pub artists: Option<Vec<String>>,
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for SongInfo {
    fn default() -> Self {
        Self {
            album: Default::default(),
            album_artists: Default::default(),
            artists: Default::default(),
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`TrailerInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TrailerInfo {
    #[serde(
        rename = "IndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub index_number: Option<i32>,
    #[serde(
        rename = "IsAutomated",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub is_automated: Option<bool>,
    #[doc = "Gets or sets the metadata country code."]
    #[serde(
        rename = "MetadataCountryCode",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_country_code: Option<String>,
    #[doc = "Gets or sets the metadata language."]
    #[serde(
        rename = "MetadataLanguage",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub metadata_language: Option<String>,
    #[doc = "Gets or sets the name."]
    #[serde(rename = "Name", default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[doc = "Gets or sets the original title."]
    #[serde(
        rename = "OriginalTitle",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub original_title: Option<String>,
    #[serde(
        rename = "ParentIndexNumber",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub parent_index_number: Option<i32>,
    #[doc = "Gets or sets the path."]
    #[serde(rename = "Path", default, skip_serializing_if = "Option::is_none")]
    pub path: Option<String>,
    #[serde(
        rename = "PremiereDate",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub premiere_date: Option<chrono::DateTime<chrono::Utc>>,
    #[doc = "Gets or sets the provider ids."]
    #[serde(
        rename = "ProviderIds",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub provider_ids: Option<std::collections::HashMap<String, Option<String>>>,
    #[doc = "Gets or sets the year."]
    #[serde(rename = "Year", default, skip_serializing_if = "Option::is_none")]
    pub year: Option<i32>,
}

impl Default for TrailerInfo {
    fn default() -> Self {
        Self {
            index_number: Default::default(),
            is_automated: Default::default(),
            metadata_country_code: Default::default(),
            metadata_language: Default::default(),
            name: Default::default(),
            original_title: Default::default(),
            parent_index_number: Default::default(),
            path: Default::default(),
            premiere_date: Default::default(),
            provider_ids: Default::default(),
            year: Default::default(),
        }
    }
}

#[doc = "`TrailerInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug)]
pub struct TrailerInfoRemoteSearchQuery {
    #[doc = "Gets or sets a value indicating whether disabled providers should be included."]
    #[serde(
        rename = "IncludeDisabledProviders",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub include_disabled_providers: Option<bool>,
    #[serde(rename = "ItemId", default, skip_serializing_if = "Option::is_none")]
    pub item_id: Option<uuid::Uuid>,
    #[serde(
        rename = "SearchInfo",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_info: Option<TrailerInfo>,
    #[doc = "Gets or sets the provider name to search within if set."]
    #[serde(
        rename = "SearchProviderName",
        default,
        skip_serializing_if = "Option::is_none"
    )]
    pub search_provider_name: Option<String>,
}

impl Default for TrailerInfoRemoteSearchQuery {
    fn default() -> Self {
        Self {
            include_disabled_providers: Default::default(),
            item_id: Default::default(),
            search_info: Default::default(),
            search_provider_name: Default::default(),
        }
    }
}
