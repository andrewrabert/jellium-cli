use crate::types;

/// What `/Channels/{channelId}/Items` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct GetChannelItems<'q> {
    /// Optional. Specify additional fields of information to return in the output.
    pub fields: Option<&'q Vec<types::ItemFields>>,
    /// Optional. Specify additional filters to apply.
    pub filters: Option<&'q Vec<types::ItemFilter>>,
    /// Optional. Folder Id.
    pub folder_id: Option<&'q uuid::Uuid>,
    /// Optional. The maximum number of records to return.
    pub limit: Option<i32>,
    /// Optional. Specify one or more sort orders, comma delimited. Options: Album, AlbumArtist, Artist, Budget, CommunityRating, CriticRating, DateCreated, DatePlayed, PlayCount, PremiereDate, ProductionYear, SortName, Random, Revenue, Runtime.
    pub sort_by: Option<&'q Vec<types::ItemSortBy>>,
    /// Optional. Sort Order - Ascending,Descending.
    pub sort_order: Option<&'q Vec<types::SortOrder>>,
    /// Optional. The record index to start at. All items with a lower index will be dropped from the results.
    pub start_index: Option<i32>,
    /// Optional. User Id.
    pub user_id: Option<&'q uuid::Uuid>,
}
