use std::collections::HashSet;

use iced::Element;
use iced::widget::{button, column, row};
use jellyfin_api::types::{ImageInfo, RemoteImageInfo};
use uuid::Uuid;

use crate::app::Message;
use crate::images::{self, Cache, Foreign};
use crate::text::{self as strings, Text};
use crate::theme;

use super::Action as Outer;
use crate::style::{self, space, typeface};
use crate::widget::prose;

/// The image kinds the metadata manager uploads, replaces and removes; no other
/// kind carries a control.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Primary,
    Backdrop,
    Thumb,
    Logo,
    Banner,
    Art,
}

impl Kind {
    pub const ALL: [Kind; 6] = [
        Kind::Primary,
        Kind::Backdrop,
        Kind::Thumb,
        Kind::Logo,
        Kind::Banner,
        Kind::Art,
    ];

    pub fn as_str(self) -> &'static str {
        match self {
            Kind::Primary => "Primary",
            Kind::Backdrop => "Backdrop",
            Kind::Thumb => "Thumb",
            Kind::Logo => "Logo",
            Kind::Banner => "Banner",
            Kind::Art => "Art",
        }
    }

    pub fn label(self) -> Text {
        match self {
            Kind::Primary => Text::ArtworkPrimary,
            Kind::Backdrop => Text::ArtworkBackdrop,
            Kind::Thumb => Text::ArtworkThumb,
            Kind::Logo => Text::ArtworkLogo,
            Kind::Banner => Text::ArtworkBanner,
            Kind::Art => Text::ArtworkArt,
        }
    }

    /// The image cache kind this control's images are held under.
    pub fn cached(self) -> images::Kind {
        match self {
            Kind::Primary => images::Kind::Primary,
            Kind::Backdrop => images::Kind::Backdrop,
            Kind::Thumb => images::Kind::Thumb,
            Kind::Logo => images::Kind::Logo,
            Kind::Banner => images::Kind::Banner,
            Kind::Art => images::Kind::Art,
        }
    }

    /// The kind `named` spells, and `None` for a kind no control covers.
    pub fn of(named: &str) -> Option<Kind> {
        Kind::ALL.into_iter().find(|kind| kind.as_str() == named)
    }
}

#[derive(Debug, Clone, Default)]
pub struct State {
    /// The images the item holds, by kind and index.
    pub held: Vec<ImageInfo>,
    /// The kind an upload or a remote search is aimed at.
    pub kind: Option<Kind>,
    /// The providers the server offers remote images from.
    pub providers: Vec<String>,
    pub provider: Option<String>,
    pub remote: Vec<RemoteImageInfo>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Select(Kind),
    Upload,
    Remove {
        kind: Kind,
        index: Option<i32>,
    },
    /// Moves one backdrop to another position.
    Move {
        index: i32,
        to: i32,
    },
    SelectProvider(Option<String>),
    Search,
    /// The Jellyfin server downloads the chosen image; no bytes cross the
    /// browser.
    Download {
        at: usize,
    },
}

/// The images `state` holds of `kind`, with the index each is filed under.
fn held_of(state: &State, kind: Kind) -> Vec<(Option<i32>, &ImageInfo)> {
    state
        .held
        .iter()
        .filter(|held| {
            held.image_type
                .map(|held| held.to_string())
                .is_some_and(|held| held == kind.as_str())
        })
        .map(|held| (held.image_index, held))
        .collect()
}

