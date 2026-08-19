//! The scheduled tasks the server holds, as every screen takes them.

/// One scheduled task as the dashboard takes it, and nothing where the server
/// named no id for it.
pub fn taken(info: jellyfin_api::types::TaskInfo) -> Option<jellium_protocol::TaskState> {
    use jellyfin_api::types::TaskState as Upstream;
    Some(jellium_protocol::TaskState {
        id: info.id?,
        name: info.name.unwrap_or_default(),
        category: info.category.unwrap_or_default(),
        description: info.description.unwrap_or_default(),
        state: match info.state {
            Some(Upstream::Cancelling) => jellium_protocol::TaskRunState::Cancelling,
            Some(Upstream::Running) => jellium_protocol::TaskRunState::Running,
            Some(Upstream::Idle) | None => jellium_protocol::TaskRunState::Idle,
        },
        progress: info.current_progress_percentage,
        last_ran: info.last_execution_result.and_then(ran),
    })
}

/// The run a task's last result names, and nothing where it names neither both
/// of its moments nor how it ended.
fn ran(result: jellyfin_api::types::TaskResult) -> Option<jellium_protocol::TaskRun> {
    use jellyfin_api::types::TaskCompletionStatus as Upstream;
    Some(jellium_protocol::TaskRun {
        started: result.start_time_utc?,
        ended: result.end_time_utc?,
        ending: match result.status? {
            Upstream::Completed => jellium_protocol::TaskEnding::Completed,
            Upstream::Failed => jellium_protocol::TaskEnding::Failed,
            Upstream::Cancelled => jellium_protocol::TaskEnding::Cancelled,
            Upstream::Aborted => jellium_protocol::TaskEnding::Aborted,
        },
    })
}
