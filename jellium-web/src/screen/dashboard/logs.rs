//! The server's log files, and the tail of the one a viewer shows.

use iced::widget::{button, column};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, Drawn, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{line, prose};
use crate::window;

/// The log files the server holds.
#[derive(Debug, Clone)]
pub struct State {
    pub files: Vec<jellyfin_api::types::LogFile>,
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
        })
    })
    .await
}

/// The last `route::TAIL_LIMIT` bytes of `name`, and the file's full length.
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
                Drawn::of(style::drawn(
                    typeface::BODY.times(typeface::LINE_HEIGHT).drawn(),
                )),
                height,
            ),
        })
    })
    .await
}

/// How many bytes read as a size on screen.
fn sized(bytes: u64) -> String {
    let mib = bytes as f64 / (1024.0 * 1024.0);
    format!("{mib:.1} MiB")
}

/// Each log file with its name and size.
pub fn view<'a>(state: &'a State) -> Element<'a, Message> {
    let mut page = column![prose(strings::lookup(Text::LogsTitle), typeface::HEADING_2)]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn()));

    for file in &state.files {
        let name = file.name.clone().unwrap_or_default();
        page = page.push(
            iced::widget::row![
                button(prose(name.clone(), typeface::BODY)).on_press(Message::DashboardAction(
                    super::Action::Open(super::Screen::Log { name })
                )),
                prose(
                    strings::format(Text::LogsSize, &[&sized(file.size.unwrap_or(0) as u64)]),
                    typeface::BODY
                ),
            ]
            .spacing(style::drawn(space::GUTTER.drawn())),
        );
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}

/// The most of a log file the local server delivers, which is what names the
/// body a tail.
const TAIL_LIMIT: u64 = 2 * 1024 * 1024;

/// The sentence naming the tail and the file's full size, and only the lines
/// the window shows.
pub fn viewer<'a>(held: &'a Viewer) -> Element<'a, Message> {
    let mut page = column![prose(held.name.clone(), typeface::HEADING_2)]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn()));

    if held.tail.truncated() {
        page = page.push(prose(
            strings::format(
                Text::LogsTail,
                &[&sized(TAIL_LIMIT), &sized(held.tail.size())],
            ),
            typeface::BODY,
        ));
    }

    page.push(window::list(held.window, held.tail.lines(), |index| {
        line(
            held.tail.line(index),
            typeface::BODY,
            typeface::Weight::Regular,
        )
    }))
    .width(Fill)
    .height(Fill)
    .into()
}
