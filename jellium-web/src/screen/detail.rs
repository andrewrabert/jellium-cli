use std::rc::Rc;

use iced::widget::{Space, button, column, container, image, row};
use iced::{Element, Fill};
use jellium_model::item::{self, Mark};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};
use uuid::Uuid;

use crate::api::Api;
use jellium_model::construct::Construct;

use crate::app::Message;
use crate::error::Answer;
use crate::icon::Icon;
use crate::images::{self, Cache, Kind};
use crate::livetv::Program;
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
    // reference: detail-children-cards
    touch: card::Touch::Plays,
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
    // reference: detail-similar-cards
    touch: card::Touch::Plays,
};

#[derive(Debug, Clone)]
pub struct State {
    pub item: BaseItemDto,
    pub children: Vec<BaseItemDto>,
    /// What the server says is like this item.
    pub similar: Vec<BaseItemDto>,
    /// The programme this item is, and nothing where it is not one.
    pub programme: Option<Program>,
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
            programme: match item.type_ {
                Some(BaseItemKind::Program) => Program::read(&item),
                _ => None,
            },
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
        Some(BaseItemKind::BoxSet) => Text::DetailItems,
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
/// Play All and Shuffle for a series, season, album, artist or box set.
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
            | BaseItemKind::MusicArtist
            | BaseItemKind::BoxSet,
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

/// Play alone while the programme is airing, and nothing otherwise.
// reference: detail-program-play
fn programme_play<'a>(
    programme: &'a Program,
    now: chrono::DateTime<chrono::Utc>,
    viewport: Viewport,
) -> Vec<Element<'a, Message>> {
    match programme.airing(now) {
        false => Vec::new(),
        true => vec![detail_button(
            Icon::PlayArrow,
            Text::DetailPlay,
            Message::LiveTvAction(crate::screen::livetv::Action::PlayChannel(
                programme.channel,
            )),
            viewport,
        )],
    }
}

/// Record, Record Series and Cancel as the timers covering the programme
/// allow, offered where the server grants Live TV management.
// reference: detail-recording-fields
fn recording_controls<'a>(programme: &'a Program, viewport: Viewport) -> Vec<Element<'a, Message>> {
    let single = match &programme.timer {
        Some(timer) => detail_button(
            Icon::Cancel,
            Text::ProgramCancelRecording,
            Message::LiveTvAction(crate::screen::livetv::Action::CancelTimer(timer.clone())),
            viewport,
        ),
        None => detail_button(
            Icon::FiberManualRecord,
            Text::ProgramRecord,
            Message::LiveTvAction(crate::screen::livetv::Action::Record(programme.item)),
            viewport,
        ),
    };
    let series = match &programme.series_timer {
        Some(timer) => detail_button(
            Icon::Cancel,
            Text::ProgramCancelSeries,
            Message::LiveTvAction(crate::screen::livetv::Action::CancelSeriesTimer(
                timer.clone(),
            )),
            viewport,
        ),
        None => detail_button(
            Icon::FiberSmartRecord,
            Text::ProgramRecordSeries,
            Message::LiveTvAction(crate::screen::livetv::Action::RecordSeries(programme.item)),
            viewport,
        ),
    };
    vec![single, series]
}

/// The programme's live, new, premiere and repeat flags, each as its own word.
fn flags<'a>(programme: &'a Program) -> Vec<Element<'a, Message>> {
    [
        (programme.live, Text::GuideBadgeLive),
        (programme.new, Text::GuideBadgeNew),
        (programme.premiere, Text::GuideBadgePremiere),
        (programme.repeat, Text::GuideBadgeRepeat),
    ]
    .into_iter()
    .filter(|(carried, _)| *carried)
    .map(|(_, label)| prose(strings::lookup(label), typeface::SECONDARY))
    .collect()
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

