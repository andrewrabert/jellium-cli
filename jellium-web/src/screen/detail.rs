use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{Space, button, column, container, image, row, scrollable, stack};
use iced::{Element, Fill};
use jellium_model::item::{self, Mark};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache, Kind};
use crate::player::Intent;
use crate::style::space::Room;
use crate::style::{self, Band, Drawn, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;
use crate::widget::prose;

#[derive(Debug, Clone)]
pub struct State {
    pub item: BaseItemDto,
    pub children: Vec<BaseItemDto>,
    /// What the server says is like this item.
    pub similar: Vec<BaseItemDto>,
}

/// The most items the similar rail shows.
pub const SIMILAR: i32 = 16;

pub async fn load(api: Rc<Api>, item: Uuid) -> Answer<State> {
    Answer::of(async {
        let id = item;
        let item = api.item(item).await.bubbled()?;
        Ok(State {
            children: api.children(&item).await.bubbled()?,
            similar: api
                .similar(id, SIMILAR)
                .await
                .or_default(Text::FailureSimilarUnread),
            item,
        })
    })
    .await
}

fn children_heading(kind: Option<BaseItemKind>) -> Text {
    match kind {
        Some(BaseItemKind::Series) => Text::DetailSeasons,
        Some(BaseItemKind::Season) => Text::DetailEpisodes,
        Some(BaseItemKind::MusicAlbum) => Text::DetailTracks,
        Some(BaseItemKind::MusicArtist) => Text::DetailAlbums,
        _ => Text::DetailEpisodes,
    }
}

fn heading(item: &BaseItemDto) -> String {
    match (
        &item.series_name,
        item.parent_index_number,
        item.index_number,
    ) {
        (Some(series), Some(season), Some(episode)) => {
            format!("{series} S{season}E{episode}")
        }
        (Some(series), _, _) => series.clone(),
        (None, _, _) => item
            .album_artist
            .clone()
            .or(item.production_year.map(|year| year.to_string()))
            .unwrap_or_default(),
    }
}

/// True when the server stored a position for an item that is not marked
/// played, which is what Resume begins from.
fn resumable(item: &BaseItemDto) -> bool {
    item::played(item) == Mark::Cleared
        && item
            .user_data
            .as_ref()
            .and_then(|data| data.playback_position_ticks)
            .is_some_and(|ticks| ticks > 0)
}

fn intent_button<'a>(label: Text, intent: Intent) -> Element<'a, Message> {
    button(prose(strings::lookup(label), typeface::BODY))
        .style(style::raised)
        .on_press(Message::PlayPressed(intent))
        .into()
}

/// Play, and Resume when the item has a stored position and is not marked
/// played, for a movie, episode, music video or song.
/// Play All and Shuffle for a series, season, album or artist.
/// Instant Mix for a song, album or artist.
fn play_controls<'a>(item: &BaseItemDto) -> Vec<Element<'a, Message>> {
    let Some(id) = item.id else {
        return Vec::new();
    };
    let mut controls = Vec::new();
    match item.type_ {
        Some(
            BaseItemKind::Movie
            | BaseItemKind::Episode
            | BaseItemKind::MusicVideo
            | BaseItemKind::Audio,
        ) => {
            controls.push(intent_button(
                Text::DetailPlay,
                Intent::Item {
                    item: id,
                    resume: false,
                },
            ));
            if resumable(item) {
                controls.push(intent_button(
                    Text::DetailResume,
                    Intent::Item {
                        item: id,
                        resume: true,
                    },
                ));
            }
        }
        Some(
            BaseItemKind::Series
            | BaseItemKind::Season
            | BaseItemKind::MusicAlbum
            | BaseItemKind::MusicArtist,
        ) => {
            controls.push(intent_button(
                Text::DetailPlayAll,
                Intent::All {
                    item: id,
                    shuffle: false,
                },
            ));
            controls.push(intent_button(
                Text::DetailShuffle,
                Intent::All {
                    item: id,
                    shuffle: true,
                },
            ));
        }
        _ => {}
    }

    if matches!(
        item.type_,
        Some(BaseItemKind::Audio | BaseItemKind::MusicAlbum | BaseItemKind::MusicArtist)
    ) {
        controls.push(intent_button(
            Text::DetailInstantMix,
            Intent::Mix { item: id },
        ));
    }

    controls
}

/// `.itemBackdrop`, the item's own backdrop standing at the head of the page,
/// covering the width it is given.
// reference: detail-backdrop
fn backdrop<'a>(item: &BaseItemDto, images: &'a Cache, height: Drawn) -> Element<'a, Message> {
    let height = style::drawn(height);
    let face = item.id.and_then(|id| {
        images.handle(images::Key {
            item: id,
            kind: Kind::Backdrop,
            index: None,
        })
    });
    match face {
        Some(handle) => image(handle)
            .width(Fill)
            .height(height)
            .content_fit(iced::ContentFit::Cover)
            .into(),
        None => Space::new().width(Fill).height(height).into(),
    }
}

