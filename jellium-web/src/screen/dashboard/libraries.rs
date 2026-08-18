//! The virtual folders the server holds, their media paths and their options.

use iced::Element;
use iced::widget::{button, row, text_input};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{self, prose};
use jellium_model::appearance::typeface::Rank;
use jellium_model::form::{Field, Form};

use super::{Control, Group, Heading};

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

/// The label one content type is offered under, and `Other` where the server
/// names none.
// reference: dashboard-library-card
fn content_label(kind: Option<jellyfin_api::types::CollectionTypeOptions>) -> Text {
    use jellyfin_api::types::CollectionTypeOptions as Kind;
    match kind {
        Some(Kind::Movies) => Text::LibrariesContentMovies,
        Some(Kind::Tvshows) => Text::LibrariesContentShows,
        Some(Kind::Music) => Text::LibrariesContentMusic,
        Some(Kind::Musicvideos) => Text::LibrariesContentMusicVideos,
        Some(Kind::Homevideos) => Text::LibrariesContentHomeVideos,
        Some(Kind::Boxsets) => Text::LibrariesContentCollections,
        Some(Kind::Books) => Text::LibrariesContentBooks,
        Some(Kind::Mixed) => Text::LibrariesContentMixed,
        None => Text::LibrariesContentOther,
    }
}

/// The library type `getLibraryIcon` reads, a virtual folder naming its own
/// type in the words a library item names its; mixed content names no library
/// type and takes the glyph the reference's default arm writes.
// reference: dashboard-library-card
fn collected(
    kind: Option<jellyfin_api::types::CollectionTypeOptions>,
) -> Option<jellyfin_api::types::CollectionType> {
    use jellyfin_api::types::CollectionType as Named;
    use jellyfin_api::types::CollectionTypeOptions as Kind;
    match kind {
        Some(Kind::Movies) => Some(Named::Movies),
        Some(Kind::Tvshows) => Some(Named::Tvshows),
        Some(Kind::Music) => Some(Named::Music),
        Some(Kind::Musicvideos) => Some(Named::Musicvideos),
        Some(Kind::Homevideos) => Some(Named::Homevideos),
        Some(Kind::Boxsets) => Some(Named::Boxsets),
        Some(Kind::Books) => Some(Named::Books),
        Some(Kind::Mixed) | None => None,
    }
}

/// The item a virtual folder names, and nothing where the server names one
/// this client cannot read; the one site that reads a folder's identifier.
fn named(folder: &jellyfin_api::types::VirtualFolderInfo) -> Option<uuid::Uuid> {
    let Ok(named) = crate::failure::unraised::read::<uuid::Uuid>(folder.item_id.as_deref()?) else {
        return None;
    };
    Some(named)
}

/// That item where the server holds a primary image for it, and nothing where
/// it holds none.
// reference: dashboard-library-card
fn imaged(folder: &jellyfin_api::types::VirtualFolderInfo) -> Option<uuid::Uuid> {
    folder.primary_image_item_id.as_ref()?;
    named(folder)
}

/// The image every library with one draws on its card.
// reference: dashboard-library-card
pub fn images(state: &State) -> std::collections::HashSet<crate::images::Key> {
    state
        .folders
        .iter()
        .filter_map(imaged)
        .map(|item| crate::images::Key {
            item,
            kind: crate::images::Kind::Primary,
            index: None,
            card: card::Card::Wall(card::Shape::Portrait),
        })
        .collect()
}

