//! The profile screen: who this account is, and its image.

use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, text_input};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::Cache;
use crate::text::{self as strings, Text};

use super::Action;
use crate::style::{self, space, typeface};
use crate::widget::prose;

/// The signed-in user as the profile screen shows them.
#[derive(Debug, Clone)]
pub struct State {
    pub id: Uuid,
    /// The name the server reports, which is what the field opens holding.
    pub name: String,
    pub administrator: bool,
    pub last_active: Option<chrono::DateTime<chrono::Utc>>,
    /// The whole `UserDto`, so a name change preserves every field no control
    /// covers.
    pub read: serde_json::Value,
    /// What has been typed into the display name field.
    pub naming: String,
}

pub async fn load(api: Rc<Api>, id: Uuid) -> Answer<State> {
    Answer::of(async {
        let read = api.user_whole(id).await.bubbled()?;
        let name = read
            .get("Name")
            .and_then(serde_json::Value::as_str)
            .unwrap_or_default()
            .to_owned();
        let stamped = read
            .get("LastActivityDate")
            .and_then(serde_json::Value::as_str)
            .and_then(|at| {
                crate::failure::read::<chrono::DateTime<chrono::FixedOffset>>(
                    Text::FailureActivityDate,
                    at,
                )
            })
            .map(|at| at.with_timezone(&chrono::Utc));
        Ok(State {
            id,
            naming: name.clone(),
            name,
            administrator: read
                .get("Policy")
                .and_then(|policy| policy.get("IsAdministrator"))
                .and_then(serde_json::Value::as_bool)
                .unwrap_or(false),
            last_active: stamped,
            read,
        })
    })
    .await
}

/// The name, whether the account is an administrator, when it was last active,
/// the image with its Choose and Remove controls, and the refusal a chosen file
/// earned; the three writing controls are absent under read-only.
pub fn view<'a>(state: &'a State, read_only: bool, images: &'a Cache) -> Element<'a, Message> {
    let mut shown = column![
        prose(state.name.clone(), typeface::HEADING_3),
        prose(
            strings::lookup(if state.administrator {
                Text::ProfileAdministrator
            } else {
                Text::ProfileMember
            })
            .to_owned(),
            typeface::BODY
        ),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    if let Some(at) = state.last_active {
        shown = shown.push(prose(
            strings::format(Text::ProfileLastActive, &[&at.to_rfc3339()]),
            typeface::BODY,
        ));
    }

    shown = shown.push(prose(strings::lookup(Text::ProfileImage), typeface::BODY));
    if let Some(handle) = images.handle(image_key(state)) {
        shown = shown.push(iced::widget::image(handle));
    }

    if !read_only {
        shown = shown
            .push(
                button(prose(
                    strings::lookup(Text::ProfileImageChoose),
                    typeface::BODY,
                ))
                .style(style::raised)
                .on_press(Message::SettingsAction(Action::ChooseImage)),
            )
            .push(
                button(prose(
                    strings::lookup(Text::UsersImageRemove),
                    typeface::BODY,
                ))
                .style(style::raised)
                .on_press(Message::SettingsAction(Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::RemoveUserImage { id: state.id },
                        state.name.clone(),
                    ),
                ))),
            );
    }

    shown = shown
        .push(prose(
            strings::lookup(Text::ProfileDisplayName),
            typeface::BODY,
        ))
        .push(
            text_input("", &state.naming)
                .style(style::input)
                .on_input(|typed| Message::SettingsAction(Action::Typed(typed))),
        );

    if !read_only {
        shown = shown.push(
            button(prose(
                strings::lookup(Text::ProfileSaveName),
                typeface::BODY,
            ))
            .style(style::submit)
            .on_press(Message::SettingsAction(Action::SaveName)),
        );
    }

    shown.into()
}

fn image_key(state: &State) -> crate::images::Key {
    crate::images::Key {
        item: state.id,
        kind: crate::images::Kind::User,
        index: None,
    }
}

/// The user's own image, which is the one image this screen draws.
pub fn images(state: &State) -> HashSet<crate::images::Key> {
    HashSet::from([image_key(state)])
}

/// The `UserDto` a name change writes: what was read with the name replaced.
pub fn renamed(state: &State) -> serde_json::Value {
    let mut written = state.read.clone();
    if let Some(object) = written.as_object_mut() {
        object.insert(
            "Name".to_owned(),
            serde_json::Value::String(state.naming.clone()),
        );
    }
    written
}
