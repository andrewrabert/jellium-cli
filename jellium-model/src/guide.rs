use std::collections::HashMap;

use chrono::{DateTime, NaiveDate, TimeDelta, Utc};
use jellium_protocol::TimerChanged;
use uuid::Uuid;

use crate::appearance::Drawn;
use crate::livetv::{Channel, Program};
use crate::window;

/// The height `count` rows of `row` occupy.
fn rows(count: usize, row: Drawn) -> Drawn {
    Drawn::of(count as f32 * row.count())
}

/// How much time the guide shows at once.
pub const SPAN: TimeDelta = TimeDelta::hours(2);

/// One time step, which is also how the time axis is ruled.
pub const STEP: TimeDelta = TimeDelta::minutes(30);

/// `at` rounded down to the half hour it falls in.
pub fn half_hour(at: DateTime<Utc>) -> DateTime<Utc> {
    let minutes = STEP.num_minutes();
    let since = at.timestamp().div_euclid(minutes * 60) * minutes * 60;
    DateTime::from_timestamp(since, 0).unwrap_or(at)
}

/// Why the guide stands with no grid, which the screen turns into its
/// sentence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trouble {
    /// The step or the date asked for falls outside the range `GuideInfo`
    /// reports.
    OutOfRange,
}

/// Which way a screen step moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Back,
    Forward,
}

/// Which way the focus moves.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Move {
    Earlier,
    Later,
    Up,
    Down,
}

/// How a programme's cell stands: the guide's focus is on it, it is airing at
/// the instant drawn, or neither.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Standing {
    Focused,
    Airing,
    Resting,
}

/// Where a programme's cell falls in the span shown: how far into it the cell
/// begins, and how long it runs there.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Placed {
    pub begins: TimeDelta,
    pub runs: TimeDelta,
}

/// Where the keyboard is, held as a channel and an instant rather than as a
/// widget.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Focus {
    pub channel: usize,
    pub at: DateTime<Utc>,
}

/// What one program fetch covered.
#[derive(Debug, Clone, PartialEq)]
pub struct Fetched {
    pub channels: std::ops::Range<usize>,
    pub span: std::ops::Range<DateTime<Utc>>,
}

impl Fetched {
    /// True when this band lies wholly inside `held`.
    fn inside(&self, held: &Fetched) -> bool {
        self.channels.start >= held.channels.start
            && self.channels.end <= held.channels.end
            && self.span.start >= held.span.start
            && self.span.end <= held.span.end
    }
}

/// The channels, the programs held for them, and where the guide is looking.
#[derive(Debug, Clone)]
pub struct State {
    /// Every TV channel, in channel-number order; radio channels are absent.
    pub channels: Vec<Channel>,
    /// The range `GuideInfo` reports.
    pub range: std::ops::Range<DateTime<Utc>>,
    /// The instant the leftmost column begins at, always on a half hour.
    pub start: DateTime<Utc>,
    pub window: window::Window,
    /// The programs held, by channel.
    pub programs: HashMap<Uuid, Vec<Program>>,
    /// What `programs` was fetched for.
    pub held: Option<Fetched>,
    pub focus: Focus,
    /// The cause shown in the grid's place.
    pub trouble: Option<Trouble>,
}

impl State {
    /// The channels and span the guide is about to show, widened by one
    /// screenful in each direction and clamped to `range`.
    pub fn wanted(&self) -> Fetched {
        let shown = self.window.shown(self.channels.len());
        let margin = shown.len().max(1);
        let channels = shown.start.saturating_sub(margin)
            ..shown.end.saturating_add(margin).min(self.channels.len());
        let span = (self.start - SPAN).max(self.range.start)
            ..(self.start + SPAN + SPAN).min(self.range.end);
        Fetched { channels, span }
    }

    /// True when `wanted` is not inside `held`, which is the only thing that
    /// issues a program query.
    pub fn stale(&self) -> bool {
        match &self.held {
            Some(held) => !self.wanted().inside(held),
            None => true,
        }
    }

    /// Records one fetch: its programs are held and `held` becomes what it
    /// covered.
    pub fn fetched(&mut self, fetched: Fetched, programs: Vec<Program>) {
        self.programs.clear();
        for program in programs {
            self.programs
                .entry(program.channel)
                .or_default()
                .push(program);
        }
        for held in self.programs.values_mut() {
            held.sort_by_key(|program| program.start);
        }
        self.held = Some(fetched);
    }

    /// The programs on the channel at `index` overlapping the shown span,
    /// earliest first.
    pub fn cells(&self, index: usize) -> Vec<&Program> {
        let Some(channel) = self.channels.get(index) else {
            return Vec::new();
        };
        let shown = self.start..self.start + SPAN;
        self.programs
            .get(&channel.id)
            .map(|programs| {
                programs
                    .iter()
                    .filter(|program| program.end > shown.start && program.start < shown.end)
                    .collect()
            })
            .unwrap_or_default()
    }

