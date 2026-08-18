use std::time::Duration;

use chrono::{DateTime, TimeDelta, Utc};
use jellium_protocol::TimerChanged;

use crate::livetv::{Channel, Program};

/// A refetch already issued for one program boundary.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Asked {
    /// The end of the program the display stood at when it went out.
    pub boundary: DateTime<Utc>,
    /// When the last one for this boundary went out.
    pub at: DateTime<Utc>,
    /// How many have gone out for this boundary.
    pub tries: u32,
}

/// What the live display draws and what a channel change moves through.
#[derive(Debug, Clone, PartialEq)]
pub struct Live {
    pub channel: Channel,
    /// Every channel of the watched channel's kind, in channel-number order,
    /// which next and previous move through.
    pub channels: Vec<Channel>,
    /// The program the display names.
    pub program: Option<Program>,
    /// How long this playback has been paused.
    pub paused: Duration,
    /// True once this stream has been resumed at the live edge after a drop.
    pub resumed: bool,
    /// True from the moment a channel is selected until its first frame.
    pub tuning: bool,
    /// What the current program boundary has already cost.
    pub asked: Option<Asked>,
}

impl Live {
    /// A paused live playback is stopped once it passes this, which is the
    /// browser's half of ADR 0024.
    pub const PAUSED: Duration = Duration::from_secs(300);

    /// How long after a refetch that did not advance the display the next one
    /// for the same boundary goes out.
    pub const RETRY: TimeDelta = TimeDelta::seconds(30);

    /// How many refetches one program boundary costs at most.
    pub const TRIES: u32 = 2;

    /// Where the watched channel stands in the channel list.
    fn standing(&self) -> Option<usize> {
        self.channels
            .iter()
            .position(|channel| channel.id == self.channel.id)
    }

    /// The channel after the one being watched, wrapping at the end.
    pub fn next(&self) -> Option<&Channel> {
        let at = self.standing()?;
        self.channels.get((at + 1) % self.channels.len())
    }

    /// The channel before the one being watched, wrapping at the start.
    pub fn previous(&self) -> Option<&Channel> {
        let at = self.standing()?;
        let before = if at == 0 {
            self.channels.len().saturating_sub(1)
        } else {
            at - 1
        };
        self.channels.get(before)
    }

    /// True once the displayed program's end has passed.
    pub fn stale(&self, now: DateTime<Utc>) -> bool {
        match &self.program {
            Some(program) => now >= program.end,
            None => false,
        }
    }

    /// The boundary the display stands at.
    fn boundary(&self) -> Option<DateTime<Utc>> {
        self.program.as_ref().map(|program| program.end)
    }

    /// True when a refetch of the watched channel's current program is owed:
    /// the display has gone stale and this boundary has neither an
    /// outstanding refetch younger than `RETRY` nor spent its `TRIES`.
    pub fn due(&self, now: DateTime<Utc>) -> bool {
        if !self.stale(now) {
            return false;
        }
        let Some(asked) = self.asked else {
            return true;
        };
        if Some(asked.boundary) != self.boundary() {
            return true;
        }
        asked.tries < Self::TRIES && now - asked.at >= Self::RETRY
    }

    /// Records a refetch issued at `now` for the boundary the display stands
    /// at.
    pub fn asking(&mut self, now: DateTime<Utc>) {
        let Some(boundary) = self.boundary() else {
            return;
        };
        self.asked = Some(match self.asked {
            Some(asked) if asked.boundary == boundary => Asked {
                boundary,
                at: now,
                tries: asked.tries.saturating_add(1),
            },
            _ => Asked {
                boundary,
                at: now,
                tries: 1,
            },
        });
    }

    /// Takes what a refetch answered with; a program the display can stand on
    /// clears the boundary.
    pub fn advanced(&mut self, program: Option<Program>, now: DateTime<Utc>) {
        self.program = program;
        if !self.stale(now) {
            self.asked = None;
        }
    }

    /// Applies one timer change to the program being watched.
    pub fn timed(&mut self, changed: &TimerChanged) {
        if let Some(program) = &mut self.program {
            program.timed(changed);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    fn channel(index: u128) -> Channel {
        Channel {
            id: Uuid::from_u128(index),
            number: index.to_string(),
            name: format!("Channel {index}"),
            kind: jellyfin_api::types::ChannelType::Tv,
            favorite: crate::item::Mark::Cleared,
            marque: crate::livetv::Marque::Name,
            current: None,
        }
    }

    fn program(end: DateTime<Utc>) -> Program {
        Program {
            item: Uuid::from_u128(99),
            channel: Uuid::from_u128(1),
            channel_name: String::new(),
            channel_number: String::new(),
            title: String::new(),
            episode_title: None,
            overview: String::new(),
            genres: Vec::new(),
            start: end - TimeDelta::minutes(30),
            end,
            live: false,
            new: false,
            premiere: false,
            repeat: false,
            timer: None,
            series_timer: None,
        }
    }

    fn live(watched: u128) -> Live {
        Live {
            channel: channel(watched),
            channels: (1..=3).map(channel).collect(),
            program: None,
            paused: Duration::ZERO,
            resumed: false,
            tuning: false,
            asked: None,
        }
    }

    fn now() -> DateTime<Utc> {
        DateTime::from_timestamp(1_000_000, 0).expect("an instant")
    }

    #[test]
    fn a_channel_start_never_resumes() {
        let live = live(1);
        assert!(!live.resumed);
        assert!(live.program.is_none());
    }

    #[test]
    fn next_and_previous_wrap_the_channel_list() {
        assert_eq!(live(1).next().map(|c| c.id), Some(Uuid::from_u128(2)));
        assert_eq!(live(3).next().map(|c| c.id), Some(Uuid::from_u128(1)));
        assert_eq!(live(1).previous().map(|c| c.id), Some(Uuid::from_u128(3)));
        assert_eq!(live(2).previous().map(|c| c.id), Some(Uuid::from_u128(1)));
    }

    #[test]
    fn a_display_goes_stale_only_once_the_program_has_ended() {
        let now = now();
        let mut live = live(1);
        assert!(!live.stale(now));

        live.program = Some(program(now + TimeDelta::minutes(1)));
        assert!(!live.stale(now));

        live.program = Some(program(now));
        assert!(live.stale(now));
    }

    #[test]
    fn a_program_boundary_costs_one_refetch_and_one_bounded_retry() {
        let now = now();
        let mut live = live(1);
        live.program = Some(program(now));

        assert!(live.due(now));
        live.asking(now);
        assert!(!live.due(now));
        assert!(!live.due(now + Live::RETRY - TimeDelta::seconds(1)));

        assert!(live.due(now + Live::RETRY));
        live.asking(now + Live::RETRY);
        assert!(!live.due(now + Live::RETRY * 10));
        assert_eq!(live.asked.map(|asked| asked.tries), Some(Live::TRIES));
    }

    #[test]
    fn an_advanced_display_owes_no_further_refetch() {
        let now = now();
        let mut live = live(1);
        live.program = Some(program(now));
        live.asking(now);

        live.advanced(Some(program(now + TimeDelta::minutes(30))), now);
        assert_eq!(live.asked, None);
        assert!(!live.due(now + Live::RETRY * 10));
    }
}
