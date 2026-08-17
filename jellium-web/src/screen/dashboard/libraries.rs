//! The virtual folders the server holds, their media paths and their options.

use iced::widget::{button, column, row, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::typeface;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::prose;
use jellium_model::form::{Field, Form};

/// The content types a library can be created as, in the order both the
/// dashboard and the wizard offer them.
pub const CONTENT_TYPES: [jellyfin_api::types::CollectionTypeOptions; 8] = {
    use jellyfin_api::types::CollectionTypeOptions as Kind;
    [
        Kind::Movies,
        Kind::Tvshows,
        Kind::Music,
        Kind::Musicvideos,
        Kind::Homevideos,
        Kind::Boxsets,
        Kind::Books,
        Kind::Mixed,
    ]
};

/// The label one content type is offered under.
fn content_label(kind: jellyfin_api::types::CollectionTypeOptions) -> Text {
    use jellyfin_api::types::CollectionTypeOptions as Kind;
    match kind {
        Kind::Movies => Text::LibrariesContentMovies,
        Kind::Tvshows => Text::LibrariesContentShows,
        Kind::Music => Text::LibrariesContentMusic,
        Kind::Musicvideos => Text::LibrariesContentMusicVideos,
        Kind::Homevideos => Text::LibrariesContentHomeVideos,
        Kind::Boxsets => Text::LibrariesContentCollections,
        Kind::Books => Text::LibrariesContentBooks,
        Kind::Mixed => Text::LibrariesContentMixed,
    }
}

/// Every content type as a picker option.
pub fn content_choices() -> Vec<crate::widget::Choice> {
    CONTENT_TYPES
        .into_iter()
        .map(|kind| crate::widget::Choice {
            label: strings::lookup(content_label(kind)).to_string(),
            value: kind.to_string(),
        })
        .collect()
}

/// Every virtual folder, and what a new one is being named.
#[derive(Debug, Clone)]
pub struct State {
    pub folders: Vec<jellyfin_api::types::VirtualFolderInfo>,
    /// The name typed for a new library, and the content type chosen for it.
    pub naming: String,
    pub content_type: crate::widget::Choice,
    /// How far a scan of each library has got, by the item the refresh names.
    pub refreshing: std::collections::HashMap<uuid::Uuid, f64>,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            folders: api.virtual_folders().await.bubbled()?,
            naming: String::new(),
            content_type: content_choices()
                .into_iter()
                .next()
                .expect("the content types are not empty"),
            refreshing: std::collections::HashMap::new(),
        })
    })
    .await
}

/// One library's screen: its paths, its options and the filesystem browser a
/// path is chosen from.
#[derive(Debug, Clone)]
pub struct One {
    pub name: String,
    /// The virtual folder's own id, which is what a save of its options names.
    pub id: String,
    pub paths: Vec<String>,
    pub options: Form,
    /// The directory the browser stands in, and what it holds.
    pub browsing: Option<String>,
    pub entries: Vec<jellyfin_api::types::FileSystemEntryInfo>,
    /// The new name typed for a rename.
    pub renaming: String,
}

/// The fields a library's options expose; every key outside them survives a
/// save.
pub const OPTIONS: &[Field] = &[
    Field::Flag {
        key: "EnableRealtimeMonitor",
    },
    Field::Flag {
        key: "EnableChapterImageExtraction",
    },
    Field::Flag {
        key: "SaveLocalMetadata",
    },
    Field::Flag {
        key: "EnableInternetProviders",
    },
    Field::Flag {
        key: "EnablePhotos",
    },
    Field::Flag {
        key: "SaveSubtitlesWithMedia",
    },
    Field::Text {
        key: "MetadataCountryCode",
    },
    Field::Text {
        key: "PreferredMetadataLanguage",
    },
    Field::Lines {
        key: "DisabledLocalMetadataReaders",
    },
    Field::Lines {
        key: "LocalMetadataReaderOrder",
    },
    Field::Lines {
        key: "DisabledSubtitleFetchers",
    },
    Field::Lines {
        key: "SubtitleFetcherOrder",
    },
    Field::Lines {
        key: "SubtitleDownloadLanguages",
    },
];

pub async fn open(api: std::rc::Rc<crate::api::Api>, name: String) -> Answer<One> {
    Answer::of(async {
        let folders = api.virtual_folders().await.bubbled()?;
        let folder = folders
            .iter()
            .find(|folder| folder.name.as_deref() == Some(name.as_str()))
            .cloned()
            .unwrap_or_default();
        Ok(One {
            id: folder.item_id.clone().unwrap_or_default(),
            paths: folder.locations.clone().unwrap_or_default(),
            options: Form::of(api.library_options(&name).await.bubbled()?),
            name,
            browsing: None,
            entries: Vec::new(),
            renaming: String::new(),
        })
    })
    .await
}

/// Takes the refresh progress one push carried, so a library's scan moves
/// without the user acting.
pub fn refreshed(state: &mut State, items: &[jellium_protocol::Refreshed]) {
    for item in items {
        state.refreshing.insert(item.item, item.progress);
    }
}

