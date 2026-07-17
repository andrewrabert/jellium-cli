use clap::Subcommand;

#[derive(Clone, Subcommand)]
pub enum TasksCommand {
    /// List scheduled tasks
    List {
        /// Filter by enabled state
        #[arg(long)]
        is_enabled: Option<bool>,
        /// Filter by hidden state
        #[arg(long)]
        is_hidden: Option<bool>,
    },
    /// Get a task by ID
    Get {
        /// Task ID
        id: String,
    },
    /// Start a task
    Start {
        /// Task ID
        id: String,
    },
    /// Stop a task
    Stop {
        /// Task ID
        id: String,
    },
    /// Update task triggers
    UpdateTriggers {
        /// Task ID
        id: String,
        /// Trigger type (DailyTrigger, WeeklyTrigger, IntervalTrigger, StartupTrigger)
        #[arg(long)]
        trigger_type: Option<jellyfin_api::types::TaskTriggerInfoType>,
        /// Time of day ticks
        #[arg(long)]
        time_of_day_ticks: Option<i64>,
        /// Interval ticks
        #[arg(long)]
        interval_ticks: Option<i64>,
        /// Day of week
        #[arg(long)]
        day_of_week: Option<jellyfin_api::types::DayOfWeek>,
        /// Maximum runtime ticks
        #[arg(long)]
        max_runtime_ticks: Option<i64>,
    },
}

pub async fn execute(
    client: &jellyfin_api::Client,
    command: TasksCommand,
) -> Result<(), Box<dyn std::error::Error>> {
    match command {
        TasksCommand::List {
            is_enabled,
            is_hidden,
        } => {
            let result = client.get_tasks(is_enabled, is_hidden).await?;
            crate::output::print_json(&result)?;
        }
        TasksCommand::Get { id } => {
            let result = client.get_task(&id).await?;
            crate::output::print_json(&result)?;
        }
        TasksCommand::Start { id } => {
            client.start_task(&id).await?;
        }
        TasksCommand::Stop { id } => {
            client.stop_task(&id).await?;
        }
        TasksCommand::UpdateTriggers {
            id,
            trigger_type,
            time_of_day_ticks,
            interval_ticks,
            day_of_week,
            max_runtime_ticks,
        } => {
            let trigger = jellyfin_api::types::TaskTriggerInfo {
                type_: trigger_type,
                time_of_day_ticks,
                interval_ticks,
                day_of_week,
                max_runtime_ticks,
            };
            let body = vec![trigger];
            client.update_task(&id, &body).await?;
        }
    }
    Ok(())
}
