//! The devices the server has seen.

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

pub async fn load(
    api: std::rc::Rc<crate::api::Api>,
    own: String,
    viewport: Viewport,
) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            devices: api.devices().await.bubbled()?,
            window: window::Window::new(
                window::Id::Devices,
                space::table_row(viewport.layout()),
                viewport.canvas().height(),
            ),
            renaming: String::new(),
            own,
        })
    })
    .await
}

/// A table of each device's last activity, its name, its application and that
/// application's version, and the user it last carried, with the controls that
/// rename and delete it.
// reference: table-devices-columns
// reference: table-devices-actions
pub fn view<'a>(state: &'a State, read_only: bool) -> frame::Filling<'a> {
    let mut toolbar: Vec<Element<'a, Message>> = Vec::new();
    if !read_only {
        toolbar.push(
            text_input(strings::lookup(Text::DevicesRename), &state.renaming)
                .style(style::input)
                .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed)))
                .into(),
        );
    }

    frame::Filling::Tabled {
        subtitle: None,
        table: Table {
            toolbar,
            columns: vec![
                Column {
                    label: Some(Text::ColumnLastActive),
                    width: space::DEVICES_LAST_ACTIVE,
                    holding: Holding::Written,
                },
                Column {
                    label: Some(Text::ColumnDevice),
                    width: space::DEVICES_DEVICE,
                    holding: Holding::Written,
                },
                Column {
                    label: Some(Text::ColumnApp),
                    width: space::DEVICES_APP,
                    holding: Holding::Written,
                },
                Column {
                    label: Some(Text::ColumnUser),
                    width: space::DEVICES_USER,
                    holding: Holding::Written,
                },
                Column {
                    label: None,
                    width: space::DEVICES_ACTIONS,
                    holding: Holding::Display,
                },
            ],
            window: state.window,
            rows: state.devices.len(),
            cells: Box::new(move |index| cells(state, read_only, index)),
        },
    }
}

/// One device's cells, in the order the reference's own columns stand.
// reference: table-devices-columns
// reference: table-devices-actions
fn cells<'a>(state: &'a State, read_only: bool, index: usize) -> Vec<Element<'a, Message>> {
    let Some(device) = state.devices.get(index) else {
        return Vec::new();
    };
    let name = device.name.clone().unwrap_or_default();
    vec![
        table::written(
            device
                .date_last_activity
                .map(table::stamped)
                .unwrap_or_default(),
        ),
        table::written(name.clone()),
        table::written(application(device)),
        table::written(device.last_user_name.clone().unwrap_or_default()),
        actions(state, read_only, device, name),
    ]
}

/// A device's application and the version of it, joined by one space, which is
/// how the reference writes that column.
// reference: table-devices-columns
fn application(device: &jellyfin_api::types::DeviceInfoDto) -> String {
    [device.app_name.as_deref(), device.app_version.as_deref()]
        .into_iter()
        .flatten()
        .collect::<Vec<&str>>()
        .join(" ")
}

/// The controls one device's own row carries: the rename, and the deletion
/// behind a confirmation naming it.
// reference: table-devices-actions
fn actions<'a>(
    state: &'a State,
    read_only: bool,
    device: &'a jellyfin_api::types::DeviceInfoDto,
    name: String,
) -> Element<'a, Message> {
    let Some(id) = device.id.clone() else {
        return iced::widget::Space::new().into();
    };
    if read_only {
        return iced::widget::Space::new().into();
    }
    iced::widget::row![
        button(icon::icon(Icon::Edit, typeface::ICON_BUTTON))
            .style(style::icon_control)
            .on_press(Message::DashboardAction(super::Action::Write(
                super::Written::SetDeviceName {
                    id: id.clone(),
                    name: state.renaming.clone(),
                },
            ))),
        button(icon::icon(Icon::Delete, typeface::ICON_BUTTON))
            .style(style::icon_control)
            .on_press(Message::DashboardAction(super::Action::Ask(
                crate::screen::confirm::Pending::of(
                    match id == state.own {
                        true =>
                            crate::screen::confirm::Destructive::DeleteOwnDevice { id: id.clone() },
                        false =>
                            crate::screen::confirm::Destructive::DeleteDevice { id: id.clone() },
                    },
                    name,
                ),
            ))),
    ]
    .into()
}
