//! The libraries the server starts with, created from the same content types
//! the dashboard offers.

use iced::widget::{button, column, container, row, text, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::Choice;

use super::{Action, Edit};

#[derive(Debug, Clone)]
pub struct State {
    pub folders: Vec<jellyfin_api::types::VirtualFolderInfo>,
    /// The library being added, and `None` while the dialog is closed.
    pub adding: Option<Adding>,
    /// The library being renamed and the name typed for it.
    pub renaming: Option<(String, String)>,
}

/// The reduced add-library dialog: a name, a content type and media paths, and
/// nothing else.
#[derive(Debug, Clone)]
pub struct Adding {
    pub name: String,
    pub content_type: Choice,
    pub paths: Vec<String>,
    /// The directory the browser stands in, and what it holds.
    pub browsing: Option<String>,
    pub entries: Vec<jellyfin_api::types::FileSystemEntryInfo>,
}

impl Adding {
    fn new() -> Adding {
        Adding {
            name: String::new(),
            content_type: crate::screen::dashboard::libraries::content_choices()
                .into_iter()
                .next()
                .expect("the content types are not empty"),
            paths: Vec::new(),
            browsing: None,
            entries: Vec::new(),
        }
    }
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            folders: api.virtual_folders().await.bubbled()?,
            adding: None,
            renaming: None,
        })
    })
    .await
}

/// Opens the add-library dialog, and abandons it.
pub fn adding(state: &mut State, open: bool) {
    state.adding = open.then(Adding::new);
}

fn browser(adding: &Adding) -> Element<'_, Message> {
    let mut listing = column![
        row![
            button(text(strings::lookup(Text::SetupLibrariesUp)))
                .on_press(Message::SetupAction(Action::BrowseUp)),
            text(adding.browsing.clone().unwrap_or_default()),
        ]
        .spacing(theme::CARD_SPACING),
    ]
    .spacing(4);

    for entry in &adding.entries {
        let path = entry.path.clone().unwrap_or_default();
        listing = listing.push(
            row![
                button(text(entry.name.clone().unwrap_or_default()))
                    .style(button::text)
                    .on_press(Message::SetupAction(Action::Browse(path.clone()))),
                button(text("+")).on_press(Message::SetupAction(Action::AddPath(path))),
            ]
            .spacing(theme::CARD_SPACING),
        );
    }
    container(listing).width(Fill).into()
}

fn dialog(adding: &Adding) -> Element<'_, Message> {
    let mut page = column![
        text_input(strings::lookup(Text::LibrariesCreate), &adding.name)
            .on_input(|typed| Message::SetupAction(Action::Edited(Edit::LibraryName(typed)))),
        iced::widget::pick_list(
            crate::screen::dashboard::libraries::content_choices(),
            Some(adding.content_type.clone()),
            |choice| Message::SetupAction(Action::Edited(Edit::ContentType(choice))),
        )
        .width(Fill),
    ]
    .spacing(theme::CARD_SPACING);

    for (index, path) in adding.paths.iter().enumerate() {
        page = page.push(
            row![
                text(path.clone()),
                button(text("-")).on_press(Message::SetupAction(Action::RemovePath(index))),
            ]
            .spacing(theme::CARD_SPACING),
        );
    }

    page = page.push(match &adding.browsing {
        Some(_) => browser(adding),
        None => button(text(strings::lookup(Text::LibrariesBrowse)))
            .on_press(Message::SetupAction(Action::Browse(String::new())))
            .into(),
    });

    page = page.push(
        row![
            button(text(strings::lookup(Text::SetupLibrariesAdd)))
                .on_press(Message::SetupAction(Action::CreateLibrary)),
            button(text(strings::lookup(Text::SetupLibrariesCancel)))
                .on_press(Message::SetupAction(Action::Adding(false))),
        ]
        .spacing(theme::CARD_SPACING),
    );
    container(page).width(Fill).into()
}

/// Every library with its rename and remove controls, the add-library dialog,
/// and the sentence stating that the step completes with none configured.
pub fn view(state: &State) -> Element<'_, Message> {
    let mut page = column![
        text(strings::lookup(Text::SetupLibraries)).size(20),
        text(strings::lookup(Text::SetupLibrariesEmpty)).size(13),
    ]
    .spacing(theme::CARD_SPACING);

    for folder in &state.folders {
        let name = folder.name.clone().unwrap_or_default();
        let renaming = state
            .renaming
            .as_ref()
            .filter(|(held, _)| *held == name)
            .map(|(_, typed)| typed.clone());
        let shown: Element<'_, Message> = match renaming {
            Some(typed) => row![
                text_input(strings::lookup(Text::LibrariesRename), &typed).on_input(|typed| {
                    Message::SetupAction(Action::Edited(Edit::Renaming(typed)))
                }),
                button(text(strings::lookup(Text::LibrariesRename)))
                    .on_press(Message::SetupAction(Action::RenameLibrary { name: typed })),
            ]
            .spacing(theme::CARD_SPACING)
            .into(),
            None => row![
                text(name.clone()).width(Fill),
                button(text(strings::lookup(Text::LibrariesRename))).on_press(
                    Message::SetupAction(Action::Renaming { name: name.clone() })
                ),
                button(text(strings::lookup(Text::LibrariesRemove))).on_press(
                    Message::SetupAction(Action::RemoveLibrary { name: name.clone() })
                ),
            ]
            .spacing(theme::CARD_SPACING)
            .into(),
        };
        page = page.push(shown);
    }

    page = page.push(match &state.adding {
        Some(adding) => dialog(adding),
        None => button(text(strings::lookup(Text::SetupLibrariesAdd)))
            .on_press(Message::SetupAction(Action::Adding(true)))
            .into(),
    });
    page.into()
}
