use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{Space, button, column, container, image, row, stack};
use iced::{Element, Fill};
use jellium_model::item::{self, Mark};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use uuid::Uuid;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::images::{self, Cache, Kind};
use crate::player::Intent;
use crate::style::space::Room;
use crate::style::{self, Drawn, Layout, Viewport, card, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget;
use crate::widget::overlap::Overlapping;
use crate::widget::prose;

/// The card a detail page's children draw on, over the two lines a wall writes
/// under one.
// reference: card-box-classes
const CHILDREN: card::Drawing = card::Drawing {
    card: card::Card::Wall(card::Shape::Portrait),
    footer: card::Footer::NameAndSubtitle,
    backing: card::Backing::Padder,
    footing: card::Footing::Bare,
    setting: card::Setting::Centred,
    bottom: card::Bottom::Padded,
};

/// The card the rail of items alike draws on.
// reference: card-box-classes
const ALIKE: card::Drawing = card::Drawing {
    card: card::Card::Rail(card::Rail::Portrait),
    footer: card::Footer::NameAndSubtitle,
    backing: card::Backing::Padder,
    footing: card::Footing::Bare,
    setting: card::Setting::Centred,
    bottom: card::Bottom::Padded,
};

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

/// One button in the row under the name: a glyph over nothing, at the padding
/// the page's width writes.
// reference: detail-button
// reference: detail-button-icon
// reference: detail-markup-play
fn detail_button<'a>(
    glyph: Icon,
    label: Text,
    press: Message,
    viewport: Viewport,
) -> Element<'a, Message> {
    let side = style::drawn(space::detail_button_side(viewport));
    let control = button(crate::icon::icon(glyph, typeface::DETAIL_ICON))
        .style(style::flat)
        .padding(
            style::padding(space::DETAIL_BUTTON_PAD)
                .left(side)
                .right(side),
        )
        .on_press(press);
    iced::widget::tooltip(
        control,
        prose(strings::lookup(label), typeface::BODY),
        iced::widget::tooltip::Position::Top,
    )
    .style(style::dialog)
    .into()
}

/// Play, and Resume when the item has a stored position and is not marked
/// played, for a movie, episode, music video or song.
/// Play All and Shuffle for a series, season, album or artist.
/// Instant Mix for a song, album or artist.
fn play_controls<'a>(item: &BaseItemDto, viewport: Viewport) -> Vec<Element<'a, Message>> {
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
            controls.push(detail_button(
                Icon::PlayArrow,
                Text::DetailPlay,
                Message::PlayPressed(Intent::Item {
                    item: id,
                    resume: false,
                }),
                viewport,
            ));
            if resumable(item) {
                controls.push(detail_button(
                    Icon::Replay,
                    Text::DetailResume,
                    Message::PlayPressed(Intent::Item {
                        item: id,
                        resume: true,
                    }),
                    viewport,
                ));
            }
        }
        Some(
            BaseItemKind::Series
            | BaseItemKind::Season
            | BaseItemKind::MusicAlbum
            | BaseItemKind::MusicArtist,
        ) => {
            controls.push(detail_button(
                Icon::PlayArrow,
                Text::DetailPlayAll,
                Message::PlayPressed(Intent::All {
                    item: id,
                    shuffle: false,
                }),
                viewport,
            ));
            controls.push(detail_button(
                Icon::Shuffle,
                Text::DetailShuffle,
                Message::PlayPressed(Intent::All {
                    item: id,
                    shuffle: true,
                }),
                viewport,
            ));
        }
        _ => {}
    }

    if matches!(
        item.type_,
        Some(BaseItemKind::Audio | BaseItemKind::MusicAlbum | BaseItemKind::MusicArtist)
    ) {
        controls.push(detail_button(
            Icon::Explore,
            Text::DetailInstantMix,
            Message::PlayPressed(Intent::Mix { item: id }),
            viewport,
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
            card: card::Card::Wall(card::Shape::Backdrop),
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
            card: card::Card::Wall(card::Shape::Portrait),
        })
    });
    match face {
        Some(handle) => container(image(handle).width(width))
            .width(width)
            .style(|theme| style::card_padder(theme, card::Backing::Padder))
            .into(),
        None => Space::new().width(width).into(),
    }
}