/// The item's primary image at the width its arrangement gives it, held to the
/// height the page caps it to and covering that box.
// reference: detail-image
// reference: detail-poster-arms
fn poster<'a>(item: &BaseItemDto, images: &'a Cache, viewport: Viewport) -> Element<'a, Message> {
    let width = poster_width(viewport);
    let height = space::detail_poster_height(viewport, card::Shape::Portrait.aspect().of(width));
    let width = style::drawn(width);
    let height = style::drawn(height);
    let face = item.id.and_then(|id| {
        images.handle(images::Key {
            item: id,
            kind: Kind::Primary,
            index: None,
            card: card::Card::Wall(card::Shape::Portrait),
        })
    });
    match face {
        Some(handle) => container(
            image(handle)
                .width(width)
                .height(height)
                .content_fit(iced::ContentFit::Cover),
        )
        .width(width)
        .style(|theme| style::card_padder(theme, card::Backing::Padder))
        .into(),
        None => Space::new().width(width).height(height).into(),
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
/// placed by whichever arrangement the layout names: what plays, the played
/// mark where the item can carry one, the rating control where the item can
/// carry one, and the control that opens the item's own menu.
// reference: detail-markup
// reference: detail-primary
// reference: detail-parent-name
// reference: detail-name-container
// reference: detail-misc
// reference: detail-markup-marks
// reference: detail-more-commands
// reference: item-can-mark-played
// reference: item-can-rate
// reference: detail-program-play
// reference: detail-recording-fields
fn head<'a>(
    item: &'a BaseItemDto,
    programme: Option<&'a Program>,
    session: &'a jellium_protocol::Session,
    viewport: Viewport,
    images: &'a Cache,
    now: chrono::DateTime<chrono::Utc>,
) -> Head<'a> {
    let mut actions = row![].spacing(style::drawn(space::CONTROL_GAP.drawn()));
    for control in play_controls(item, viewport) {
        actions = actions.push(control);
    }
    if let Some(programme) = programme {
        for control in programme_play(programme, now, viewport) {
            actions = actions.push(control);
        }
        if crate::screen::overflow::manageable(session) {
            for control in recording_controls(programme, viewport) {
                actions = actions.push(control);
            }
        }
    }
    if let Some(id) = item.id {
        if item::markable(item) && !session.read_only {
            actions = actions.push(detail_button(
                Icon::Check,
                match item::played(item) {
                    Mark::Set => Text::DetailMarkUnplayed,
                    Mark::Cleared => Text::DetailMarkPlayed,
                },
                Message::PlayedToggled(id, item::played(item).flipped()),
                viewport,
            ));
        }
        if item::ratable(item) && !session.read_only {
            actions = actions.push(detail_button(
                Icon::Favorite,
                match item::favorited(item) {
                    Mark::Set => Text::DetailUnfavorite,
                    Mark::Cleared => Text::DetailFavorite,
                },
                Message::FavoriteToggled(id, item::favorited(item).flipped()),
                viewport,
            ));
        }
        let offered = crate::screen::overflow::commands(
            crate::screen::overflow::Subject::Item(item),
            session,
            None,
            now,
        );
        if !offered.is_empty() {
            actions = actions.push(detail_button(
                Icon::MoreVert,
                Text::DetailMoreCommands,
                Message::OverflowAction(crate::screen::overflow::Action::Open { offered }),
                viewport,
            ));
        }
    }

    let mut lines = column![].spacing(style::drawn(space::SECTION_GAP.drawn()));
    if let Some(programme) = programme {
        lines = lines.push(prose(crate::livetv::airtime(programme), typeface::BODY));
        let mut marks = row![].spacing(style::drawn(space::CONTROL_GAP.drawn()));
        for flag in flags(programme) {
            marks = marks.push(flag);
        }
        lines = lines.push(marks);
    }
    if let Some(overview) = &item.overview {
        lines = lines
            .push(prose(
                strings::lookup(Text::DetailOverview),
                typeface::HEADING_3,
            ))
            .push(prose(overview.clone(), typeface::BODY));
    }
    if let Some(programme) = programme.filter(|held| !held.genres.is_empty()) {
        lines = lines.push(prose(programme.genres.join(", "), typeface::BODY));
    }

    Head {
        backdrop: backdrop(item, images, space::backdrop(viewport)),
        poster: poster(item, images, viewport),
        name: column![
            prose(
                match programme {
                    Some(programme) => strings::format(
                        Text::ProgramChannel,
                        &[&programme.channel_name, &programme.channel_number],
                    ),
                    None => heading(item),
                },
                typeface::BODY
            ),
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

/// The same parts stacked and centred, which is what the mobile layout draws,
/// the poster pinned by its own foot inside the ribbon and the name and the
/// buttons standing clear of it.
// reference: detail-info-wrapper
// reference: detail-centred
// reference: detail-misc-narrow
// reference: detail-poster-arms
fn stacked<'a>(head: Head<'a>, viewport: Viewport) -> Element<'a, Message> {
    let canvas = viewport.canvas();
    let side = style::drawn(space::page_side(canvas));
    let body = style::drawn(space::DETAIL_SIDE.of(canvas.width()));

    let ribbon = column![
        Space::new().height(style::drawn(space::BACKDROP_TOP.drawn())),
        head.backdrop,
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
        .padding(style::padding(space::RIBBON_PAD).left(side).right(side)),
    ];

    let poster = row![
        Space::new().width(style::drawn(space::detail_poster_stacked_inset(viewport))),
        head.poster,
    ];

    column![
        Overlapping::lifted(
            ribbon.into(),
            poster.into(),
            space::detail_poster_lift(viewport),
        ),
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
    now: chrono::DateTime<chrono::Utc>,
    session: &'a jellium_protocol::Session,
) -> Element<'a, Message> {
    let item = &state.item;

    let head = head(
        item,
        state.programme.as_ref(),
        session,
        viewport,
        images,
        now,
    );
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
                now,
                session,
                item.id.filter(|_| item.type_ == Some(BaseItemKind::BoxSet)),
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
                    item: Some(id),
                    part: None,
                })),
        );
    }

    if !state.similar.is_empty() {
        page = page.push(widget::section(
            widget::prose(strings::lookup(Text::DetailSimilar), typeface::HEADING_2),
            widget::rail(
                ALIKE,
                widget::Rail::of(Construct::ItemsContainer),
                state.similar.iter(),
                Room::content(viewport),
                images,
                now,
                session,
            ),
        ));
    }

    crate::widget::scrolled(column![drawn, page]).into()
}

pub fn images(state: &State) -> images::Wanted {
    let mut keys = widget::card_images(&state.children, CHILDREN.card);
    keys.extend(widget::card_images(&state.similar, ALIKE.card));
    if let Some(id) = state.item.id {
        keys.want(images::Poster::of(images::Key {
            item: id,
            kind: Kind::Primary,
            index: None,
            card: card::Card::Wall(card::Shape::Portrait),
        }));
        keys.want(images::Poster::of(images::Key {
            item: id,
            kind: Kind::Backdrop,
            index: None,
            card: card::Card::Wall(card::Shape::Backdrop),
        }));
    }
    keys
}