    /// How the cell of `program` on the channel at `index` stands at `now`.
    pub fn standing(&self, index: usize, program: &Program, now: DateTime<Utc>) -> Standing {
        let focused = index == self.focus.channel
            && program.start <= self.focus.at
            && self.focus.at < program.end;
        if focused {
            return Standing::Focused;
        }
        match program.airing(now) {
            true => Standing::Airing,
            false => Standing::Resting,
        }
    }

    // a cell beginning before the span shown begins at nought and runs only
    // as long as the part of it the span holds, as the reference clips a cell
    // to its day
    // reference: guide-cell-span
    pub fn placed(&self, program: &Program) -> Placed {
        let shown = self.start..self.start + SPAN;
        let begins = (program.start - shown.start).max(TimeDelta::zero());
        let runs = (program.end.min(shown.end) - (shown.start + begins)).max(TimeDelta::zero());
        Placed { begins, runs }
    }

    /// The program the focus rests on.
    pub fn focused(&self) -> Option<&Program> {
        let channel = self.channels.get(self.focus.channel)?;
        self.programs
            .get(&channel.id)?
            .iter()
            .find(|program| program.start <= self.focus.at && self.focus.at < program.end)
    }

    /// Moves the focus by one time step or one channel, carrying the shown
    /// span and the window with it when the focus leaves an edge.
    pub fn moved(&mut self, moved: Move) {
        match moved {
            Move::Earlier => {
                self.focus.at = (self.focus.at - STEP).max(self.range.start);
                if self.focus.at < self.start {
                    self.start = half_hour(self.focus.at);
                }
            }
            Move::Later => {
                self.focus.at = (self.focus.at + STEP).min(self.range.end - STEP);
                if self.focus.at >= self.start + SPAN {
                    self.start = half_hour(self.focus.at) - SPAN + STEP;
                }
            }
            Move::Up => {
                self.focus.channel = self.focus.channel.saturating_sub(1);
                let shown = self.window.shown(self.channels.len());
                if self.focus.channel < shown.start {
                    self.window.scrolled(window::Scrolled {
                        id: self.window.id(),
                        offset: rows(self.focus.channel, self.window.cell()),
                        extent: rows(shown.len(), self.window.cell()),
                    });
                }
            }
            Move::Down => {
                let last = self.channels.len().saturating_sub(1);
                self.focus.channel = (self.focus.channel + 1).min(last);
                let shown = self.window.shown(self.channels.len());
                if self.focus.channel >= shown.end {
                    let first = self
                        .focus
                        .channel
                        .saturating_sub(shown.len().saturating_sub(1));
                    self.window.scrolled(window::Scrolled {
                        id: self.window.id(),
                        offset: rows(first, self.window.cell()),
                        extent: rows(shown.len(), self.window.cell()),
                    });
                }
            }
        }
        self.trouble = None;
    }

    /// Moves the shown span one screenful.
    /// A step outside `range` leaves the span standing and sets `trouble` to
    /// the cause.
    pub fn stepped(&mut self, step: Step) {
        let moved = match step {
            Step::Back => self.start - SPAN,
            Step::Forward => self.start + SPAN,
        };
        if moved < self.range.start || moved + SPAN > self.range.end {
            self.trouble = Some(Trouble::OutOfRange);
            return;
        }
        self.start = moved;
        self.focus.at = self.start;
        self.trouble = None;
    }

    /// Moves the shown span to the first half hour of `date`.
    /// A date outside `range` leaves the span standing and sets `trouble`.
    pub fn dated(&mut self, date: NaiveDate) {
        let Some(moved) = date.and_hms_opt(0, 0, 0).map(|at| at.and_utc()) else {
            self.trouble = Some(Trouble::OutOfRange);
            return;
        };
        if moved < self.range.start || moved >= self.range.end {
            self.trouble = Some(Trouble::OutOfRange);
            return;
        }
        self.start = moved;
        self.focus.at = moved;
        self.trouble = None;
    }

