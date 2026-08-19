//! The activity log, fetched a page at a time as its window moves.

use std::collections::BTreeMap;

use iced::Element;
use iced::widget::button;

use crate::app::Message;
use crate::error::Answer;
use crate::icon::{self, Icon};
use crate::route::Route;
use crate::style::{self, Layout, Viewport, space, typeface};
use crate::text::Text;
use crate::widget::table::{self, Column, Holding, Table};
use crate::widget::{Segment, Showing};
use crate::window;
use jellium_model::paged::Paged;
use jellium_protocol::ActivityEntry;

use super::frame;

/// The activity log, fetched a page at a time as its window moves.
#[derive(Debug, Clone)]
pub struct State {
    pub entries: Paged<ActivityEntry>,
    pub window: window::Window,
    /// The name each user the log names is drawn under, which is what the
    /// reference's own user column reads from.
    pub users: BTreeMap<uuid::Uuid, String>,
    /// True while only entries naming a user are shown, false while only those
    /// naming none are, and nothing while all are.
    pub with_user: Option<bool>,
}

pub async fn load(
    api: std::rc::Rc<crate::api::Api>,
    with_user: Option<bool>,
    viewport: Viewport,
) -> Answer<State> {
    Answer::of(async {
        let (rows, total) = api
            .activity(0, Paged::<ActivityEntry>::PAGE as i32, with_user)
            .await
            .bubbled()?;
        let users = api
            .users()
            .await
            .bubbled()?
            .into_iter()
            .filter_map(|user| Some((user.id?, user.name?)))
            .collect();
        let mut entries = Paged::new(total);
        entries.filled(0..rows.len(), rows);
        Ok(State {
            entries,
            window: window::Window::new(
                window::Id::Activity,
                space::table_row(viewport.layout()),
                viewport.canvas().height(),
            ),
            users,
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

/// Whether the log is showing entries naming a user, which is what stands the
/// reference's own user column.
fn naming_users(with_user: Option<bool>) -> bool {
    with_user != Some(false)
}

/// The toolbar's three-segment group over a table of the entry's time, its
/// level, the user it names, its name, its overview, its type, and the control
/// that opens the item it names.
// reference: table-activity-columns
// reference: table-activity-view
pub fn view<'a>(state: &'a State, layout: Layout) -> frame::Filling<'a> {
    let segments = [
        (Text::ActivityAll, None),
        (Text::ActivityWithUser, Some(true)),
        (Text::ActivityWithoutUser, Some(false)),
    ]
    .map(|(label, wanted)| Segment {
        label,
        showing: match state.with_user == wanted {
            true => Showing::Shown,
            false => Showing::Offered(Message::DashboardAction(super::Action::Filtered(wanted))),
        },
    });

    let mut columns = vec![
        Column {
            label: Some(Text::ColumnTime),
            width: space::ACTIVITY_TIME,
            holding: Holding::Written,
        },
        Column {
            label: Some(Text::ColumnLevel),
            width: space::ACTIVITY_LEVEL,
            holding: Holding::Written,
        },
    ];
    if naming_users(state.with_user) {
        columns.push(Column {
            label: Some(Text::ColumnUser),
            width: space::ACTIVITY_USER,
            holding: Holding::Written,
        });
    }
    columns.extend([
        Column {
            label: Some(Text::ColumnName),
            width: space::ACTIVITY_NAME,
            holding: Holding::Written,
        },
        Column {
            label: Some(Text::ColumnOverview),
            width: space::ACTIVITY_OVERVIEW,
            holding: Holding::Written,
        },
        Column {
            label: Some(Text::ColumnType),
            width: space::ACTIVITY_TYPE,
            holding: Holding::Written,
        },
        Column {
            label: None,
            width: space::ACTIVITY_ACTIONS,
            holding: Holding::Display,
        },
    ]);

    frame::Filling::Tabled {
        subtitle: None,
        table: Table {
            toolbar: vec![crate::widget::toggles(segments, layout)],
            columns,
            window: state.window,
            rows: state.entries.len(),
            cells: Box::new(move |index| cells(state, index)),
        },
    }
}

/// One entry's cells, in the order the reference's own columns stand.
// reference: table-activity-columns
fn cells<'a>(state: &'a State, index: usize) -> Vec<Element<'a, Message>> {
    let Some(entry) = state.entries.row(index) else {
        return Vec::new();
    };
    let mut written = vec![
        table::written(stamped(entry.at)),
        table::written(entry.severity.clone()),
    ];
    if naming_users(state.with_user) {
        written.push(table::written(
            entry
                .user
                .and_then(|user| state.users.get(&user))
                .cloned()
                .unwrap_or_default(),
        ));
    }
    written.extend([
        table::written(entry.name.clone()),
        table::written(entry.overview.clone()),
        table::written(entry.kind.clone()),
        match entry.item {
            Some(item) => button(icon::icon(Icon::PermMedia, typeface::ICON_BUTTON))
                .style(style::icon_control)
                .on_press(Message::Navigated(Route::Detail { id: item }))
                .into(),
            None => iced::widget::Space::new().into(),
        },
    ]);
    written
}

/// One entry's time, which the wire carries as milliseconds since the epoch.
fn stamped(at: i64) -> String {
    chrono::DateTime::from_timestamp_millis(at)
        .map(table::stamped)
        .unwrap_or_default()
}
