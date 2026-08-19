//! The server's log files, and the tail of the one a viewer shows.

use iced::Element;
use iced::widget::column;
use jellium_model::form::{Field, Form};

use super::frame;
use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, Drawn, Layout, Viewport, space, typeface};
use crate::text::{self as strings, Template, Text};
use crate::widget::{self, line};
use crate::window;

/// The switch the logging form offers.
pub const SLOW_RESPONSE: Field = Field::Flag {
    key: "EnableSlowResponseWarning",
};

/// The threshold past which the server calls a response slow.
pub const SLOW_RESPONSE_THRESHOLD: Field = Field::Number {
    key: "SlowResponseThresholdMs",
};

/// The log files the server holds, and the server configuration the logging
/// form edits.
#[derive(Debug, Clone)]
pub struct State {
    pub files: Vec<jellyfin_api::types::LogFile>,
    pub form: Form,
    /// True once a save has landed and nothing has been edited since.
    pub saved: bool,
}

/// One log file's tail, windowed by line.
#[derive(Debug, Clone)]
pub struct Viewer {
    pub name: String,
    pub tail: jellium_model::log::Tail,
    pub window: window::Window,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            files: api.log_files().await.bubbled()?,
            form: Form::of(api.server_configuration().await.bubbled()?),
            saved: false,
        })
    })
    .await
}

/// The last `jellium_model::log::TAIL_LIMIT` bytes of `name`, and the file's
/// full length.
/// A file the server does not hold reads as `Trouble::LogMissing`.
pub async fn open(
    api: std::rc::Rc<crate::api::Api>,
    name: String,
    height: Drawn,
) -> Answer<Viewer> {
    Answer::of(async {
        let tail = api.log_tail(&name).await.bubbled()?;
        Ok(Viewer {
            name,
            tail,
            window: window::Window::new(
                window::Id::Log,
                typeface::LINE_HEIGHT.of(typeface::BODY).drawn(),
                height,
            ),
        })
    })
    .await
}

/// How many bytes read as a size on screen.
fn sized(bytes: jellium_model::log::Bytes) -> String {
    strings::format(
        Template::LogsMebibytes,
        &[&format!("{:.1}", bytes.mebibytes())],
    )
}

/// When a file was last written, as the date and the time the reference writes
/// beside its name, in the zone the browser stands in.
// reference: date-locale-date
// reference: date-display-time
fn modified(at: chrono::DateTime<chrono::Utc>) -> String {
    at.with_timezone(&chrono::Local)
        .format("%-m/%-d/%Y %-I:%M %p")
        .to_string()
}

/// What editing `field` to `value` sends.
fn edited(field: Field, value: String) -> Message {
    Message::DashboardAction(super::Action::Edited(field, value))
}

/// The logging form over the files, each file one row reaching its own viewer
/// under its name at MUI's `h3` over the date and time it was last modified.
// the threshold field is offered whether or not the switch is on, where the
// reference disables it while the switch is off
// reference: dashboard-content
// reference: logs-body
// reference: logs-list
// reference: date-locale-date
// reference: date-display-time
pub fn view<'a>(state: &'a State, read_only: bool, viewport: Viewport) -> frame::Filling<'a> {
    let layout = viewport.layout();
    let mut rows: Vec<Element<'a, Message>> = vec![
        widget::modern::flag(
            Text::LogsSlowResponse,
            None,
            state.form.flagged(SLOW_RESPONSE),
            move |on| edited(SLOW_RESPONSE, on.to_string()),
            layout,
        ),
        widget::modern::field(
            Text::LogsSlowResponseTime,
            None,
            None,
            &state.form.value(SLOW_RESPONSE_THRESHOLD),
            move |typed| edited(SLOW_RESPONSE_THRESHOLD, typed),
            layout,
        ),
    ];

    if state.saved {
        rows.push(widget::modern::succeeded(Text::DashboardSaved, layout));
    }

    if !read_only {
        let press = state
            .form
            .dirty()
            .then_some(Message::DashboardAction(super::Action::Save));
        rows.push(widget::modern::contained(
            Text::DashboardSave,
            press,
            layout,
        ));
    }

    rows.push(widget::modern::listed(
        state.files.iter().map(|file| {
            let name = file.name.clone().unwrap_or_default();
            widget::modern::Row {
                lead: None,
                primary: widget::modern::Primary::Headed(
                    typeface::Rank::Third,
                    name.clone().into(),
                ),
                beneath: file
                    .date_modified
                    .map(|at| widget::modern::Beneath::Said(modified(at).into())),
                within: None,
                showing: Some(widget::Showing::Offered(Message::DashboardAction(
                    super::Action::Open(super::Screen::Log { name }),
                ))),
                trailing: None,
            }
        }),
        layout,
    ));

    frame::Filling::Stacked { above: None, rows }
}

/// The file's own name as the page's heading, the sentence naming the tail and
/// the file's full size, and the lines the window shows, on the paper the
/// reference stands a log's body on.
// reference: logs-viewer
pub fn viewer<'a>(held: &'a Viewer, layout: Layout) -> frame::Filling<'a> {
    let mut page: Vec<Element<'a, Message>> = vec![widget::modern::heading(
        typeface::Rank::First,
        held.name.clone(),
    )];

    if held.tail.truncated() {
        page.push(widget::prose(
            strings::format(
                Template::LogsTail,
                &[
                    &sized(jellium_model::log::TAIL_LIMIT),
                    &sized(held.tail.size()),
                ],
            ),
            typeface::BODY,
        ));
    }

    page.push(widget::modern::papered(
        iced::widget::container(window::list(held.window, held.tail.lines(), |index| {
            line(
                held.tail.line(index),
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            )
        }))
        .padding(style::drawn(space::VIEWER_PAD.drawn(layout)))
        .into(),
        layout,
    ));
    frame::Filling::Whole(
        widget::scrolled(column(page).spacing(style::drawn(space::VIEWER_GAP.drawn(layout))))
            .into(),
    )
}