/// The held images with their removals, the backdrops with their reordering,
/// and the remote picker whose thumbnails are addressed by minted handles.
pub fn view<'a>(
    state: &'a State,
    images: &'a Cache,
    foreign: &'a Foreign,
    item: Uuid,
    read_only: bool,
) -> Element<'a, Message> {
    let strip = row(Kind::ALL.into_iter().map(|kind| {
        let mut control = button(prose(
            strings::lookup(kind.label()).to_owned(),
            typeface::BODY,
        ));
        if state.kind != Some(kind) {
            control = control.on_press(Message::MetadataAction(Outer::Artwork(Action::Select(
                kind,
            ))));
        }
        control.into()
    }))
    .spacing(style::drawn(space::GUTTER.drawn()));

    let mut page = column![strip].spacing(style::drawn(space::GUTTER.drawn()));

    for kind in Kind::ALL {
        let held = held_of(state, kind);
        if held.is_empty() {
            continue;
        }
        let count = held.len();
        let mut shown = row![prose(
            strings::lookup(kind.label()).to_owned(),
            typeface::BODY
        )]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .align_y(iced::Alignment::Center);

        for (position, (index, _)) in held.into_iter().enumerate() {
            let handle = images.handle(images::Key {
                item,
                kind: kind.cached(),
                index,
                width: theme::IMAGE_WIDTH,
            });
            let drawn: Element<'a, Message> = match handle {
                Some(held) => iced::widget::image(held).width(theme::CARD_WIDTH).into(),
                None => iced::widget::Space::new()
                    .width(theme::CARD_WIDTH)
                    .height(theme::CARD_WIDTH * 0.6)
                    .into(),
            };

            let mut cell = column![drawn].spacing(style::drawn(space::BLOCK_GAP.drawn()));
            if !read_only {
                cell = cell.push(
                    button(prose(
                        strings::lookup(Text::ArtworkRemove).to_owned(),
                        typeface::BODY,
                    ))
                    .on_press(Message::MetadataAction(Outer::Artwork(
                        Action::Remove { kind, index },
                    ))),
                );
                if kind == Kind::Backdrop
                    && let Some(at) = index
                {
                    if position > 0 {
                        cell = cell.push(
                            button(prose(
                                strings::lookup(Text::ArtworkMoveEarlier).to_owned(),
                                typeface::BODY,
                            ))
                            .on_press(Message::MetadataAction(
                                Outer::Artwork(Action::Move {
                                    index: at,
                                    to: at - 1,
                                }),
                            )),
                        );
                    }
                    if position + 1 < count {
                        cell = cell.push(
                            button(prose(
                                strings::lookup(Text::ArtworkMoveLater).to_owned(),
                                typeface::BODY,
                            ))
                            .on_press(Message::MetadataAction(
                                Outer::Artwork(Action::Move {
                                    index: at,
                                    to: at + 1,
                                }),
                            )),
                        );
                    }
                }
            }
            shown = shown.push(cell);
        }
        page = page.push(shown);
    }

    if read_only || state.kind.is_none() {
        return page.into();
    }

    page = page.push(
        button(prose(
            strings::lookup(Text::ArtworkUpload).to_owned(),
            typeface::BODY,
        ))
        .on_press(Message::MetadataAction(Outer::Artwork(Action::Upload))),
    );

    let providers = row(state.providers.iter().map(|provider| {
        let named = provider.clone();
        button(prose(provider.clone(), typeface::BODY))
            .style(style::flat)
            .on_press(Message::MetadataAction(Outer::Artwork(
                Action::SelectProvider(Some(named)),
            )))
            .into()
    }))
    .spacing(style::drawn(space::GUTTER.drawn()));

    page = page.push(providers).push(
        button(prose(
            strings::lookup(Text::ArtworkSearch).to_owned(),
            typeface::BODY,
        ))
        .on_press(Message::MetadataAction(Outer::Artwork(Action::Search))),
    );

    let found = row(state.remote.iter().enumerate().map(|(at, remote)| {
        let drawn: Element<'a, Message> = match remote
            .thumbnail_url
            .as_deref()
            .or(remote.url.as_deref())
            .and_then(|handle| foreign.handle(handle))
        {
            Some(held) => iced::widget::image(held).width(theme::CARD_WIDTH).into(),
            None => iced::widget::Space::new()
                .width(theme::CARD_WIDTH)
                .height(theme::CARD_WIDTH * 0.6)
                .into(),
        };
        column![
            drawn,
            button(prose(
                strings::lookup(Text::ArtworkDownload).to_owned(),
                typeface::BODY
            ))
            .on_press(Message::MetadataAction(Outer::Artwork(Action::Download {
                at
            }))),
        ]
        .spacing(style::drawn(space::BLOCK_GAP.drawn()))
        .into()
    }))
    .spacing(style::drawn(space::GUTTER.drawn()));

    page.push(found).into()
}

pub fn images(state: &State, item: Uuid) -> HashSet<images::Key> {
    state
        .held
        .iter()
        .filter_map(|held| {
            let kind = Kind::of(&held.image_type?.to_string())?;
            Some(images::Key {
                item,
                kind: kind.cached(),
                index: held.image_index,
                width: theme::IMAGE_WIDTH,
            })
        })
        .collect()
}

/// The handles this surface draws, so the session fetches each once.
pub fn handles(state: &State) -> HashSet<String> {
    state
        .remote
        .iter()
        .filter_map(|remote| remote.thumbnail_url.clone().or_else(|| remote.url.clone()))
        .collect()
}