/// The item's primary image at the width its arrangement gives it.
// reference: detail-image
fn poster<'a>(item: &BaseItemDto, images: &'a Cache, width: Drawn) -> Element<'a, Message> {
    let width = style::drawn(width);
    let face = item.id.and_then(|id| {
        images.handle(images::Key {
            item: id,
            kind: Kind::Primary,
            index: None,
        })
    });
    match face {
        Some(handle) => container(image(handle).width(width))
            .width(width)
            .style(style::card_padder)
            .into(),
        None => Space::new().width(width).into(),
    }
}

/// A quarter of the page beside the ribbon and three tenths of it over the
/// backdrop, which is the one measure the two arrangements disagree on before
/// they are laid out.
// reference: detail-poster
fn poster_width(viewport: Viewport) -> Drawn {
    match viewport.band() {
        Band::Desktop => space::DETAIL_POSTER,
        Band::Mobile => space::DETAIL_POSTER_STACKED,
    }
    .of(viewport.canvas().width())
}

/// What both arrangements lay out, built once.
struct Head<'a> {
    backdrop: Element<'a, Message>,
    poster: Element<'a, Message>,
    name: Element<'a, Message>,
    lines: Element<'a, Message>,
    buttons: Element<'a, Message>,
}

/// The parts the reference's own primary container holds, drawn once and
/// placed by whichever arrangement the band names.
// reference: detail-markup
// reference: detail-primary
// reference: detail-parent-name
// reference: detail-name-container
// reference: detail-misc
fn head<'a>(
    item: &'a BaseItemDto,
    viewport: Viewport,
    images: &'a Cache,
    session: &'a jellium_protocol::Session,
) -> Head<'a> {
    let mut actions = row![].spacing(style::drawn(space::GUTTER.drawn()));
    for control in play_controls(item) {
        actions = actions.push(control);
    }
    if let Some(id) = item.id {
        let mark = match item::played(item) {
            Mark::Set => Text::DetailMarkUnplayed,
            Mark::Cleared => Text::DetailMarkPlayed,
        };
        let star = match item::favorited(item) {
            Mark::Set => Text::DetailUnfavorite,
            Mark::Cleared => Text::DetailFavorite,
        };
        actions = actions
            .push(
                button(prose(strings::lookup(mark), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::PlayedToggled(id, item::played(item).flipped())),
            )
            .push(
                button(prose(strings::lookup(star), typeface::BODY))
                    .style(style::raised)
                    .on_press(Message::FavoriteToggled(
                        id,
                        item::favorited(item).flipped(),
                    )),
            );

        if session.administrator && !session.read_only {
            actions = actions
                .push(
                    button(prose(
                        strings::lookup(Text::DetailRefreshMetadata),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::RefreshItem {
                        item: id,
                        replace: false,
                        recursive: true,
                    }),
                )
                .push(
                    button(prose(
                        strings::lookup(Text::DetailRefreshReplace),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::RefreshItem {
                        item: id,
                        replace: true,
                        recursive: true,
                    }),
                )
                .push(
                    button(prose(
                        strings::lookup(Text::DetailRefreshScanMode),
                        typeface::BODY,
                    ))
                    .style(style::raised)
                    .on_press(Message::RefreshItem {
                        item: id,
                        replace: false,
                        recursive: false,
                    }),
                );
        }
    }

    let mut lines = column![].spacing(style::drawn(space::GUTTER.drawn()));
    if let Some(overview) = &item.overview {
        lines = lines
            .push(prose(
                strings::lookup(Text::DetailOverview),
                typeface::HEADING_3,
            ))
            .push(prose(overview.clone(), typeface::BODY));
    }

    Head {
        backdrop: backdrop(item, images, space::BACKDROP.of(viewport.canvas().height())),
        poster: poster(item, images, poster_width(viewport)),
        name: column![
            prose(heading(item), typeface::BODY),
            prose(item.name.clone().unwrap_or_default(), typeface::HEADING_1),
        ]
        .into(),
        lines: lines.into(),
        buttons: actions.into(),
    }
}

/// The backdrop, the poster over it, the ribbon carrying the name, the item's
/// own lines and the row of buttons: side by side on the desktop band.
// reference: detail-markup
// reference: detail-ribbon
// reference: detail-poster
// reference: detail-content
fn ribboned<'a>(head: Head<'a>, viewport: Viewport) -> Element<'a, Message> {
    let canvas = viewport.canvas();
    let tall = space::BACKDROP.of(canvas.height());
    let ribbon = space::RIBBON.drawn();
    let rise = space::DETAIL_POSTER_RISE.drawn();
    let lead = style::drawn(space::RIBBON_INSET.of(canvas.width()));
    let trail = style::drawn(space::DETAIL_TRAIL.of(canvas.width()));

    // the reference lifts the ribbon 7.2em over the backdrop and the poster a
    // further 12.96em over the ribbon, and iced takes no negative offset
    // this client stands both over the backdrop instead: the ribbon on its
    // foot, and the poster's own top on what the rise leaves above that foot
    let cleared = Drawn::of((tall.count() - ribbon.count() - rise.count()).max(0.0));

    let banner = stack![
        head.backdrop,
        container(
            row![
                Space::new().width(lead),
                container(head.name).width(Fill),
                head.buttons,
                Space::new().width(trail),
            ]
            .align_y(iced::Center)
            .height(style::drawn(ribbon))
        )
        .height(Fill)
        .align_y(iced::Bottom),
        container(row![
            Space::new().width(style::drawn(space::DETAIL_POSTER_INSET.of(canvas.width()))),
            head.poster,
        ])
        .padding(iced::Padding::ZERO.top(style::drawn(cleared))),
    ];

    column![
        banner,
        container(row![
            Space::new().width(lead),
            container(head.lines).width(Fill),
            Space::new().width(trail),
        ])
        .padding(iced::Padding::ZERO.top(style::drawn(space::DETAIL_BODY_TOP.drawn()))),
    ]
    .into()
}

