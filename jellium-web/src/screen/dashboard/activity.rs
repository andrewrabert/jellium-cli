//! The activity log, fetched a page at a time as its window moves.

use iced::widget::{button, column, row};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, Drawn, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{line, prose};
use crate::window;
use jellium_model::paged::Paged;
use jellium_protocol::ActivityEntry;

/// The activity log, fetched a page at a time as its window moves.
#[derive(Debug, Clone)]
pub struct State {
    pub entries: Paged<ActivityEntry>,
    pub window: window::Window,
    /// True while only entries naming a user are shown, false while only those
    /// naming none are, and nothing while all are.
    pub with_user: Option<bool>,
}

pub async fn load(
    api: std::rc::Rc<crate::api::Api>,
    with_user: Option<bool>,
    height: Drawn,
) -> Answer<State> {
    Answer::of(async {
        let (rows, total) = api
            .activity(0, Paged::<ActivityEntry>::PAGE as i32, with_user)
            .await
            .bubbled()?;
        let mut entries = Paged::new(total);
        entries.filled(0..rows.len(), rows);
        Ok(State {
            entries,
            window: window::Window::new(
                window::Id::Activity,
                Drawn::of(style::drawn(space::LIST_ROW.drawn())),
                height,
            ),
            with_user,
        })
    })
    .await
}

/// The page `state`'s window needs and no page already held or in flight, and
/// nothing while everything shown is held.
pub fn wanted(state: &mut State) -> Option<std::ops::Range<usize>> {
    let built = state.window.built(state.entries.len());
    let page = state.entries.wanted(built)?;
    state.entries.began(page.clone());
    Some(page)
}

/// Puts `entries` at the front without moving the scroll position, which is
/// what a coalesced push does.
pub fn prepended(state: &mut State, entries: Vec<ActivityEntry>) {
    if entries.is_empty() {
        return;
    }
    state.entries.prepend(entries);
}

/// Takes the rows a page answered with.
pub fn filled(state: &mut State, page: std::ops::Range<usize>, rows: Vec<ActivityEntry>) {
    state.entries.filled(page, rows);
}

/// Each entry's time, name, short overview, type and user, and the filter.
pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    let mut filters = row![].spacing(style::drawn(space::GUTTER.drawn()));
    for (label, wanted) in [
        (Text::ActivityAll, None),
        (Text::ActivityWithUser, Some(true)),
        (Text::ActivityWithoutUser, Some(false)),
    ] {
        let control = button(prose(strings::lookup(label), typeface::BODY)).style(style::flat);
        filters = filters.push(if state.with_user == wanted {
            control
        } else {
            control.on_press(Message::DashboardAction(super::Action::Filtered(wanted)))
        });
    }

    column![
        prose(strings::lookup(Text::ActivityTitle), typeface::HEADING_2),
        filters,
        window::list(state.window, state.entries.len(), |index| {
            match state.entries.row(index) {
                Some(entry) => column![
                    line(
                        format!("{} · {}", stamped(entry.at), entry.name),
                        typeface::BODY,
                        typeface::Weight::Regular,
                    ),
                    line(
                        format!("{} · {} · {}", entry.overview, entry.kind, entry.user_name),
                        typeface::BODY,
                        typeface::Weight::Regular,
                    ),
                ]
                .into(),
                None => prose("", typeface::BODY),
            }
        }),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .padding(style::drawn(space::GUTTER.drawn()))
    .width(Fill)
    .height(Fill)
    .into()
}

/// One entry's time, as the local clock reads it.
fn stamped(at: i64) -> String {
    chrono::DateTime::from_timestamp_millis(at)
        .map(|at| at.format("%Y-%m-%d %H:%M").to_string())
        .unwrap_or_default()
}