/// Every content type as a picker option.
pub fn content_choices() -> Vec<crate::widget::Choice<String>> {
    CONTENT_TYPES
        .into_iter()
        .map(|kind| crate::widget::Choice {
            label: strings::lookup(content_label(Some(kind))).to_string(),
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
    pub content_type: crate::widget::Choice<String>,
    /// How far a scan of each library has got, by the item the refresh names.
    pub refreshing: std::collections::HashMap<uuid::Uuid, f64>,
    /// The library whose commands are open, by name.
    pub menu: Option<String>,
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
            menu: None,
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

/// The library options the reference's own editor draws, in its order; every
/// key outside them survives a save.
// reference: library-options-heading
// reference: library-options-metadata-language
// reference: library-options-photos
// reference: library-options-realtime
// reference: library-options-metadata-readers
// reference: library-options-save-local
// reference: library-options-chapters
// reference: library-options-subtitle-languages
// reference: library-options-subtitle-fetchers
// reference: library-options-save-subtitles
pub const OPTIONS: &[Group] = &[
    Group {
        heading: Some(Heading {
            rank: Rank::Second,
            title: Text::LibrariesSettings,
        }),
        note: None,
        controls: &[
            Control {
                field: Field::Text {
                    key: "PreferredMetadataLanguage",
                },
                label: Text::LibrariesMetadataLanguage,
                helper: &[],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Text {
                    key: "MetadataCountryCode",
                },
                label: Text::LibrariesMetadataCountry,
                helper: &[],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Flag {
                    key: "EnablePhotos",
                },
                label: Text::LibrariesPhotos,
                helper: &[Text::LibrariesPhotosHelp],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Flag {
                    key: "EnableRealtimeMonitor",
                },
                label: Text::LibrariesRealtimeMonitor,
                helper: &[Text::LibrariesRealtimeMonitorHelp],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Lines {
                    key: "LocalMetadataReaderOrder",
                },
                label: Text::LibrariesMetadataReaders,
                helper: &[Text::LibrariesMetadataReadersHelp],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Flag {
                    key: "SaveLocalMetadata",
                },
                label: Text::LibrariesSaveArtwork,
                helper: &[Text::LibrariesSaveArtworkHelp],
                unit: None,
                offered: None,
            },
        ],
        closing: None,
    },
    Group {
        heading: Some(Heading {
            rank: Rank::Second,
            title: Text::LibrariesChapterImages,
        }),
        note: None,
        controls: &[Control {
            field: Field::Flag {
                key: "EnableChapterImageExtraction",
            },
            label: Text::LibrariesChapterExtraction,
            helper: &[Text::LibrariesChapterExtractionHelp],
            unit: None,
            offered: None,
        }],
        closing: None,
    },
    Group {
        heading: Some(Heading {
            rank: Rank::Second,
            title: Text::LibrariesSubtitleDownloads,
        }),
        note: None,
        controls: &[
            Control {
                field: Field::Lines {
                    key: "SubtitleDownloadLanguages",
                },
                label: Text::LibrariesDownloadLanguages,
                helper: &[],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Lines {
                    key: "SubtitleFetcherOrder",
                },
                label: Text::LibrariesSubtitleDownloaders,
                helper: &[Text::LibrariesSubtitleDownloadersHelp],
                unit: None,
                offered: None,
            },
            Control {
                field: Field::Flag {
                    key: "SaveSubtitlesWithMedia",
                },
                label: Text::LibrariesSaveSubtitles,
                helper: &[Text::LibrariesSaveSubtitlesHelp],
                unit: None,
                offered: None,
            },
        ],
        closing: None,
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

/// The commands one library's card offers: the screen that manages it, its
/// scan, and its removal, the last two absent under read-only.
// the reference floats these rows over the card in a `MuiMenu`; here they
// stand in the page above the grid
// the reference's own scan row carries `refresh`, which this client's icon
// font does not hold
// reference: dashboard-library-card
fn menu<'a>(open: &'a str, read_only: bool) -> Element<'a, Message> {
    let mut rows = vec![crate::widget::list::Row {
        face: Some(crate::widget::list::Face::Glyph(crate::icon::Icon::Folder)),
        index: None,
        title: strings::lookup(Text::LibrariesManage).into(),
        secondary: Vec::new(),
        press: crate::widget::list::Press::Whole(Message::DashboardAction(super::Action::Open(
            super::Screen::Library {
                name: open.to_owned(),
            },
        ))),
        controls: Vec::new(),
    }];
    if !read_only {
        rows.push(crate::widget::list::Row {
            face: Some(crate::widget::list::Face::Glyph(
                crate::icon::Icon::Autorenew,
            )),
            index: None,
            title: strings::lookup(Text::LibrariesScan).into(),
            secondary: Vec::new(),
            press: crate::widget::list::Press::Whole(Message::DashboardAction(
                super::Action::Write(super::Written::ScanLibrary {
                    name: open.to_owned(),
                }),
            )),
            controls: Vec::new(),
        });
        rows.push(crate::widget::list::Row {
            face: Some(crate::widget::list::Face::Glyph(crate::icon::Icon::Delete)),
            index: None,
            title: strings::lookup(Text::LibrariesRemove).into(),
            secondary: Vec::new(),
            press: crate::widget::list::Press::Whole(Message::DashboardAction(super::Action::Ask(
                crate::screen::confirm::Pending::of(
                    crate::screen::confirm::Destructive::DeleteLibrary {
                        name: open.to_owned(),
                    },
                    open.to_owned(),
                ),
            ))),
            controls: Vec::new(),
        });
    }
    crate::widget::list::listed(space::ListRow::glyph(space::Lines::One), rows)
}

/// Every library on the card `LibraryCard` draws, in the grid its own ladder
/// lays them in, with the create row above and the open card's commands under
/// it.
// reference: dashboard-libraries-grid
// reference: dashboard-library-card
pub fn view<'a>(
    state: &'a State,
    read_only: bool,
    images: &'a crate::images::Cache,
    viewport: style::Viewport,
) -> Vec<Element<'a, Message>> {
    let mut page: Vec<Element<'a, Message>> = Vec::new();

    if !read_only {
        page.push(
            row![
                text_input(strings::lookup(Text::LibrariesCreate), &state.naming)
                    .style(style::input)
                    .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
                iced::widget::pick_list(
                    content_choices(),
                    Some(state.content_type.clone()),
                    |choice| Message::DashboardAction(super::Action::ContentType(choice)),
                ),
                button(prose(
                    strings::lookup(Text::LibrariesCreate),
                    typeface::BODY
                ))
                .style(style::submit)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::CreateLibrary {
                        name: state.naming.clone(),
                        content_type: state.content_type.value.clone(),
                    }
                ))),
            ]
            .spacing(style::drawn(space::CONTROL_GAP.drawn()))
            .into(),
        );
    }

    if let Some(open) = state.menu.as_deref() {
        page.push(menu(open, read_only));
    }

    page.push(crate::widget::mui::grid(
        space::LIBRARY_CELL,
        state.folders.iter().map(|folder| {
            let name = folder.name.clone().unwrap_or_default();
            let media = match imaged(folder).and_then(|item| {
                images.handle(crate::images::Key {
                    item,
                    kind: crate::images::Kind::Primary,
                    index: None,
                    card: card::Card::Wall(card::Shape::Portrait),
                })
            }) {
                Some(handle) => crate::widget::mui::Media::Image(handle),
                None => crate::widget::mui::Media::Glyph(
                    crate::icon::Icon::library(collected(folder.collection_type)),
                    typeface::LIBRARY_CARD_ICON,
                ),
            };
            let said = match named(folder).and_then(|id| state.refreshing.get(&id)) {
                Some(progress) => {
                    strings::format(Text::LibrariesScanning, &[&format!("{progress:.0}")])
                }
                None => strings::lookup(content_label(folder.collection_type)).to_owned(),
            };
            crate::widget::mui::card(
                crate::widget::mui::Card {
                    title: name.clone().into(),
                    text: Some(said.into()),
                    media,
                    height: space::LIBRARY_CARD,
                    opens: Some(Message::DashboardAction(super::Action::Open(
                        super::Screen::Library { name: name.clone() },
                    ))),
                    action: Some(Message::DashboardAction(super::Action::LibraryMenu(
                        match state.menu.as_deref() == Some(name.as_str()) {
                            true => None,
                            false => Some(name),
                        },
                    ))),
                },
                viewport.layout(),
            )
        }),
        viewport,
    ));

    page
}

