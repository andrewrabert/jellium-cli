//! The devices the server has seen, and the API keys it holds.

use iced::widget::{button, column, row, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, Drawn, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{line, prose};
use crate::window;

/// Every device the server has seen, and what one is being renamed to.
#[derive(Debug, Clone)]
pub struct State {
    pub devices: Vec<jellyfin_api::types::DeviceInfoDto>,
    pub window: window::Window,
    pub renaming: String,
    /// This installation's own device id, which is what names the session that
    /// ends when it is deleted.
    pub own: String,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>, own: String, height: Drawn) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            devices: api.devices().await.bubbled()?,
            window: window::Window::new(
                window::Id::Devices,
                Drawn::of(style::drawn(space::LIST_ROW.drawn())),
                height,
            ),
            renaming: String::new(),
            own,
        })
    })
    .await
}

/// Each device's name, client, user and last-seen time, its rename, and its
/// deletion behind a confirmation naming it.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page = column![prose(
        strings::lookup(Text::DevicesTitle),
        typeface::HEADING_2
    )]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .padding(style::drawn(space::GUTTER.drawn()));

    if !read_only {
        page = page.push(
            text_input(strings::lookup(Text::DevicesRename), &state.renaming)
                .style(style::input)
                .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
        );
    }

    let listed = window::list(state.window, state.devices.len(), move |index| {
        let Some(device) = state.devices.get(index) else {
            return prose("", typeface::BODY);
        };
        let Some(id) = device.id.clone() else {
            return prose("", typeface::BODY);
        };
        let name = device.name.clone().unwrap_or_default();
        let mut held = row![
            line(name.clone(), typeface::BODY, typeface::Weight::Regular),
            line(
                device.app_name.clone().unwrap_or_default(),
                typeface::BODY,
                typeface::Weight::Regular,
            ),
            line(
                device.last_user_name.clone().unwrap_or_default(),
                typeface::BODY,
                typeface::Weight::Regular,
            ),
            line(
                device
                    .date_last_activity
                    .map(|at| at.format("%Y-%m-%d %H:%M").to_string())
                    .unwrap_or_default(),
                typeface::BODY,
                typeface::Weight::Regular,
            ),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()));

        if !read_only {
            held = held.push(
                button(prose(strings::lookup(Text::DevicesRename), typeface::BODY))
                    .style(style::submit)
                    .on_press(Message::DashboardAction(super::Action::Write(
                        super::Written::SetDeviceName {
                            id: id.clone(),
                            name: state.renaming.clone(),
                        },
                    ))),
            );
            held = held.push(
                button(prose(strings::lookup(Text::DevicesDelete), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::Ask(
                        crate::screen::confirm::Pending::of(
                            crate::screen::confirm::Destructive::DeleteDevice {
                                id: id.clone(),
                                own: id == state.own,
                            },
                            name,
                        ),
                    ))),
            );
        }
        held.into()
    });

    page.push(listed).width(Fill).height(Fill).into()
}
