//! The home screen layout: which libraries the home screen shows, in what
//! order, and which of the two rows stand.

use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;

use super::{Action, Setting};

/// The libraries the server offers, so the order and the exclusions are edited
/// against names rather than ids.
#[derive(Debug, Clone)]
pub struct State {
    pub libraries: Vec<jellyfin_api::types::BaseItemDto>,
}

pub async fn load(api: Rc<Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            libraries: api.libraries().await.bubbled()?,
        })
    })
    .await
}

fn named(state: &State, id: Uuid) -> String {
    state
        .libraries
        .iter()
        .find(|library| library.id == Some(id))
        .and_then(|library| library.name.clone())
        .unwrap_or_default()
}

/// The two row flags in the screen's own section; the libraries in the
/// configuration's order as a ruled list carrying the reference's two move
/// controls, over the checkbox list saying which of them the home screen shows.
// reference: settings-home-form
// reference: settings-home-order
// reference: settings-home-media
pub fn sections<'a>(
    state: &'a State,
    held: jellium_model::prefs::Held,
    configuration: &'a jellium_model::form::Form,
) -> Vec<Element<'a, Message>> {
    let ids: Vec<Uuid> = state.libraries.iter().filter_map(|it| it.id).collect();
    let order = jellium_model::user::ids(configuration, jellium_model::user::ORDERED_VIEWS);
    let hidden = jellium_model::user::ids(configuration, jellium_model::user::MY_MEDIA_EXCLUDES);
    let arranged = jellium_model::user::arranged(&ids, &order, &[]);

    let ordered = widget::list::listed(
        space::ListRow::glyph(space::Lines::One),
        arranged.iter().copied().map(|id| widget::list::Row {
            face: Some(widget::list::Face::Glyph(Icon::FolderOpen)),
            index: None,
            title: named(state, id).into(),
            secondary: Vec::new(),
            press: widget::list::Press::Inert,
            controls: vec![
                widget::icon_button(
                    Icon::KeyboardArrowUp,
                    typeface::ICON_BUTTON,
                    Text::HomeMoveUp,
                    Message::SettingsAction(Action::MoveLibrary {
                        id,
                        toward: jellium_model::user::Toward::Earlier,
                    }),
                ),
                widget::icon_button(
                    Icon::KeyboardArrowDown,
                    typeface::ICON_BUTTON,
                    Text::HomeMoveDown,
                    Message::SettingsAction(Action::MoveLibrary {
                        id,
                        toward: jellium_model::user::Toward::Later,
                    }),
                ),
            ],
        }),
    );

    let shown = widget::labelled(
        Text::HomeInMyMedia,
        column(arranged.iter().copied().map(|id| {
            widget::flag(named(state, id), None, !hidden.contains(&id), move |on| {
                Message::SettingsAction(Action::HideLibrary { id, hidden: !on })
            })
        }))
        .spacing(style::drawn(space::CHECKBOX_LIST_GAP.drawn()))
        .into(),
    );

    vec![
        widget::fields(
            Text::SettingsHome,
            [
                widget::flag(
                    strings::lookup(Text::HomeContinueWatchingRow),
                    None,
                    held.continue_watching,
                    |on| Message::SettingsAction(Action::Set(Setting::ContinueWatchingRow(on))),
                ),
                widget::flag(
                    strings::lookup(Text::HomeNextUpRow),
                    None,
                    held.next_up,
                    |on| Message::SettingsAction(Action::Set(Setting::NextUpRow(on))),
                ),
            ],
        ),
        widget::fields(Text::HomeOrder, [ordered, shown]),
    ]
}
