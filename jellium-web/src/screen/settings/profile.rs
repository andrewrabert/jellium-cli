//! The profile screen: who this account is, and its image.

use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::Cache;
use crate::style::{card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};

use super::Action;

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
    /// What has been typed into the two password fields, which the reference
    /// draws under the name on this page.
    pub password: super::password::State,
    /// The tag the server reports for this account's own image, and none for
    /// an account that has none.
    pub image: Option<String>,
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
            image: read
                .get("PrimaryImageTag")
                .and_then(serde_json::Value::as_str)
                .map(str::to_owned),
            read,
            password: super::password::State::default(),
        })
    })
    .await
}

/// What removing this account's image asks confirmation for.
fn removing(state: &State) -> crate::screen::confirm::Pending {
    crate::screen::confirm::Pending::of(
        crate::screen::confirm::Destructive::RemoveUserImage { id: state.id },
        state.name.clone(),
    )
}

/// This account's own image where the cache holds it, and the placeholder the
/// reference stands in for an account with no image.
// reference: settings-profile-image
fn face(state: &State, images: &Cache) -> iced::widget::image::Handle {
    const AVATAR: &[u8] = include_bytes!("../../../branding/avatar.png");
    images
        .handle(image_key(state))
        .unwrap_or_else(|| iced::widget::image::Handle::from_bytes(AVATAR))
}

/// The name, whether the account is an administrator, when it was last active,
/// the image with the one control it offers and the display name field in the
/// screen's own section, then the password section; the writing controls are
/// absent under read-only.
// reference: settings-profile-form
pub fn sections<'a>(
    state: &'a State,
    read_only: bool,
    images: &'a Cache,
) -> Vec<Element<'a, Message>> {
    let mut rows = vec![
        prose(state.name.clone(), typeface::HEADING_3),
        widget::description(
            match state.administrator {
                true => Text::ProfileAdministrator,
                false => Text::ProfileMember,
            },
            space::DESCRIPTION_INSET,
        ),
    ];

    if let Some(at) = state.last_active {
        rows.push(prose(
            strings::format(Text::ProfileLastActive, &[&at.to_rfc3339()]),
            typeface::BODY,
        ));
    }

    rows.push(widget::labelled(
        strings::lookup(Text::ProfileImage),
        iced::widget::image(face(state, images)).into(),
    ));

    // reference: settings-profile-image-controls
    if !read_only {
        rows.push(match state.image {
            Some(_) => widget::control(
                strings::lookup(Text::UsersImageRemove),
                Some(Message::SettingsAction(Action::Ask(removing(state)))),
                widget::Emphasis::Raised,
            ),
            None => widget::control(
                strings::lookup(Text::ProfileImageChoose),
                Some(Message::SettingsAction(Action::ChooseImage)),
                widget::Emphasis::Submit,
            ),
        });
    }

    rows.push(widget::field(
        strings::lookup(Text::ProfileDisplayName),
        &state.naming,
        None,
        None,
        |typed| Message::SettingsAction(Action::Typed(typed)),
        match read_only {
            true => Message::Unchanged,
            false => Message::SettingsAction(Action::SaveName),
        },
        widget::Secrecy::Shown,
    ));

    if !read_only {
        rows.push(widget::control(
            strings::lookup(Text::ProfileSaveName),
            Some(Message::SettingsAction(Action::SaveName)),
            widget::Emphasis::Submit,
        ));
    }

    std::iter::once(widget::fields(
        typeface::Rank::Second,
        Text::SettingsProfile,
        rows,
    ))
    .chain(super::password::sections(&state.password, read_only))
    .collect()
}

fn image_key(state: &State) -> crate::images::Key {
    crate::images::Key {
        item: state.id,
        kind: crate::images::Kind::User,
        index: None,
        card: card::Card::USER,
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
