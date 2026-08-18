use chrono::{DateTime, Utc};
use jellium_protocol::{TimerChange, TimerChanged};
use jellyfin_api::types::{BaseItemDto, BaseItemKind, ChannelType, RecordingStatus, TimerInfoDto};
use uuid::Uuid;

use crate::appearance::{Layout, Share, card};
use crate::item::Mark;
use crate::paged::Limit;

/// What the guide's channel header writes for a channel: the primary image the
/// Jellyfin server holds for it, or the channel's own name where it holds none.
// reference: guide-channel-header-markup
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Marque {
    Logo,
    Name,
}

/// The one badge a programme's cell carries beside its name.
// reference: guide-program-indicators
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Badge {
    Live,
    Premiere,
    New,
}

/// The glyph the timer covering an item draws.
// reference: guide-timer-indicator
// reference: indicator-timer
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Recording {
    /// `fiber_manual_record`.
    Once,
    /// `fiber_smart_record`, in the timer's own colour.
    Series,
    /// `fiber_smart_record` drawn inactive.
    SeriesCancelled,
}

impl Recording {
    /// The series glyph, inactive where the server reports the timer cancelled.
    // reference: indicator-timer
    fn series(status: RecordingStatus) -> Recording {
        match status {
            RecordingStatus::Cancelled => Recording::SeriesCancelled,
            RecordingStatus::New
            | RecordingStatus::InProgress
            | RecordingStatus::Completed
            | RecordingStatus::ConflictedOk
            | RecordingStatus::ConflictedNotOk
            | RecordingStatus::Error => Recording::Series,
        }
    }

    // a series timer draws the series glyph, inactive where the item's own
    // recording status reads as cancelled and where it carries none
    // a timer standing alone draws the single glyph
    // an item naming neither timer carries no glyph
    // reference: indicator-timer
    pub fn covering(item: &BaseItemDto) -> Option<Recording> {
        if item.series_timer_id.is_some() {
            return Some(Recording::series(reported(item.status.as_deref())));
        }
        item.timer_id.as_ref().map(|_| Recording::Once)
    }

    // a timer belonging to a series timer draws the series glyph, inactive
    // where the server reports that timer cancelled
    // reference: indicator-timer
    pub fn scheduled(timer: &TimerInfoDto) -> Recording {
        match timer.series_timer_id {
            Some(_) => Recording::series(timer.status.unwrap_or(RecordingStatus::Cancelled)),
            None => Recording::Once,
        }
    }
}

/// The recording status the Jellyfin server names on an item, which the
/// reference reads as cancelled where the item names none. This is the one site
/// that reads the server's own status string.
// reference: indicator-timer
fn reported(status: Option<&str>) -> RecordingStatus {
    status
        .and_then(|named| named.parse().ok())
        .unwrap_or(RecordingStatus::Cancelled)
}

/// One scheduled airing, as a guide cell, a channel row and program detail
/// draw it.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub item: Uuid,
    pub channel: Uuid,
    pub channel_name: String,
    pub channel_number: String,
    pub title: String,
    /// The line the reference writes under a guide cell's name.
    pub episode_title: Option<String>,
    pub overview: String,
    pub genres: Vec<String>,
    pub start: DateTime<Utc>,
    pub end: DateTime<Utc>,
    pub live: bool,
    pub new: bool,
    pub premiere: bool,
    pub repeat: bool,
    /// The single timer covering it.
    pub timer: Option<String>,
    /// The series timer covering it.
    pub series_timer: Option<String>,
}

impl Program {
    /// The program `item` describes, or nothing when it names no channel, no
    /// start or no end.
    pub fn read(item: &BaseItemDto) -> Option<Program> {
        let id = item.id?;
        let channel = item.channel_id?;
        let start = item.start_date?;
        let end = item.end_date?;
        let repeat = item.is_repeat.unwrap_or(false);
        Some(Program {
            item: id,
            channel,
            channel_name: item.channel_name.clone().unwrap_or_default(),
            channel_number: item.channel_number.clone().unwrap_or_default(),
            title: item.name.clone().unwrap_or_default(),
            episode_title: item.episode_title.clone(),
            overview: item.overview.clone().unwrap_or_default(),
            genres: item.genres.clone().unwrap_or_default(),
            start,
            end,
            live: item.is_live.unwrap_or(false),
            new: item.is_series.unwrap_or(false) && !repeat,
            premiere: item.is_premiere.unwrap_or(false),
            repeat,
            timer: item.timer_id.clone(),
            series_timer: item.series_timer_id.clone(),
        })
    }

