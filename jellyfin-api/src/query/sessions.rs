/// What `/Sessions/{sessionId}/Playing` is narrowed by.
#[derive(Debug, Clone, Default)]
pub struct Play<'q> {
    /// Optional. The index of the audio stream to play.
    pub audio_stream_index: Option<i32>,
    /// Optional. The media source id.
    pub media_source_id: Option<&'q str>,
    /// Optional. The start index.
    pub start_index: Option<i32>,
    /// The starting position of the first item.
    pub start_position_ticks: Option<i64>,
    /// Optional. The index of the subtitle stream to play.
    pub subtitle_stream_index: Option<i32>,
}