/// Every library with its scan control, and the control that creates one.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page = column![prose(
        strings::lookup(Text::LibrariesTitle).to_owned(),
        typeface::HEADING_2
    )]
    .spacing(theme::CARD_SPACING)
    .padding(theme::CARD_SPACING);

    if !read_only {
        page = page.push(
            row![
                text_input(strings::lookup(Text::LibrariesCreate), &state.naming)
                    .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
                iced::widget::pick_list(
                    content_choices(),
                    Some(state.content_type.clone()),
                    |choice| Message::DashboardAction(super::Action::ContentType(choice)),
                ),
                button(prose(
                    strings::lookup(Text::LibrariesCreate).to_owned(),
                    typeface::BODY
                ))
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::CreateLibrary {
                        name: state.naming.clone(),
                        content_type: state.content_type.value.clone(),
                    }
                ))),
            ]
            .spacing(theme::CARD_SPACING),
        );
    }

    for folder in &state.folders {
        let name = folder.name.clone().unwrap_or_default();
        let mut held =
            row![
                button(prose(name.clone(), typeface::BODY)).on_press(Message::DashboardAction(
                    super::Action::Open(super::Screen::Library { name: name.clone() })
                )),
            ]
            .spacing(theme::CARD_SPACING);

        let named = folder
            .item_id
            .as_deref()
            .and_then(|id| crate::failure::read::<uuid::Uuid>(Text::FailureLibraryId, id));
        if let Some(progress) = named.and_then(|id| state.refreshing.get(&id)) {
            held = held.push(prose(
                strings::format(Text::LibrariesScanning, &[&format!("{progress:.0}")]),
                typeface::BODY,
            ));
        }

        if !read_only {
            held = held.push(
                button(prose(
                    strings::lookup(Text::LibrariesScan).to_owned(),
                    typeface::BODY,
                ))
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::ScanLibrary { name: name.clone() },
                ))),
            );
            held = held.push(
                button(prose(
                    strings::lookup(Text::LibrariesRemove).to_owned(),
                    typeface::BODY,
                ))
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::DeleteLibrary { name: name.clone() },
                        name,
                    ),
                ))),
            );
        }
        page = page.push(held);
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}

/// One library: its paths, the browser a path is chosen from, and its options.
pub fn one<'a>(state: &'a One, read_only: bool) -> Element<'a, Message> {
    let mut page = column![prose(state.name.clone(), typeface::HEADING_2)]
        .spacing(theme::CARD_SPACING)
        .padding(theme::CARD_SPACING);

    if !read_only {
        page = page.push(
            row![
                text_input(strings::lookup(Text::LibrariesRename), &state.renaming)
                    .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
                button(prose(
                    strings::lookup(Text::LibrariesRename).to_owned(),
                    typeface::BODY
                ))
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::RenameLibrary {
                        name: state.name.clone(),
                        renamed: state.renaming.clone(),
                    }
                ))),
            ]
            .spacing(theme::CARD_SPACING),
        );
    }

    page = page.push(prose(
        strings::lookup(Text::LibrariesPaths).to_owned(),
        typeface::BODY,
    ));
    for path in &state.paths {
        let mut held = row![prose(path.clone(), typeface::BODY)].spacing(theme::CARD_SPACING);
        if !read_only {
            held = held.push(
                button(prose(
                    strings::lookup(Text::LibrariesPathRemove).to_owned(),
                    typeface::BODY,
                ))
                .on_press(Message::DashboardAction(super::Action::Ask(
                    crate::screen::confirm::Pending::of(
                        crate::screen::confirm::Destructive::DeletePath {
                            library: state.name.clone(),
                            path: path.clone(),
                        },
                        path.clone(),
                    ),
                ))),
            );
        }
        page = page.push(held);
    }

    if !read_only {
        page = page.push(
            button(prose(
                strings::lookup(Text::LibrariesBrowse).to_owned(),
                typeface::BODY,
            ))
            .on_press(Message::DashboardAction(super::Action::Browse(
                state.browsing.clone().unwrap_or_default(),
            ))),
        );
        for entry in &state.entries {
            let Some(path) = entry.path.clone() else {
                continue;
            };
            page = page.push(
                row![
                    button(prose(
                        entry.name.clone().unwrap_or_default(),
                        typeface::BODY
                    ))
                    .on_press(Message::DashboardAction(
                        super::Action::Browse(path.clone())
                    )),
                    button(prose(
                        strings::lookup(Text::LibrariesPathAdd).to_owned(),
                        typeface::BODY
                    ))
                    .on_press(Message::DashboardAction(super::Action::Write(
                        super::Written::AddPath {
                            library: state.name.clone(),
                            path,
                        }
                    ))),
                ]
                .spacing(theme::CARD_SPACING),
            );
        }
    }

    page = page.push(prose(
        strings::lookup(Text::LibrariesOptions).to_owned(),
        typeface::BODY,
    ));
    for field in OPTIONS {
        page = page.push(super::control(*field, state.options.value(*field), false));
    }
    if !read_only {
        page = page.push(
            button(prose(
                strings::lookup(Text::DashboardSave).to_owned(),
                typeface::BODY,
            ))
            .on_press(Message::DashboardAction(super::Action::Save)),
        );
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}