    // live wins over premiere, and premiere over a first showing
    // a repeat carries none, the reference's own default withholding it
    // reference: guide-program-indicators
    // reference: guide-indicator-options
    pub fn badge(&self) -> Option<Badge> {
        if self.live {
            return Some(Badge::Live);
        }
        if self.premiere {
            return Some(Badge::Premiere);
        }
        if self.new {
            return Some(Badge::New);
        }
        None
    }

    // a series timer wins over the single timer beside it, and draws inactive:
    // a guide programme carries no recording status, which the reference's own
    // status branch reads as cancelled
    // reference: guide-timer-indicator
    pub fn recording(&self) -> Option<Recording> {
        if self.series_timer.is_some() {
            return Some(Recording::SeriesCancelled);
        }
        self.timer.as_ref().map(|_| Recording::Once)
    }

    /// True while `now` falls inside it.
    pub fn airing(&self, now: DateTime<Utc>) -> bool {
        self.start <= now && now < self.end
    }

    /// How far through it `now` is.
    pub fn elapsed(&self, now: DateTime<Utc>) -> Share {
        Share::part(
            (now - self.start).num_seconds(),
            (self.end - self.start).num_seconds(),
        )
    }

    /// Applies one timer change: a created timer or series timer is recorded
    /// when it names this program, and a cancelled one cleared wherever its id
    /// stands.
    pub fn timed(&mut self, changed: &TimerChanged) {
        let names_this = changed.program == Some(self.item);
        match changed.change {
            TimerChange::Created if names_this => self.timer = Some(changed.timer.clone()),
            TimerChange::SeriesCreated if names_this => {
                self.series_timer = Some(changed.timer.clone());
            }
            TimerChange::Created | TimerChange::SeriesCreated => {}
            TimerChange::Cancelled => {
                if self.timer.as_deref() == Some(changed.timer.as_str()) {
                    self.timer = None;
                }
            }
            TimerChange::SeriesCancelled => {
                if self.series_timer.as_deref() == Some(changed.timer.as_str()) {
                    self.series_timer = None;
                }
            }
        }
    }
}

/// One channel: its number, name, logo, favourite mark and current program.
#[derive(Debug, Clone, PartialEq)]
pub struct Channel {
    pub id: Uuid,
    pub number: String,
    pub name: String,
    pub kind: ChannelType,
    pub favorite: Mark,
    /// `Logo` where the Jellyfin server reports a primary image tag.
    pub marque: Marque,
    pub current: Option<Program>,
}

