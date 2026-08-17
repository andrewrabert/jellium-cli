use chrono::{DateTime, Utc};
use jellium_protocol::{TimerChange, TimerChanged};
use jellyfin_api::types::{BaseItemDto, BaseItemKind, ChannelType};
use uuid::Uuid;

use crate::appearance::Share;

/// One scheduled airing, as a guide cell, a channel row and program detail
/// draw it.
#[derive(Debug, Clone, PartialEq)]
pub struct Program {
    pub id: String,
    pub item: Uuid,
    pub channel: Uuid,
    pub channel_name: String,
    pub channel_number: String,
    pub title: String,
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
            id: id.to_string(),
            item: id,
            channel,
            channel_name: item.channel_name.clone().unwrap_or_default(),
            channel_number: item.channel_number.clone().unwrap_or_default(),
            title: item.name.clone().unwrap_or_default(),
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
    pub favorite: bool,
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
            favorite: item
                .user_data
                .as_ref()
                .and_then(|data| data.is_favorite)
                .unwrap_or(false),
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