/// A quarter of the page beside the ribbon and beside the televised poster,
/// and three tenths of it over the backdrop.
// reference: detail-poster-arms
fn poster_width(viewport: Viewport) -> Drawn {
    match viewport.layout() {
        Layout::Desktop | Layout::Television => space::DETAIL_POSTER,
        Layout::Mobile => space::DETAIL_POSTER_STACKED,
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
/// placed by whichever arrangement the layout names.
// reference: detail-markup
// reference: detail-primary
// reference: detail-parent-name
// reference: detail-name-container
// reference: detail-misc
fn head<'a>(item: &'a BaseItemDto, viewport: Viewport, images: &'a Cache) -> Head<'a> {
    let mut actions = row![].spacing(style::drawn(space::CONTROL_GAP.drawn()));
    for control in play_controls(item, viewport) {
        actions = actions.push(control);
    }
    // reference: detail-markup-marks
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
            .push(detail_button(
                Icon::Check,
                mark,
                Message::PlayedToggled(id, item::played(item).flipped()),
                viewport,
            ))
            .push(detail_button(
                Icon::Favorite,
                star,
                Message::FavoriteToggled(id, item::favorited(item).flipped()),
                viewport,
            ));
    }

    let mut lines = column![].spacing(style::drawn(space::SECTION_GAP.drawn()));
    if let Some(overview) = &item.overview {
        lines = lines
            .push(prose(
                strings::lookup(Text::DetailOverview),
                typeface::HEADING_3,
            ))
            .push(prose(overview.clone(), typeface::BODY));
    }

    Head {
        backdrop: backdrop(item, images, space::backdrop(viewport)),
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
/// own lines and the row of buttons: side by side on the desktop layout.
// reference: detail-markup
// reference: detail-ribbon
// reference: detail-poster
// reference: detail-content
fn ribboned<'a>(head: Head<'a>, viewport: Viewport) -> Element<'a, Message> {
    let canvas = viewport.canvas();
    let lead = style::drawn(space::RIBBON_INSET.of(canvas.width()));
    let trail = style::drawn(space::DETAIL_TRAIL.of(canvas.width()));
    let buttons = style::drawn(space::DETAIL_BUTTONS.drawn());

    let ribbon = container(
        row![
            Space::new().width(lead),
            container(head.name).width(Fill),
            container(head.buttons).padding(iced::Padding::ZERO.top(buttons).bottom(buttons)),
            Space::new().width(trail),
        ]
        .align_y(iced::Center)
        .height(style::drawn(space::RIBBON.drawn())),
    );

    let poster = row![
        Space::new().width(style::drawn(space::DETAIL_POSTER_INSET.of(canvas.width()))),
        head.poster,
    ];

    column![
        Overlapping::new(
            Overlapping::new(head.backdrop, ribbon.into(), space::RIBBON_OVERLAP).into(),
            poster.into(),
            space::detail_poster_overlap(viewport),
        ),
        container(row![
            Space::new().width(lead),
            container(head.lines).width(Fill),
            Space::new().width(trail),
        ])
        .padding(iced::Padding::ZERO.top(style::drawn(space::DETAIL_BODY_TOP.drawn()))),
    ]
    .into()
}

/// The same parts stacked and centred, which is what the mobile layout draws.
// reference: detail-info-wrapper
// reference: detail-centred
// reference: detail-misc-narrow
fn stacked<'a>(head: Head<'a>, viewport: Viewport) -> Element<'a, Message> {
    let canvas = viewport.canvas();
    let ribbon = style::drawn(space::page_side(canvas));
    let body = style::drawn(space::DETAIL_SIDE.of(canvas.width()));

    let banner = stack![
        head.backdrop,
        container(row![
            Space::new().width(style::drawn(space::detail_poster_stacked_inset(viewport))),
            head.poster,
        ])
        .height(Fill)
        .align_y(iced::Bottom),
    ];

    column![
        Space::new().height(style::drawn(space::BACKDROP_TOP.drawn())),
        banner,
        container(column![
            container(head.name)
                .width(Fill)
                .align_x(iced::Center)
                .padding(
                    iced::Padding::ZERO.left(style::drawn(space::detail_head_inset(viewport)))
                ),
            container(head.buttons)
                .width(Fill)
                .align_x(iced::Center)
                .padding(
                    iced::Padding::ZERO
                        .top(style::drawn(space::DETAIL_BUTTONS.drawn()))
                        .bottom(style::drawn(space::detail_buttons_bottom(viewport)))
                        .left(style::drawn(space::detail_buttons_inset(viewport)))
                ),
        ])
        .padding(style::padding(space::RIBBON_PAD).left(ribbon).right(ribbon)),
        container(container(head.lines).center_x(Fill)).padding(
            iced::Padding::ZERO
                .top(style::drawn(space::DETAIL_BODY_TOP_STACKED.drawn()))
                .left(body)
                .right(body)
        ),
    ]
    .into()
}