impl Channel {
    /// The channel `item` describes, or nothing when the Jellyfin server did
    /// not hand it over as one.
    /// An item is a channel when its type is `TvChannel` or `LiveTvChannel`,
    /// or when it carries a channel type; its kind is that channel type, and
    /// `Tv` when it carries none.
    pub fn read(item: &BaseItemDto) -> Option<Channel> {
        let typed = matches!(
            item.type_,
            Some(BaseItemKind::TvChannel | BaseItemKind::LiveTvChannel)
        );
        if !typed && item.channel_type.is_none() {
            return None;
        }
        Some(Channel {
            id: item.id?,
            number: item.channel_number.clone().unwrap_or_default(),
            name: item.name.clone().unwrap_or_default(),
            kind: item.channel_type.unwrap_or(ChannelType::Tv),
            favorite: crate::item::favorited(item),
            marque: match item
                .image_tags
                .as_ref()
                .is_some_and(|tags| tags.contains_key("Primary"))
            {
                true => Marque::Logo,
                false => Marque::Name,
            },
            current: item
                .current_program
                .as_ref()
                .as_ref()
                .and_then(Program::read),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::TimeDelta;

    fn at(minutes: i64) -> DateTime<Utc> {
        DateTime::from_timestamp(0, 0).expect("the epoch") + TimeDelta::minutes(minutes)
    }

    fn program_item() -> BaseItemDto {
        BaseItemDto {
            id: Some(Uuid::from_u128(9)),
            channel_id: Some(Uuid::from_u128(3)),
            channel_name: Some("Channel 3".to_string()),
            channel_number: Some("3".to_string()),
            name: Some("A programme".to_string()),
            overview: Some("An overview.".to_string()),
            genres: Some(vec!["Drama".to_string()]),
            start_date: Some(at(0)),
            end_date: Some(at(30)),
            is_live: Some(true),
            is_series: Some(true),
            is_premiere: Some(true),
            is_repeat: Some(false),
            timer_id: Some("timer-1".to_string()),
            series_timer_id: Some("series-1".to_string()),
            ..BaseItemDto::default()
        }
    }

    fn channel_item() -> BaseItemDto {
        BaseItemDto {
            id: Some(Uuid::from_u128(3)),
            type_: Some(BaseItemKind::TvChannel),
            channel_number: Some("3".to_string()),
            name: Some("Channel 3".to_string()),
            channel_type: Some(ChannelType::Tv),
            current_program: Box::new(Some(program_item())),
            ..BaseItemDto::default()
        }
    }

    #[test]
    fn a_program_item_reads_its_flags_and_its_timer_ids() {
        let program = Program::read(&program_item()).expect("a programme");
        assert_eq!(program.item, Uuid::from_u128(9));
        assert_eq!(program.channel, Uuid::from_u128(3));
        assert_eq!(program.channel_number, "3");
        assert!(program.live);
        assert!(program.new);
        assert!(program.premiere);
        assert!(!program.repeat);
        assert_eq!(program.timer.as_deref(), Some("timer-1"));
        assert_eq!(program.series_timer.as_deref(), Some("series-1"));
        assert_eq!(program.genres, vec!["Drama".to_string()]);
    }

    #[test]
    fn an_item_naming_no_start_reads_as_no_program() {
        for missing in [
            BaseItemDto {
                start_date: None,
                ..program_item()
            },
            BaseItemDto {
                end_date: None,
                ..program_item()
            },
            BaseItemDto {
                channel_id: None,
                ..program_item()
            },
        ] {
            assert!(Program::read(&missing).is_none());
        }
    }

    #[test]
    fn a_created_timer_marks_the_program_it_names_and_no_other() {
        let mut named = Program {
            timer: None,
            ..Program::read(&program_item()).expect("a programme")
        };
        let mut other = Program {
            item: Uuid::from_u128(10),
            timer: None,
            ..named.clone()
        };
        let created = TimerChanged {
            change: TimerChange::Created,
            timer: "timer-9".to_string(),
            program: Some(Uuid::from_u128(9)),
        };

        named.timed(&created);
        other.timed(&created);
        assert_eq!(named.timer.as_deref(), Some("timer-9"));
        assert_eq!(other.timer, None);
    }

    #[test]
    fn a_cancelled_series_timer_clears_every_program_it_covered() {
        let base = Program::read(&program_item()).expect("a programme");
        let mut covered = vec![
            base.clone(),
            Program {
                item: Uuid::from_u128(10),
                ..base.clone()
            },
        ];
        let cancelled = TimerChanged {
            change: TimerChange::SeriesCancelled,
            timer: "series-1".to_string(),
            program: None,
        };

        for program in &mut covered {
            program.timed(&cancelled);
        }
        assert!(covered.iter().all(|program| program.series_timer.is_none()));
        assert!(covered.iter().all(|program| program.timer.is_some()));
    }

    #[test]
    fn an_elapsed_fraction_is_clamped_to_the_program() {
        let program = Program::read(&program_item()).expect("a programme");
        assert_eq!(program.elapsed(at(-10)), Share::per_ten_thousand(0));
        assert_eq!(program.elapsed(at(15)), Share::per_ten_thousand(5000));
        assert_eq!(program.elapsed(at(90)), Share::WHOLE);
        assert!(program.airing(at(15)));
        assert!(!program.airing(at(30)));
    }

    #[test]
    fn a_channel_item_reads_its_number_kind_and_current_program() {
        let channel = Channel::read(&channel_item()).expect("a channel");
        assert_eq!(channel.id, Uuid::from_u128(3));
        assert_eq!(channel.number, "3");
        assert_eq!(channel.kind, ChannelType::Tv);
        assert_eq!(
            channel.current.map(|program| program.item),
            Some(Uuid::from_u128(9))
        );

        let radio = Channel::read(&BaseItemDto {
            type_: None,
            channel_type: Some(ChannelType::Radio),
            ..channel_item()
        })
        .expect("a channel");
        assert_eq!(radio.kind, ChannelType::Radio);

        let untyped = Channel::read(&BaseItemDto {
            channel_type: None,
            ..channel_item()
        })
        .expect("a channel");
        assert_eq!(untyped.kind, ChannelType::Tv);
    }

    #[test]
    fn an_item_the_server_did_not_type_as_a_channel_reads_as_none() {
        assert!(Channel::read(&program_item()).is_none());
        assert!(
            Channel::read(&BaseItemDto {
                type_: Some(BaseItemKind::Movie),
                channel_type: None,
                ..channel_item()
            })
            .is_none()
        );
    }
}

/// One section of the reference's Programs tab, in the order it draws them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    OnNow,
    Shows,
    Movies,
    Sports,
    Kids,
    News,
}

/// Which programmes a section asks for.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Airing {
    /// `/LiveTv/Programs/Recommended` with `IsAiring`.
    Now,
    /// `/LiveTv/Programs` with `HasAired` false, narrowed by the flags the
    /// reference writes on that section.
    Upcoming(Upcoming),
}

/// The flags the reference narrows one upcoming section by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Upcoming {
    /// `IsSeries` true with movies, sports, kids and news excluded.
    Shows,
    Movies,
    Sports,
    Kids,
    News,
}

