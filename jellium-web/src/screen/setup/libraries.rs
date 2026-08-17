//! The libraries the server starts with, created from the same content types
//! the dashboard offers.

use iced::widget::{button, column, container, row, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::widget::Choice;

use super::{Action, Edit};
use crate::style::{self, space, typeface};
use crate::widget::prose;

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
            button(prose(
                strings::lookup(Text::SetupLibrariesUp).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::SetupAction(Action::BrowseUp)),
            prose(adding.browsing.clone().unwrap_or_default(), typeface::BODY),
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()));

    for entry in &adding.entries {
        let path = entry.path.clone().unwrap_or_default();
        listing = listing.push(
            row![
                button(prose(
                    entry.name.clone().unwrap_or_default(),
                    typeface::BODY
                ))
                .style(style::flat)
                .on_press(Message::SetupAction(Action::Browse(path.clone()))),
                button(prose("+".to_owned(), typeface::BODY))
                    .on_press(Message::SetupAction(Action::AddPath(path))),
            ]
            .spacing(style::drawn(space::GUTTER.drawn())),
        );
    }
    container(listing).width(Fill).into()
}

fn dialog(adding: &Adding) -> Element<'_, Message> {
    let mut page = column![
        text_input(strings::lookup(Text::LibrariesCreate), &adding.name)
            .style(style::input)
            .on_input(|typed| Message::SetupAction(Action::Edited(Edit::LibraryName(typed)))),
        iced::widget::pick_list(
            crate::screen::dashboard::libraries::content_choices(),
            Some(adding.content_type.clone()),
            |choice| Message::SetupAction(Action::Edited(Edit::ContentType(choice))),
        )
        .width(Fill),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    for (index, path) in adding.paths.iter().enumerate() {
        page = page.push(
            row![
                prose(path.clone(), typeface::BODY),
                button(prose("-".to_owned(), typeface::BODY))
                    .on_press(Message::SetupAction(Action::RemovePath(index))),
            ]
            .spacing(style::drawn(space::GUTTER.drawn())),
        );
    }

    page = page.push(match &adding.browsing {
        Some(_) => browser(adding),
        None => button(prose(
            strings::lookup(Text::LibrariesBrowse).to_owned(),
            typeface::BODY,
        ))
        .on_press(Message::SetupAction(Action::Browse(String::new())))
        .into(),
    });

    page = page.push(
        row![
            button(prose(
                strings::lookup(Text::SetupLibrariesAdd).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::SetupAction(Action::CreateLibrary)),
            button(prose(
                strings::lookup(Text::SetupLibrariesCancel).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::SetupAction(Action::Adding(false))),
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    );
    container(page).width(Fill).into()
}

/// Every library with its rename and remove controls, the add-library dialog,
/// and the sentence stating that the step completes with none configured.
pub fn view(state: &State) -> Element<'_, Message> {
    let mut page = column![
        prose(
            strings::lookup(Text::SetupLibraries).to_owned(),
            typeface::HEADING_3
        ),
        prose(
            strings::lookup(Text::SetupLibrariesEmpty).to_owned(),
            typeface::SECONDARY
        ),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()));

    for folder in &state.folders {
        let name = folder.name.clone().unwrap_or_default();
        let renaming = state
            .renaming
            .as_ref()
            .filter(|(held, _)| *held == name)
            .map(|(_, typed)| typed.clone());
        let shown: Element<'_, Message> = match renaming {
            Some(typed) => row![
                text_input(strings::lookup(Text::LibrariesRename), &typed)
                    .style(style::input)
                    .on_input(|typed| {
                        Message::SetupAction(Action::Edited(Edit::Renaming(typed)))
                    }),
                button(prose(
                    strings::lookup(Text::LibrariesRename).to_owned(),
                    typeface::BODY
                ))
                .on_press(Message::SetupAction(Action::RenameLibrary { name: typed })),
            ]
            .spacing(style::drawn(space::GUTTER.drawn()))
            .into(),
            None => row![
                container(prose(name.clone(), typeface::BODY)).width(Fill),
                button(prose(
                    strings::lookup(Text::LibrariesRename).to_owned(),
                    typeface::BODY
                ))
                .on_press(Message::SetupAction(Action::Renaming {
                    name: name.clone()
                })),
                button(prose(
                    strings::lookup(Text::LibrariesRemove).to_owned(),
                    typeface::BODY
                ))
                .on_press(Message::SetupAction(Action::RemoveLibrary {
                    name: name.clone()
                })),
            ]
            .spacing(style::drawn(space::GUTTER.drawn()))
            .into(),
        };
        page = page.push(shown);
    }

    page = page.push(match &state.adding {
        Some(adding) => dialog(adding),
        None => button(prose(
            strings::lookup(Text::SetupLibrariesAdd).to_owned(),
            typeface::BODY,
        ))
        .on_press(Message::SetupAction(Action::Adding(true)))
        .into(),
    });
    page.into()
}
