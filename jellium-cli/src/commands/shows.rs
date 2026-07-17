use clap::Subcommand;
use jellyfin_api::types::{ItemFields, ItemSortBy};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum ShowsCommand {
    /// Get episodes for a TV series
    Episodes {
        /// The series ID
        series_id: Uuid,

        /// Filter by season number
        #[arg(long)]
        season: Option<i32>,

        /// Filter by season ID
        #[arg(long)]
        season_id: Option<Uuid>,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Sort order
        #[arg(long)]
        sort_by: Option<ItemSortBy>,

        /// Return items adjacent to this item
        #[arg(long)]
        adjacent_to: Option<Uuid>,

        /// Filter by missing episodes
        #[arg(long)]
        is_missing: Option<bool>,

        /// Skip to a given item
        #[arg(long)]
        start_item_id: Option<Uuid>,
    },

    /// Get seasons for a TV series
    Seasons {
        /// The series ID
        series_id: Uuid,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Filter by missing episodes
        #[arg(long)]
        is_missing: Option<bool>,

        /// Filter by special season
        #[arg(long)]
        is_special_season: Option<bool>,

        /// Return items adjacent to this item
        #[arg(long)]
        adjacent_to: Option<Uuid>,
    },

    /// Get next up episodes
    NextUp {
        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Filter by series ID
        #[arg(long)]
        series_id: Option<Uuid>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,

        /// Disable first episode
        #[arg(long)]
        disable_first_episode: Option<bool>,

        /// Include resumable episodes
        #[arg(long)]
        enable_resumable: Option<bool>,

        /// Include watched episodes
        #[arg(long)]
        enable_rewatching: Option<bool>,
    },

    /// Get upcoming episodes
    Upcoming {
        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Parent folder ID
        #[arg(long)]
        parent_id: Option<Uuid>,
    },

    /// Get similar shows
    Similar {
        /// The item ID
        item_id: Uuid,

        /// User ID
        #[arg(long)]
        user_id: Option<Uuid>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &ShowsCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        ShowsCommand::Episodes {
            series_id,
            season,
            season_id,
            user_id: uid,
            fields,
            limit,
            start_index,
            sort_by,
            adjacent_to,
            is_missing,
            start_item_id,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_episodes(
                    series_id,
                    adjacent_to.as_ref(),
                    None, // enable_image_types
                    None, // enable_images
                    None, // enable_user_data
                    fields.as_ref(),
                    None, // image_type_limit
                    *is_missing,
                    *limit,
                    *season,
                    season_id.as_ref(),
                    *sort_by,
                    *start_index,
                    start_item_id.as_ref(),
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ShowsCommand::Seasons {
            series_id,
            user_id: uid,
            fields,
            is_missing,
            is_special_season,
            adjacent_to,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_seasons(
                    series_id,
                    adjacent_to.as_ref(),
                    None, // enable_image_types
                    None, // enable_images
                    None, // enable_user_data
                    fields.as_ref(),
                    None, // image_type_limit
                    *is_missing,
                    *is_special_season,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        ShowsCommand::NextUp {
            user_id: uid,
            limit,
            start_index,
            fields,
            series_id,
            parent_id,
            disable_first_episode,
            enable_resumable,
            enable_rewatching,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_next_up(
                    *disable_first_episode,
                    None, // enable_image_types
                    None, // enable_images
                    *enable_resumable,
                    *enable_rewatching,
                    None, // enable_total_record_count
                    None, // enable_user_data
                    fields.as_ref(),
                    None, // image_type_limit
                    *limit,
                    None, // next_up_date_cutoff
                    parent_id.as_ref(),
                    series_id.as_ref(),
                    *start_index,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ShowsCommand::Upcoming {
            user_id: uid,
            limit,
            start_index,
            fields,
            parent_id,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_upcoming_episodes(
                    None, // enable_image_types
                    None, // enable_images
                    None, // enable_user_data
                    fields.as_ref(),
                    None, // image_type_limit
                    *limit,
                    parent_id.as_ref(),
                    *start_index,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        ShowsCommand::Similar {
            item_id,
            user_id: uid,
            limit,
            fields,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_similar_shows(
                    item_id,
                    None, // exclude_artist_ids
                    fields.as_ref(),
                    *limit,
                    Some(effective_uid),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
