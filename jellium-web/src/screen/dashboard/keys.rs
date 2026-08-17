//! The API keys the server holds.

use iced::Element;
use iced::widget::{button, text_input};

use crate::app::Message;
use crate::error::Answer;
use crate::icon::{self, Icon};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::table::{self, Column, Holding, Table};
use crate::window;

use super::frame;

/// Every API key, and the application a new one is being named for.
#[derive(Debug, Clone)]
pub struct State {
    pub keys: Vec<jellyfin_api::types::AuthenticationInfo>,
    pub window: window::Window,
    pub naming: String,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>, viewport: Viewport) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            keys: api.keys().await.bubbled()?,
            window: window::Window::new(
                window::Id::Keys,
                space::table_row(viewport.band()),
                viewport.canvas().height(),
            ),
            naming: String::new(),
        })
    })
    .await
}

/// A table of each key's token, the application it was issued for and the date
/// it was issued, with the control that revokes it, under the line the
/// reference writes about what a key is for.
// reference: table-keys-columns
// reference: table-keys-actions
// reference: table-keys-subtitle
pub fn view<'a>(state: &'a State, read_only: bool) -> frame::Filling<'a> {
    let mut toolbar: Vec<Element<'a, Message>> = Vec::new();
    if !read_only {
        toolbar.push(
            text_input(strings::lookup(Text::KeysApp), &state.naming)
                .style(style::input)
                .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed)))
                .into(),
        );
        toolbar.push(
            button(
                iced::widget::row![
                    icon::icon(Icon::Add, typeface::BUTTON_ICON),
                    crate::widget::prose(strings::lookup(Text::KeysCreate), typeface::BODY),
                ]
                .spacing(style::drawn(space::CONTROL_GAP.drawn())),
            )
            .style(style::submit)
            .on_press(Message::DashboardAction(super::Action::Write(
                super::Written::CreateKey {
                    app: state.naming.clone(),
                },
            )))
            .into(),
        );
    }

    frame::Filling::Tabled {
        subtitle: Some(Text::KeysHelp),
        table: Table {
            toolbar,
            columns: vec![
                Column {
                    label: Some(Text::ColumnKey),
                    width: space::KEYS_TOKEN,
                    holding: Holding::Written,
                },
                Column {
                    label: Some(Text::ColumnApp),
                    width: space::KEYS_APP,
                    holding: Holding::Written,
                },
                Column {
                    label: Some(Text::ColumnIssued),
                    width: space::KEYS_ISSUED,
                    holding: Holding::Written,
                },
                Column {
                    label: None,
                    width: space::KEYS_ACTIONS,
                    holding: Holding::Display,
                },
            ],
            window: state.window,
            rows: state.keys.len(),
            cells: Box::new(move |index| cells(state, read_only, index)),
        },
    }
}

/// One key's cells, in the order the reference's own columns stand.
// reference: table-keys-columns
// reference: table-keys-actions
fn cells<'a>(state: &'a State, read_only: bool, index: usize) -> Vec<Element<'a, Message>> {
    let Some(key) = state.keys.get(index) else {
        return Vec::new();
    };
    let token = key.access_token.clone().unwrap_or_default();
    let app = key.app_name.clone().unwrap_or_default();
    vec![
        table::written(token.clone()),
        table::written(app.clone()),
        table::written(key.date_created.map(table::stamped).unwrap_or_default()),
        revoke(read_only, token, app),
    ]
}

/// The control one key's own row carries: its revocation, behind a confirmation
/// naming the application it was issued for.
// reference: table-keys-actions
fn revoke<'a>(read_only: bool, token: String, app: String) -> Element<'a, Message> {
    if read_only || token.is_empty() {
        return iced::widget::Space::new().into();
    }
    button(icon::icon(Icon::Delete, typeface::ICON_BUTTON))
        .style(style::icon_control)
        .on_press(Message::DashboardAction(super::Action::Ask(
            crate::screen::confirm::Pending::of(
                crate::screen::confirm::Destructive::RevokeKey { key: token },
                app,
            ),
        )))
        .into()
}