/// One library: its paths under the heading the reference writes over them,
/// the browser a path is chosen from, and its options.
// reference: library-folders
pub fn one<'a>(
    state: &'a One,
    read_only: bool,
    viewport: style::Viewport,
) -> Vec<Element<'a, Message>> {
    let mut page: Vec<Element<'a, Message>> = Vec::new();

    if !read_only {
        page.push(
            row![
                text_input(strings::lookup(Text::LibrariesRename), &state.renaming)
                    .style(style::input)
                    .on_input(|typed| Message::DashboardAction(super::Action::Typed(typed))),
                button(prose(
                    strings::lookup(Text::LibrariesRename),
                    typeface::BODY
                ))
                .style(style::submit)
                .on_press(Message::DashboardAction(super::Action::Write(
                    super::Written::RenameLibrary {
                        name: state.name.clone(),
                        renamed: state.renaming.clone(),
                    }
                ))),
            ]
            .spacing(style::drawn(space::CONTROL_GAP.drawn()))
            .into(),
        );
    }

    page.push(widget::heading(
        typeface::Rank::First,
        strings::lookup(Text::LibrariesFolders),
    ));
    for path in &state.paths {
        let mut held = row![prose(path.clone(), typeface::BODY)]
            .spacing(style::drawn(space::CONTROL_GAP.drawn()));
        if !read_only {
            held = held.push(
                button(prose(
                    strings::lookup(Text::LibrariesPathRemove),
                    typeface::BODY,
                ))
                .style(style::raised)
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
        page.push(held.into());
    }

    if !read_only {
        page.push(
            button(prose(
                strings::lookup(Text::LibrariesBrowse),
                typeface::BODY,
            ))
            .style(style::raised)
            .on_press(Message::DashboardAction(super::Action::Browse(
                state.browsing.clone().unwrap_or_default(),
            )))
            .into(),
        );
        for entry in &state.entries {
            let Some(path) = entry.path.clone() else {
                continue;
            };
            page.push(
                row![
                    widget::anchor(
                        entry.name.clone().unwrap_or_default(),
                        Message::DashboardAction(super::Action::Browse(path.clone())),
                    ),
                    button(prose(
                        strings::lookup(Text::LibrariesPathAdd),
                        typeface::BODY
                    ))
                    .style(style::submit)
                    .on_press(Message::DashboardAction(super::Action::Write(
                        super::Written::AddPath {
                            library: state.name.clone(),
                            path,
                        }
                    ))),
                ]
                .spacing(style::drawn(space::CONTROL_GAP.drawn()))
                .into(),
            );
        }
    }

    page.push(super::controls(
        OPTIONS,
        &state.options,
        super::Controls::Emby,
        viewport,
    ));
    if !read_only {
        page.push(super::save(
            super::Controls::Emby,
            Some(Message::DashboardAction(super::Action::Save)),
            viewport.layout(),
        ));
    }

    page
}
