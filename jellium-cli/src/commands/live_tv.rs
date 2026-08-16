use clap::Subcommand;
use jellyfin_api::types::{ChannelType, ItemFields, ItemSortBy, RecordingStatus, SortOrder};
use uuid::Uuid;

#[derive(Subcommand)]
pub enum LiveTvCommand {
    /// Get available live TV channels
    Channels {
        /// Include current program info
        #[arg(long)]
        add_current_program: Option<bool>,

        /// Sort favorites to the top
        #[arg(long)]
        enable_favorite_sorting: Option<bool>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Filter by disliked channels
        #[arg(long)]
        is_disliked: Option<bool>,

        /// Filter by favorite channels
        #[arg(long)]
        is_favorite: Option<bool>,

        /// Filter for kids content
        #[arg(long)]
        is_kids: Option<bool>,

        /// Filter by liked channels
        #[arg(long)]
        is_liked: Option<bool>,

        /// Filter for movies
        #[arg(long)]
        is_movie: Option<bool>,

        /// Filter for news
        #[arg(long)]
        is_news: Option<bool>,

        /// Filter for series
        #[arg(long)]
        is_series: Option<bool>,

        /// Filter for sports
        #[arg(long)]
        is_sports: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Sort by fields (comma separated)
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order
        #[arg(long)]
        sort_order: Option<SortOrder>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Filter by channel type
        #[arg(long)]
        type_: Option<ChannelType>,

        /// Filter by user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get a live TV channel
    Channel {
        /// The channel ID
        channel_id: Uuid,

        /// Attach user data
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get guide info
    GuideInfo,

    /// Get available live TV services info
    Info,

    /// Get available live TV programs (EPG)
    Programs {
        /// Filter by channel IDs (comma separated)
        #[arg(long, value_delimiter = ',')]
        channel_ids: Option<Vec<Uuid>>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Filter by programs that have completed airing
        #[arg(long)]
        has_aired: Option<bool>,

        /// Filter by programs currently airing
        #[arg(long)]
        is_airing: Option<bool>,

        /// Filter for kids content
        #[arg(long)]
        is_kids: Option<bool>,

        /// Filter for movies
        #[arg(long)]
        is_movie: Option<bool>,

        /// Filter for news
        #[arg(long)]
        is_news: Option<bool>,

        /// Filter for series
        #[arg(long)]
        is_series: Option<bool>,

        /// Filter for sports
        #[arg(long)]
        is_sports: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Filter by series timer ID
        #[arg(long)]
        series_timer_id: Option<String>,

        /// Sort by fields (comma separated)
        #[arg(long, value_delimiter = ',')]
        sort_by: Option<Vec<ItemSortBy>>,

        /// Sort order (comma separated)
        #[arg(long, value_delimiter = ',')]
        sort_order: Option<Vec<SortOrder>>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Filter by user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get a live TV program
    Program {
        /// The program ID
        program_id: String,

        /// Attach user data
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get recommended live TV programs
    RecommendedPrograms {
        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Filter by programs that have completed airing
        #[arg(long)]
        has_aired: Option<bool>,

        /// Filter by programs currently airing
        #[arg(long)]
        is_airing: Option<bool>,

        /// Filter for kids content
        #[arg(long)]
        is_kids: Option<bool>,

        /// Filter for movies
        #[arg(long)]
        is_movie: Option<bool>,

        /// Filter for news
        #[arg(long)]
        is_news: Option<bool>,

        /// Filter for series
        #[arg(long)]
        is_series: Option<bool>,

        /// Filter for sports
        #[arg(long)]
        is_sports: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Filter by user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get live TV recordings
    Recordings {
        /// Filter by channel ID
        #[arg(long)]
        channel_id: Option<String>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Filter by recordings in progress
        #[arg(long)]
        is_in_progress: Option<bool>,

        /// Filter for kids content
        #[arg(long)]
        is_kids: Option<bool>,

        /// Filter for library items
        #[arg(long)]
        is_library_item: Option<bool>,

        /// Filter for movies
        #[arg(long)]
        is_movie: Option<bool>,

        /// Filter for news
        #[arg(long)]
        is_news: Option<bool>,

        /// Filter for series
        #[arg(long)]
        is_series: Option<bool>,

        /// Filter for sports
        #[arg(long)]
        is_sports: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Filter by series timer ID
        #[arg(long)]
        series_timer_id: Option<String>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Filter by recording status
        #[arg(long)]
        status: Option<RecordingStatus>,

        /// Filter by user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get a live TV recording
    Recording {
        /// The recording ID
        recording_id: Uuid,

        /// Attach user data
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Delete a live TV recording
    DeleteRecording {
        /// The recording ID
        recording_id: Uuid,
    },

    /// Get recording folders
    RecordingFolders {
        /// Filter by user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get recording groups
    RecordingGroups {
        /// Filter by user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get a recording group
    RecordingGroup {
        /// The group ID
        group_id: Uuid,
    },

    /// Get recording series
    RecordingSeries {
        /// Filter by channel ID
        #[arg(long)]
        channel_id: Option<String>,

        /// Additional fields to return
        #[arg(long, value_delimiter = ',')]
        fields: Option<Vec<ItemFields>>,

        /// Filter by recording group ID
        #[arg(long)]
        group_id: Option<String>,

        /// Filter by recordings in progress
        #[arg(long)]
        is_in_progress: Option<bool>,

        /// Maximum number of records to return
        #[arg(long)]
        limit: Option<i32>,

        /// Filter by series timer ID
        #[arg(long)]
        series_timer_id: Option<String>,

        /// Record index to start at
        #[arg(long)]
        start_index: Option<i32>,

        /// Filter by recording status
        #[arg(long)]
        status: Option<RecordingStatus>,

        /// Filter by user ID
        #[arg(long)]
        user_id: Option<Uuid>,
    },

    /// Get series timers
    SeriesTimers {
        /// Sort by (SortName or Priority)
        #[arg(long)]
        sort_by: Option<String>,

        /// Sort order
        #[arg(long)]
        sort_order: Option<SortOrder>,
    },

    /// Create a series timer (reads SeriesTimerInfoDto JSON from stdin)
    CreateSeriesTimer,

    /// Get a series timer
    SeriesTimer {
        /// The timer ID
        timer_id: String,
    },

    /// Update a series timer (reads SeriesTimerInfoDto JSON from stdin)
    UpdateSeriesTimer {
        /// The timer ID
        timer_id: String,
    },

    /// Cancel a series timer
    CancelSeriesTimer {
        /// The timer ID
        timer_id: String,
    },

    /// Get live TV timers
    Timers {
        /// Filter by channel ID
        #[arg(long)]
        channel_id: Option<String>,

        /// Filter by active timers
        #[arg(long)]
        is_active: Option<bool>,

        /// Filter by series timer ID
        #[arg(long)]
        series_timer_id: Option<String>,
    },

    /// Create a timer (reads TimerInfoDto JSON from stdin)
    CreateTimer,

    /// Get a timer
    Timer {
        /// The timer ID
        timer_id: String,
    },

    /// Update a timer (reads TimerInfoDto JSON from stdin)
    UpdateTimer {
        /// The timer ID
        timer_id: String,
    },

    /// Cancel a timer
    CancelTimer {
        /// The timer ID
        timer_id: String,
    },

    /// Get default timer values
    DefaultTimer {
        /// Optional program ID to base defaults on
        #[arg(long)]
        program_id: Option<String>,
    },

    /// Add a listing provider (reads ListingsProviderInfo JSON from stdin)
    AddListingProvider {
        /// Password
        #[arg(long)]
        pw: Option<String>,

        /// Validate listings
        #[arg(long)]
        validate_listings: Option<bool>,

        /// Validate login
        #[arg(long)]
        validate_login: Option<bool>,
    },

    /// Delete a listing provider
    DeleteListingProvider {
        /// The listing provider ID
        #[arg(long)]
        id: Option<String>,
    },

    /// Get default listing provider info
    DefaultListingProvider,

    /// Get available lineups
    Lineups {
        /// Country
        #[arg(long)]
        country: Option<String>,

        /// Provider ID
        #[arg(long)]
        id: Option<String>,

        /// Location
        #[arg(long)]
        location: Option<String>,

        /// Provider type
        #[arg(long)]
        type_: Option<String>,
    },

    /// Get SchedulesDirect countries
    SchedulesDirectCountries,

    /// Add a tuner host (reads TunerHostInfo JSON from stdin)
    AddTunerHost,

    /// Delete a tuner host
    DeleteTunerHost {
        /// The tuner host ID
        #[arg(long)]
        id: Option<String>,
    },

    /// Get tuner host types
    TunerHostTypes,

    /// Reset a tuner
    ResetTuner {
        /// The tuner ID
        tuner_id: String,
    },

    /// Discover tuners
    DiscoverTuners {
        /// Only discover new devices
        #[arg(long)]
        new_devices_only: Option<bool>,
    },

    /// Get channel mapping options
    ChannelMappingOptions {
        /// The provider ID
        #[arg(long)]
        provider_id: Option<String>,
    },

    /// Set channel mapping (reads SetChannelMappingDto JSON from stdin)
    SetChannelMapping,
}

pub async fn execute(
    client: &jellyfin_api::Client,
    user_id: &Uuid,
    command: &LiveTvCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        LiveTvCommand::Channels {
            add_current_program,
            enable_favorite_sorting,
            fields,
            is_disliked,
            is_favorite,
            is_kids,
            is_liked,
            is_movie,
            is_news,
            is_series,
            is_sports,
            limit,
            sort_by,
            sort_order,
            start_index,
            type_,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_live_tv_channels(&jellyfin_api::query::GetLiveTvChannels {
                    add_current_program: *add_current_program,
                    enable_favorite_sorting: *enable_favorite_sorting,
                    fields: fields.as_ref(),
                    is_disliked: *is_disliked,
                    is_favorite: *is_favorite,
                    is_kids: *is_kids,
                    is_liked: *is_liked,
                    is_movie: *is_movie,
                    is_news: *is_news,
                    is_series: *is_series,
                    is_sports: *is_sports,
                    limit: *limit,
                    sort_by: sort_by.as_ref(),
                    sort_order: *sort_order,
                    start_index: *start_index,
                    type_: *type_,
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        LiveTvCommand::Channel {
            channel_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_channel(channel_id, Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::GuideInfo => {
            let result = client.get_guide_info().await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::Info => {
            let result = client.get_live_tv_info().await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::Programs {
            channel_ids,
            fields,
            has_aired,
            is_airing,
            is_kids,
            is_movie,
            is_news,
            is_series,
            is_sports,
            limit,
            series_timer_id,
            sort_by,
            sort_order,
            start_index,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_live_tv_programs(&jellyfin_api::query::GetLiveTvPrograms {
                    channel_ids: channel_ids.as_ref(),
                    fields: fields.as_ref(),
                    has_aired: *has_aired,
                    is_airing: *is_airing,
                    is_kids: *is_kids,
                    is_movie: *is_movie,
                    is_news: *is_news,
                    is_series: *is_series,
                    is_sports: *is_sports,
                    limit: *limit,
                    series_timer_id: series_timer_id.as_deref(),
                    sort_by: sort_by.as_ref(),
                    sort_order: sort_order.as_ref(),
                    start_index: *start_index,
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        LiveTvCommand::Program {
            program_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_program(program_id, Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::RecommendedPrograms {
            fields,
            has_aired,
            is_airing,
            is_kids,
            is_movie,
            is_news,
            is_series,
            is_sports,
            limit,
            start_index,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_recommended_programs(&jellyfin_api::query::GetRecommendedPrograms {
                    fields: fields.as_ref(),
                    has_aired: *has_aired,
                    is_airing: *is_airing,
                    is_kids: *is_kids,
                    is_movie: *is_movie,
                    is_news: *is_news,
                    is_series: *is_series,
                    is_sports: *is_sports,
                    limit: *limit,
                    start_index: *start_index,
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        LiveTvCommand::Recordings {
            channel_id,
            fields,
            is_in_progress,
            is_kids,
            is_library_item,
            is_movie,
            is_news,
            is_series,
            is_sports,
            limit,
            series_timer_id,
            start_index,
            status,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_recordings(&jellyfin_api::query::GetRecordings {
                    channel_id: channel_id.as_deref(),
                    fields: fields.as_ref(),
                    is_in_progress: *is_in_progress,
                    is_kids: *is_kids,
                    is_library_item: *is_library_item,
                    is_movie: *is_movie,
                    is_news: *is_news,
                    is_series: *is_series,
                    is_sports: *is_sports,
                    limit: *limit,
                    series_timer_id: series_timer_id.as_deref(),
                    start_index: *start_index,
                    status: *status,
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        LiveTvCommand::Recording {
            recording_id,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_recording(recording_id, Some(effective_uid))
                .await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::DeleteRecording { recording_id } => {
            client.delete_recording(recording_id).await?;
        }
        LiveTvCommand::RecordingFolders { user_id: uid } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_recording_folders(Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::RecordingGroups { user_id: uid } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client.get_recording_groups(Some(effective_uid)).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::RecordingGroup { group_id } => {
            let result = client.get_recording_group(group_id).await?;
            let body = result.text().await?;
            println!("{}", body);
        }
        LiveTvCommand::RecordingSeries {
            channel_id,
            fields,
            group_id,
            is_in_progress,
            limit,
            series_timer_id,
            start_index,
            status,
            user_id: uid,
        } => {
            let effective_uid = uid.as_ref().unwrap_or(user_id);
            let result = client
                .get_recordings_series(&jellyfin_api::query::GetRecordingsSeries {
                    channel_id: channel_id.as_deref(),
                    fields: fields.as_ref(),
                    group_id: group_id.as_deref(),
                    is_in_progress: *is_in_progress,
                    limit: *limit,
                    series_timer_id: series_timer_id.as_deref(),
                    start_index: *start_index,
                    status: *status,
                    user_id: Some(effective_uid),
                    ..Default::default()
                })
                .await?;
            crate::output::print_ndjson(&result.items)?;
        }
        LiveTvCommand::SeriesTimers {
            sort_by,
            sort_order,
        } => {
            let result = client
                .get_series_timers(sort_by.as_deref(), *sort_order)
                .await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::CreateSeriesTimer => {
            let body: jellyfin_api::types::SeriesTimerInfoDto =
                serde_json::from_reader(std::io::stdin())?;
            client.create_series_timer(&body).await?;
        }
        LiveTvCommand::SeriesTimer { timer_id } => {
            let result = client.get_series_timer(timer_id).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::UpdateSeriesTimer { timer_id } => {
            let body: jellyfin_api::types::SeriesTimerInfoDto =
                serde_json::from_reader(std::io::stdin())?;
            client.update_series_timer(timer_id, &body).await?;
        }
        LiveTvCommand::CancelSeriesTimer { timer_id } => {
            client.cancel_series_timer(timer_id).await?;
        }
        LiveTvCommand::Timers {
            channel_id,
            is_active,
            series_timer_id,
        } => {
            let result = client
                .get_timers(
                    channel_id.as_deref(),
                    *is_active,
                    None, // is_scheduled
                    series_timer_id.as_deref(),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::CreateTimer => {
            let body: jellyfin_api::types::TimerInfoDto =
                serde_json::from_reader(std::io::stdin())?;
            client.create_timer(&body).await?;
        }
        LiveTvCommand::Timer { timer_id } => {
            let result = client.get_timer(timer_id).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::UpdateTimer { timer_id } => {
            let body: jellyfin_api::types::TimerInfoDto =
                serde_json::from_reader(std::io::stdin())?;
            client.update_timer(timer_id, &body).await?;
        }
        LiveTvCommand::CancelTimer { timer_id } => {
            client.cancel_timer(timer_id).await?;
        }
        LiveTvCommand::DefaultTimer { program_id } => {
            let result = client.get_default_timer(program_id.as_deref()).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::AddListingProvider {
            pw,
            validate_listings,
            validate_login,
        } => {
            let body: jellyfin_api::types::ListingsProviderInfo =
                serde_json::from_reader(std::io::stdin())?;
            let result = client
                .add_listing_provider(pw.as_deref(), *validate_listings, *validate_login, &body)
                .await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::DeleteListingProvider { id } => {
            client.delete_listing_provider(id.as_deref()).await?;
        }
        LiveTvCommand::DefaultListingProvider => {
            let result = client.get_default_listing_provider().await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::Lineups {
            country,
            id,
            location,
            type_,
        } => {
            let result = client
                .get_lineups(
                    country.as_deref(),
                    id.as_deref(),
                    location.as_deref(),
                    type_.as_deref(),
                )
                .await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::SchedulesDirectCountries => {
            let result = client.get_schedules_direct_countries().await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::AddTunerHost => {
            let body: jellyfin_api::types::TunerHostInfo =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.add_tuner_host(&body).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::DeleteTunerHost { id } => {
            client.delete_tuner_host(id.as_deref()).await?;
        }
        LiveTvCommand::TunerHostTypes => {
            let result = client.get_tuner_host_types().await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::ResetTuner { tuner_id } => {
            client.reset_tuner(tuner_id).await?;
        }
        LiveTvCommand::DiscoverTuners { new_devices_only } => {
            let result = client.discover_tuners(*new_devices_only).await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::ChannelMappingOptions { provider_id } => {
            let result = client
                .get_channel_mapping_options(provider_id.as_deref())
                .await?;
            crate::output::print_json(&result)?;
        }
        LiveTvCommand::SetChannelMapping => {
            let body: jellyfin_api::types::SetChannelMappingDto =
                serde_json::from_reader(std::io::stdin())?;
            let result = client.set_channel_mapping(&body).await?;
            crate::output::print_json(&result)?;
        }
    }
    Ok(())
}