impl Section {
    pub const ALL: [Section; 6] = [
        Section::OnNow,
        Section::Shows,
        Section::Movies,
        Section::Sports,
        Section::Kids,
        Section::News,
    ];

    // reference: programs-query
    pub fn airing(self) -> Airing {
        match self {
            Section::OnNow => Airing::Now,
            Section::Shows => Airing::Upcoming(Upcoming::Shows),
            Section::Movies => Airing::Upcoming(Upcoming::Movies),
            Section::Sports => Airing::Upcoming(Upcoming::Sports),
            Section::Kids => Airing::Upcoming(Upcoming::Kids),
            Section::News => Airing::Upcoming(Upcoming::News),
        }
    }

    /// Movies takes the portrait rail and the other five the backdrop rail:
    /// `renderItems` gives every section the backdrop shape, and `reload`
    /// overrides the movies section alone with the portrait one.
    // reference: programs-shapes
    // reference: programs-query
    pub fn card(self) -> card::Card {
        match self {
            Section::Movies => card::Card::Rail(card::Rail::Portrait),
            Section::OnNow | Section::Shows | Section::Sports | Section::Kids | Section::News => {
                card::Card::Rail(card::Rail::Backdrop)
            }
        }
    }
}

/// How many programmes one section asks for: nine on the desktop band, twelve
/// where the reference scrolls sideways, and twice that for On Now there.
// reference: programs-query
pub fn asked(section: Section, layout: Layout) -> Limit {
    let scrolling = layout != Layout::Desktop;
    let limit = match scrolling {
        true => SCROLLING,
        false => STILL,
    };
    match scrolling && section == Section::OnNow {
        true => Limit::of(limit.count() * 2),
        false => limit,
    }
}

/// What one section asks for where the reference scrolls its rows sideways.
// reference: programs-query
const SCROLLING: Limit = Limit::of(12);

/// What one section asks for where it does not.
// reference: programs-query
const STILL: Limit = Limit::of(9);
