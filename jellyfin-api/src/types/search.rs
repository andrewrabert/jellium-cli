use super::*;

#[doc = "`AlbumInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`AlbumInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`ArtistInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`ArtistInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`BookInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`BookInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`BoxSetInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`BoxSetInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`MovieInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`MovieInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`MusicVideoInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`MusicVideoInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`PersonLookupInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`PersonLookupInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`RemoteSearchResult`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "Class SearchHintResult."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "Class SearchHintResult."]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`SeriesInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`SeriesInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`SongInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`TrailerInfo`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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

#[doc = "`TrailerInfoRemoteSearchQuery`"]
#[derive(serde::Deserialize, serde::Serialize, Clone, Debug, Default)]
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