    /// Applies one timer change to every program held, refetching nothing.
    pub fn timed(&mut self, changed: &TimerChanged) {
        for programs in self.programs.values_mut() {
            for program in programs {
                program.timed(changed);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jellium_protocol::TimerChange;

    const ROW_HEIGHT: Drawn = Drawn::of(64.0);

    fn channel(index: usize) -> Channel {
        Channel {
            id: Uuid::from_u128(index as u128 + 1),
            number: format!("{}", index + 1),
            name: format!("Channel {}", index + 1),
            kind: jellyfin_api::types::ChannelType::Tv,
            favorite: false,
            marque: crate::livetv::Marque::Name,
            current: None,
        }
    }

    fn opens() -> DateTime<Utc> {
        DateTime::from_timestamp(0, 0).expect("the epoch")
    }

    fn state(channels: usize) -> State {
        let start = opens() + TimeDelta::days(1);
        State {
            channels: (0..channels).map(channel).collect(),
            range: opens()..opens() + TimeDelta::days(14),
            start,
            window: window::Window::new(window::Id::Guide, ROW_HEIGHT, rows(10, ROW_HEIGHT)),
            programs: HashMap::new(),
            held: None,
            focus: Focus {
                channel: 0,
                at: start,
            },
            trouble: None,
        }
    }

    fn program(channel: Uuid, start: DateTime<Utc>) -> Program {
        Program {
            id: format!("{channel}-{}", start.timestamp()),
            item: Uuid::from_u128(start.timestamp() as u128 + 1_000),
            channel,
            channel_name: String::new(),
            channel_number: String::new(),
            title: "A programme".to_string(),
            episode_title: None,
            overview: String::new(),
            genres: Vec::new(),
            start,
            end: start + STEP,
            live: false,
            new: false,
            premiere: false,
            repeat: false,
            timer: None,
            series_timer: None,
        }
    }

    #[test]
    fn a_guide_opens_at_the_current_half_hour_showing_two_hours() {
        let state = state(20);
        assert_eq!(state.start, half_hour(state.start));
        assert_eq!(SPAN, TimeDelta::hours(2));
        assert_eq!(STEP, TimeDelta::minutes(30));
        assert_eq!(state.focus.at, state.start);
    }

    #[test]
    fn a_wanted_band_carries_one_screenful_of_margin_on_each_axis() {
        let state = state(500);
        let wanted = state.wanted();
        let shown = state.window.shown(500);
        assert_eq!(wanted.channels.start, 0);
        assert_eq!(wanted.channels.end, shown.end + shown.len());
        assert_eq!(wanted.span.start, state.start - SPAN);
        assert_eq!(wanted.span.end, state.start + SPAN + SPAN);
    }

    #[test]
    fn a_wanted_band_inside_what_is_held_issues_no_request() {
        let mut state = state(500);
        assert!(state.stale());

        let wanted = state.wanted();
        state.fetched(wanted, Vec::new());
        assert!(!state.stale());

        state.moved(Move::Down);
        assert!(!state.stale());
    }

    #[test]
    fn a_screen_step_past_the_reported_range_leaves_the_span_and_names_the_cause() {
        let mut state = state(20);
        state.start = state.range.end - SPAN;
        let standing = state.start;

        state.stepped(Step::Forward);
        assert_eq!(state.start, standing);
        assert_eq!(state.trouble, Some(Trouble::OutOfRange));

        state.start = state.range.start;
        state.stepped(Step::Back);
        assert_eq!(state.start, state.range.start);
        assert_eq!(state.trouble, Some(Trouble::OutOfRange));
    }

    #[test]
    fn a_date_outside_the_reported_range_names_the_cause() {
        let mut state = state(20);
        let standing = state.start;

        state.dated((state.range.end + TimeDelta::days(1)).date_naive());
        assert_eq!(state.start, standing);
        assert_eq!(state.trouble, Some(Trouble::OutOfRange));

        state.dated((state.range.start + TimeDelta::days(2)).date_naive());
        assert_eq!(state.trouble, None);
        assert_ne!(state.start, standing);
    }

    #[test]
    fn a_focus_move_past_the_screen_edge_carries_the_span_with_it() {
        let mut state = state(20);
        let opened = state.start;

        for _ in 0..(SPAN.num_minutes() / STEP.num_minutes()) {
            state.moved(Move::Later);
        }
        assert!(state.start > opened);
        assert!(state.focus.at >= state.start);
        assert!(state.focus.at < state.start + SPAN);

        state.moved(Move::Earlier);
        assert!(state.focus.at >= state.start);
    }

    #[test]
    fn a_timer_event_marks_the_cell_it_names_without_a_refetch() {
        let mut state = state(20);
        let channel = state.channels[0].id;
        let program = program(channel, state.start);
        let item = program.item;
        let wanted = state.wanted();
        state.fetched(wanted.clone(), vec![program]);

        state.timed(&TimerChanged {
            change: TimerChange::Created,
            timer: "timer-1".to_string(),
            program: Some(item),
        });

        assert_eq!(
            state.programs[&channel][0].timer.as_deref(),
            Some("timer-1")
        );
        assert_eq!(state.held, Some(wanted));
        assert!(!state.stale());
    }
}