/// The parts with no backdrop, the poster standing at the page's own leading
/// edge, and the ribbon and the body padded to clear it.
// reference: detail-backdrop
// reference: detail-ribbon
// reference: detail-poster-arms
// reference: detail-content
fn televised<'a>(head: Head<'a>, viewport: Viewport) -> Element<'a, Message> {
    let canvas = viewport.canvas();
    let edge = style::drawn(space::DETAIL_POSTER_TELEVISED.of(canvas.width()));
    let beside = style::drawn(space::RIBBON_INSET.of(canvas.width()));
    let trail = style::drawn(space::DETAIL_TRAIL.of(canvas.width()));
    let buttons = style::drawn(space::DETAIL_BUTTONS.drawn());

    row![
        container(row![Space::new().width(edge), head.poster]).width(beside),
        column![
            head.name,
            container(head.buttons).padding(iced::Padding::ZERO.top(buttons).bottom(buttons)),
            container(head.lines)
                .padding(iced::Padding::ZERO.top(style::drawn(space::DETAIL_BODY_TOP.drawn()))),
        ]
        .width(Fill),
        Space::new().width(trail),
    ]
    .into()
}

/// The item drawn in the arrangement its layout names, and under it what the
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

    let head = head(item, viewport, images);
    let drawn = match viewport.layout() {
        Layout::Desktop => ribboned(head, viewport),
        Layout::Mobile => stacked(head, viewport),
        Layout::Television => televised(head, viewport),
    };

    let mut page = column![]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .padding(style::padding(space::PAGE_PAD));

    if !state.children.is_empty() {
        page = page
            .push(prose(
                strings::lookup(children_heading(item.type_)),
                typeface::HEADING_2,
            ))
            .push(widget::posters(
                CHILDREN,
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
                    part: None,
                })),
        );
    }

    if !state.similar.is_empty() {
        page = page.push(widget::section(
            strings::lookup(Text::DetailSimilar),
            widget::rail(
                ALIKE,
                state.similar.iter(),
                Room::content(viewport),
                images,
                overflow,
            ),
        ));
    }

    crate::widget::scrolled(column![drawn, page]).into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let mut keys = widget::card_images(&state.children, CHILDREN.card);
    keys.extend(widget::card_images(&state.similar, ALIKE.card));
    if let Some(id) = state.item.id {
        keys.insert(images::Key {
            item: id,
            kind: Kind::Primary,
            index: None,
            card: card::Card::Wall(card::Shape::Portrait),
        });
        keys.insert(images::Key {
            item: id,
            kind: Kind::Backdrop,
            index: None,
            card: card::Card::Wall(card::Shape::Backdrop),
        });
    }
    keys
}