/// The same parts stacked and centred, which is what the mobile band draws.
// reference: detail-info-wrapper
// reference: detail-centred
// reference: detail-misc-narrow
fn stacked<'a>(head: Head<'a>, viewport: Viewport) -> Element<'a, Message> {
    let side = style::drawn(space::DETAIL_SIDE.of(viewport.canvas().width()));

    let banner = stack![
        head.backdrop,
        container(row![Space::new().width(side), head.poster])
            .height(Fill)
            .align_y(iced::Bottom),
    ];

    column![
        Space::new().height(style::drawn(space::BACKDROP_TOP.drawn())),
        banner,
        container(
            column![head.name, head.buttons]
                .align_x(iced::Center)
                .width(Fill)
        )
        .padding(style::padding(space::RIBBON_PAD).left(side).right(side)),
        container(container(head.lines).center_x(Fill)).padding(
            iced::Padding::ZERO
                .top(style::drawn(space::DETAIL_BODY_TOP_STACKED.drawn()))
                .left(side)
                .right(side)
        ),
    ]
    .into()
}

/// The item drawn in the arrangement its band names, and under it what the
/// server holds beneath the item: its children, its similar rail and — for an
/// administrator alone — the way into its metadata.
pub fn view<'a>(
    state: &'a State,
    viewport: Viewport,
    images: &'a Cache,
    session: &'a jellium_protocol::Session,
) -> Element<'a, Message> {
    let item = &state.item;
    let overflow = match session.read_only {
        true => widget::Overflow::Withheld,
        false => widget::Overflow::Offered,
    };

    let head = head(item, viewport, images, session);
    let drawn = match viewport.band() {
        Band::Desktop => ribboned(head, viewport),
        Band::Mobile => stacked(head, viewport),
    };

    let mut page = column![]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn()));

    if !state.children.is_empty() {
        page = page
            .push(prose(
                strings::lookup(children_heading(item.type_)),
                typeface::HEADING_2,
            ))
            .push(widget::posters(
                card::Card::Wall(card::Shape::Portrait),
                state.children.iter(),
                Room::content(viewport),
                images,
                overflow,
            ));
    }

    if session.administrator
        && !session.read_only
        && let Some(id) = item.id
    {
        page = page.push(
            button(prose(strings::lookup(Text::MetadataOpen), typeface::BODY))
                .style(style::raised)
                .on_press(Message::Navigated(crate::route::Route::Metadata {
                    item: id,
                    part: crate::screen::metadata::Part::Fields,
                })),
        );
    }

    if !state.similar.is_empty() {
        page = page.push(widget::section(
            strings::lookup(Text::DetailSimilar),
            widget::rail(
                card::Card::Rail(card::Rail::Portrait),
                state.similar.iter(),
                Room::content(viewport),
                images,
                overflow,
            ),
        ));
    }

    scrollable(column![drawn, page]).into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut keys = widget::card_images(&state.children);
    keys.extend(widget::card_images(&state.similar));
    if let Some(id) = state.item.id {
        keys.insert(images::Key {
            item: id,
            kind: Kind::Primary,
            index: None,
        });
        keys.insert(images::Key {
            item: id,
            kind: Kind::Backdrop,
            index: None,
        });
    }
    keys
}
