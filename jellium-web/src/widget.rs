mod line;
pub mod list;
pub mod mui;
pub mod overlap;
pub mod sheet;
pub mod table;

use std::borrow::Cow;
use std::collections::HashSet;

use iced::widget::{
    Space, button, column, container, grid, image, rich_text, row, scrollable, span, stack, text,
};
use iced::{Element, Fill, Length};
use jellium_model::item::{self, Mark};
use jellium_protocol::{Session, SyncAccess};
use jellyfin_api::types::{BaseItemDto, BaseItemKind};

use crate::app::Message;
use crate::icon::Icon;
use crate::images::{self, Cache, Kind};
use crate::live;
use crate::livetv::{Channel, Program, Recording};
use crate::player::group::Joined;
use crate::route::Route;
use crate::style::space::Room;
use crate::style::{self, Drawn, Layout, Share, Viewport, card, scheme, space, typeface};
use crate::text::{self as strings, Text};

/// Wrapping text, which is what a server's own disclaimer is. Every string the
/// client draws passes here, so the coverage it needs is observed once. A
/// sentence out of the string table arrives borrowed and is drawn as it lies.
pub fn prose<'a>(content: impl Into<Cow<'a, str>>, size: style::Length) -> Element<'a, Message> {
    tinted(
        content,
        size,
        typeface::Weight::Regular,
        typeface::LINE_HEIGHT,
        iced::widget::text::default,
    )
}

/// Wrapping text in one of the scheme's own colors, in the face and the line
/// box it is set in, which is what a field's label, its description and a tab's
/// own lettering are drawn in.
fn tinted<'a>(
    content: impl Into<Cow<'a, str>>,
    size: style::Length,
    weight: typeface::Weight,
    leading: typeface::Leading,
    color: fn(&iced::Theme) -> iced::widget::text::Style,
) -> Element<'a, Message> {
    let content = content.into();
    crate::fonts::observed(&content, weight);
    text(content)
        .size(style::drawn(size.drawn()))
        .font(style::font(weight))
        .line_height(style::leading(leading))
        .style(color)
        .into()
}

/// One line of text, cut with an ellipsis at the width it is given, which is
/// what the reference does to a list row's title and its secondary line.
pub fn line<'a>(
    content: impl Into<Cow<'a, str>>,
    size: style::Length,
    weight: typeface::Weight,
    leading: typeface::Leading,
) -> Element<'a, Message> {
    let content = content.into();
    crate::fonts::observed(&content, weight);
    line::Line::new(content.into_owned(), size, weight, leading).into()
}

/// One option a control offers: what it reads as, and the value choosing it
/// carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Choice<T> {
    pub label: String,
    pub value: T,
}

impl<T> std::fmt::Display for Choice<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.label)
    }
}

// the key `getCardImageUrl` resolves for an item drawn on `card`, in the order
// the reference tests its branches, each answering the item its tag belongs to
// and the item's own id where the reference names none
// a banner card takes the item's own Banner tag
// the item's own Primary tag, unless it is an episode reporting no children
// the series' Primary tag, under the series' id
// the parent's Primary tag, under the parent's id
// the album's Primary tag, under the album's id
// a season's own Thumb tag
// the item's first Backdrop tag
// the item's own Thumb tag
// the series' Thumb tag, under the series' id
// the parent's Thumb tag, under the parent's id
// the parent's first Backdrop tag, under the parent's id
// `None` where no branch answers a tag, and where the item carries no id
// reference: card-image-banner
// reference: card-image-primary
// reference: card-image-inherited
pub fn poster_key(item: &BaseItemDto, card: card::Card) -> Option<images::Key> {
    let held = |kind: Kind| {
        item.image_tags
            .as_ref()
            .is_some_and(|tags| tags.contains_key(kind.as_str()))
    };
    let any = |tags: &Option<Vec<String>>| tags.as_ref().is_some_and(|tags| !tags.is_empty());
    let childless = item.type_ == Some(BaseItemKind::Episode) && item.child_count == Some(0);
    let (kind, at) = if card.shape() == card::Shape::Banner && held(Kind::Banner) {
        (Kind::Banner, None)
    } else if held(Kind::Primary) && !childless {
        (Kind::Primary, None)
    } else if item.series_primary_image_tag.is_some() {
        (Kind::Primary, item.series_id)
    } else if item.parent_primary_image_tag.is_some() {
        (Kind::Primary, item.parent_primary_image_item_id)
    } else if item.album_id.is_some() && item.album_primary_image_tag.is_some() {
        (Kind::Primary, item.album_id)
    } else if item.type_ == Some(BaseItemKind::Season) && held(Kind::Thumb) {
        (Kind::Thumb, None)
    } else if any(&item.backdrop_image_tags) {
        (Kind::Backdrop, None)
    } else if held(Kind::Thumb) {
        (Kind::Thumb, None)
    } else if item.series_thumb_image_tag.is_some() {
        (Kind::Thumb, item.series_id)
    } else if let Some(parent) = item.parent_thumb_item_id {
        // the reference stops here on the parent's id alone: an item naming one
        // and carrying no parent thumb tag draws nothing, rather than falling
        // to the parent's backdrop below
        item.parent_thumb_image_tag.as_ref()?;
        (Kind::Thumb, Some(parent))
    } else if any(&item.parent_backdrop_image_tags) {
        (Kind::Backdrop, item.parent_backdrop_item_id)
    } else {
        return None;
    };
    Some(images::Key {
        item: at.or(item.id)?,
        kind,
        index: None,
        card,
    })
}

pub fn card_images<'a>(
    items: impl IntoIterator<Item = &'a BaseItemDto>,
    card: card::Card,
) -> HashSet<images::Key> {
    items
        .into_iter()
        .filter_map(|item| poster_key(item, card))
        .collect()
}

/// The route a card opens: a collection and a playlist each have their own
/// screen, and every other item opens its detail.
fn opens(item: &BaseItemDto, id: uuid::Uuid) -> Route {
    match item.type_ {
        Some(BaseItemKind::BoxSet) => Route::Collection {
            id,
            listing: Box::default(),
        },
        Some(BaseItemKind::Playlist) => Route::Playlist { id },
        _ => Route::Detail { id },
    }
}

fn subtitle(item: &BaseItemDto) -> String {
    match (&item.series_name, item.production_year) {
        (Some(series), _) => series.clone(),
        (None, Some(year)) => year.to_string(),
        (None, None) => String::new(),
    }
}

/// Whether a card's hover menu offers the controls that write to the Jellyfin
/// server — the played mark, the rating control and the overflow menu — which a
/// read-only session offers none of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Writes {
    Offered,
    Withheld,
}

/// What a card draws where its image goes: the image itself, or the glyph the
/// reference's own `.cardImageIcon` stands in with.
#[derive(Debug, Clone)]
pub enum Face {
    Image(image::Handle),
    Icon(Icon),
}

/// What a card stands where its image goes: the image the cache holds, and the
/// glyph `getDefaultText` stands in with where the item names no image at all
/// — its collection's glyph where it carries a collection type or is a
/// collection folder, and its own type's glyph otherwise.
/// `None` while the image is in flight and where the item's type names no
/// glyph.
// reference: card-no-image
// reference: card-default-glyph
// reference: library-icon
// reference: library-icon-unknown
// reference: item-type-icon
fn faced(item: &BaseItemDto, card: card::Card, images: &Cache) -> Option<Face> {
    if let Some(key) = poster_key(item, card) {
        return images.handle(key).map(Face::Image);
    }
    let collected =
        item.type_ == Some(BaseItemKind::CollectionFolder) || item.collection_type.is_some();
    collected
        .then(|| Icon::library(item.collection_type))
        .or_else(|| Icon::of(item.type_))
        .map(Face::Icon)
}

/// The frame a card's image stands in, at the card's width inside its pitch
/// and its shape's own aspect: the image, the glyph `.cardImageIcon` stands in
/// with over the background the name picks, or that background alone while the
/// image is in flight and where nothing names a glyph.
// reference: card-container
// reference: card-content
fn framed<'a>(
    card: card::Card,
    room: Room,
    face: Option<Face>,
    name: &str,
    backing: card::Backing,
) -> Element<'a, Message> {
    let width = style::drawn(card.inside(room));
    let height = style::drawn(card.shape().aspect().of(card.inside(room)));
    let background = scheme::card_background(name);
    let painted = move |theme: &iced::Theme| style::card_face(theme, background, backing);
    let padder = move |theme: &iced::Theme| style::card_padder(theme, backing);
    match face {
        Some(Face::Image(handle)) => container(image(handle).width(width))
            .width(width)
            .height(height)
            .style(padder)
            .into(),
        Some(Face::Icon(drawn)) => container(crate::icon::icon(drawn, typeface::CARD_ICON))
            .center_x(width)
            .center_y(height)
            .style(painted)
            .into(),
        None => container(Space::new())
            .width(width)
            .height(height)
            .style(painted)
            .into(),
    }
}

/// One `.cardText` line, in the padding the reference writes in the em of the
/// size that line is set in, over the top the line's own place in the footer
/// gives it.
// reference: card-text
// reference: card-text-first
fn carded<'a>(
    written: Element<'a, Message>,
    size: style::Length,
    top: style::Length,
) -> Element<'a, Message> {
    container(written)
        .padding(style::padding(space::card_text(size)).top(style::drawn(top.drawn())))
        .into()
}

/// `.cardFooter`: the lines a card writes under its image, set where `setting`
/// sets them, inside the footer's own padding where it stands in it, with the
/// channel logo at its leading edge and `trailing` on its trailing edge at the
/// drop the reference gives it.
// reference: card-footer
// reference: card-footer-logo
// reference: card-footer-logo-face
// reference: card-text-centered
fn footed<'a>(
    lines: Vec<Element<'a, Message>>,
    setting: card::Setting,
    footing: card::Footing,
    logo: Option<image::Handle>,
    trailing: Option<Element<'a, Message>>,
    layout: Layout,
) -> Element<'a, Message> {
    let written = column(lines).width(Fill).align_x(match setting {
        card::Setting::Centred => iced::alignment::Horizontal::Center,
        card::Setting::Leading => iced::alignment::Horizontal::Left,
    });
    let held: Element<'a, Message> = match trailing {
        None => written.into(),
        Some(control) => row![
            written,
            container(control).padding(
                iced::Padding::ZERO.top(style::drawn(space::USER_CARD_MENU_TOP.drawn(layout)))
            ),
        ]
        .into(),
    };
    let held: Element<'a, Message> = match logo {
        None => held,
        Some(handle) => iced::widget::stack![
            container(held).padding(
                iced::Padding::ZERO.left(style::drawn(space::CARD_FOOTER_LOGO_INSET.drawn()))
            ),
            container(image(handle).width(style::drawn(
                space::CARD_FOOTER_LOGO_IMAGE.of(space::CARD_FOOTER_LOGO.drawn())
            )))
            .center_x(style::drawn(space::CARD_FOOTER_LOGO.drawn()))
            .center_y(Fill),
        ]
        .into(),
    };
    container(held)
        .padding(match footing {
            card::Footing::Padded => style::padding(space::CARD_FOOTER_PAD),
            card::Footing::Bare => iced::Padding::ZERO,
        })
        .width(Fill)
        .style(style::card_footer)
        .into()
}

/// A card's frame over its footer, held to the card's own width and standing
/// on the backing its box carries.
// reference: card-box
// reference: card-visual
fn boxed<'a>(
    card: card::Card,
    room: Room,
    framed: Element<'a, Message>,
    footer: Option<Element<'a, Message>>,
    backing: card::Backing,
) -> Element<'a, Message> {
    let width = style::drawn(card.inside(room));
    let held = match footer {
        None => column![framed],
        Some(footer) => column![framed, footer],
    }
    .width(width);
    match backing {
        card::Backing::Padder => held.into(),
        card::Backing::Paper => container(held).width(width).style(style::card_paper).into(),
    }
}

/// A card's own name, as its footer's first `.cardText` line writes it.
// reference: card-text
fn named<'a>(name: String) -> Element<'a, Message> {
    carded(
        line(
            name,
            typeface::BODY,
            typeface::Weight::Regular,
            typeface::LINE_HEIGHT,
        ),
        typeface::BODY,
        space::CARD_TEXT_FIRST_TOP,
    )
}

/// One line under a card's first, in the secondary lettering `.cardText-secondary`
/// writes it in.
// reference: card-text-lines
fn beneath<'a>(said: String) -> Element<'a, Message> {
    carded(
        tinted(
            said,
            typeface::SECONDARY,
            typeface::Weight::Regular,
            typeface::LINE_HEIGHT,
            style::description,
        ),
        typeface::SECONDARY,
        space::card_text(typeface::SECONDARY).top,
    )
}

/// The margin `.cardBox-bottompadded` reserves under a card, which the
/// reference sets by the section the card stands in.
// reference: card-box-bottom
fn reserving<'a>(
    body: Element<'a, Message>,
    room: Room,
    bottom: card::Bottom,
) -> Element<'a, Message> {
    match bottom {
        card::Bottom::Flush => body,
        card::Bottom::Padded => {
            let reserved = match room.viewport().matches(space::CARD_BOTTOM_AT) {
                true => space::CARD_BOTTOM_NARROW,
                false => space::CARD_BOTTOM,
            };
            column![body, Space::new().height(style::drawn(reserved.drawn()))].into()
        }
    }
}

/// An image at a card's pitch with its shape's aspect and nothing under it.
// reference: card-container
pub fn tile<'a>(card: card::Card, room: Room, face: Option<image::Handle>) -> Element<'a, Message> {
    let width = style::drawn(card.inside(room));
    let height = style::drawn(card.shape().aspect().of(card.inside(room)));
    match face {
        Some(handle) => container(image(handle).width(width))
            .width(width)
            .height(height)
            .style(|theme| style::card_padder(theme, card::Backing::Padder))
            .into(),
        None => Space::new().width(width).height(height).into(),
    }
}

/// One control the hover menu carries: the glyph it draws, the face it draws
/// that glyph in, the title it names itself by, and what pressing it sends.
#[derive(Debug, Clone)]
pub struct Control {
    pub glyph: Icon,
    pub tint: style::Tint,
    pub label: Text,
    pub press: Message,
}

/// `.cardOverlayContainer`: what the reference lays over a card's image while
/// the pointer is over it, the control that plays at its middle and the rest
/// in `.cardOverlayButton-br`'s group at its trailing foot.
#[derive(Debug, Clone, Default)]
pub struct Hovered {
    pub plays: Option<Message>,
    pub controls: Vec<Control>,
}

/// What one card carries: what its frame stands, the name its background is
/// picked from and whose glyphs it draws, the channel logo its footer carries,
/// the timer its indicators mark, how far through its own program it is, what
/// pressing it opens, and its hover menu.
#[derive(Debug, Clone)]
pub struct Poster {
    pub face: Option<Face>,
    pub name: String,
    pub logo: Option<image::Handle>,
    pub timer: Option<Recording>,
    /// What `.innerCardFooter`'s bar reads across the foot of the card's
    /// image, and `None` where the card marks no progress.
    pub elapsed: Option<Share>,
    pub press: Option<Message>,
    pub hovered: Hovered,
}

/// The glyph of the timer covering an item: a single timer's in the
/// reference's own red, and a series timer's in the lettering faded.
// reference: indicator-timer
// reference: indicator-timer-face
pub fn timer<'a>(recording: Recording, size: style::Length) -> Element<'a, Message> {
    match recording {
        Recording::Once => crate::icon::tinted(Icon::FiberManualRecord, size, style::timer),
        Recording::Series => crate::icon::tinted(Icon::FiberSmartRecord, size, style::series_timer),
    }
}

/// `.cardIndicators`: the timer's glyph laid on the top trailing corner of a
/// card's image, and nothing where no timer covers the item.
// reference: card-indicators
fn marked<'a>(frame: Element<'a, Message>, recording: Option<Recording>) -> Element<'a, Message> {
    let Some(recording) = recording else {
        return frame;
    };
    let inset = style::drawn(space::CARD_INDICATORS_INSET.drawn());
    iced::widget::stack![
        frame,
        container(timer(recording, typeface::INDICATOR_ICON))
            .padding(iced::Padding::ZERO.top(inset).right(inset))
            .align_right(Fill),
    ]
    .into()
}

/// `.innerCardFooter.fullInnerCardFooter.innerCardFooterClear`: the elapsed bar
/// laid across the foot of a card's image, and the frame alone where the card
/// marks no progress.
// reference: card-inner-footer
// reference: progress-bar
fn progressed<'a>(frame: Element<'a, Message>, elapsed: Option<Share>) -> Element<'a, Message> {
    let Some(elapsed) = elapsed else {
        return frame;
    };
    iced::widget::stack![frame, container(elapsed_bar(elapsed)).align_bottom(Fill)].into()
}

/// The scrim the reference raises over a card's image while the pointer is
/// over it: the fab that plays at its middle, and the card's own controls in
/// one group at its trailing foot.
// reference: card-hover-menu
// reference: card-hover-menu-desktop
// reference: card-overlay-shown
// reference: card-overlay-fab
// reference: card-overlay-button
// reference: card-overlay-button-icon
fn hovered<'a>(
    frame: Element<'a, Message>,
    hovered: Hovered,
    backing: card::Backing,
) -> Element<'a, Message> {
    if hovered.plays.is_none() && hovered.controls.is_empty() {
        return frame;
    }
    let mut layers: Vec<Element<'a, Message>> = vec![
        container(Space::new())
            .width(Fill)
            .height(Fill)
            .style(move |theme: &iced::Theme| style::card_overlay(theme, backing))
            .into(),
    ];
    if let Some(play) = hovered.plays {
        let disc = style::drawn(space::CARD_OVERLAY_FAB.drawn());
        layers.push(
            container(
                iced::widget::tooltip(
                    button(
                        container(crate::icon::icon(
                            Icon::PlayArrow,
                            typeface::CARD_OVERLAY_ICON,
                        ))
                        .center_x(disc)
                        .center_y(disc),
                    )
                    .style(style::card_overlay_fab)
                    .padding(style::drawn(Drawn::ZERO))
                    .on_press(play),
                    prose(strings::lookup(Text::CardPlay), typeface::BODY),
                    iced::widget::tooltip::Position::Top,
                )
                .style(style::dialog),
            )
            .center_x(Fill)
            .center_y(Fill)
            .into(),
        );
    }
    if !hovered.controls.is_empty() {
        let glyph = style::drawn(space::CARD_OVERLAY_GLYPH.drawn());
        layers.push(
            container(row(hovered.controls.into_iter().map(|control| {
                let tint = control.tint;
                iced::widget::tooltip(
                    button(
                        container(crate::icon::icon(
                            control.glyph,
                            typeface::CARD_OVERLAY_ICON,
                        ))
                        .center_x(glyph)
                        .center_y(glyph),
                    )
                    .style(move |theme, status| style::card_overlay_control(theme, status, tint))
                    .padding(style::drawn(space::CARD_OVERLAY_PAD.drawn()))
                    .on_press(control.press),
                    prose(strings::lookup(control.label), typeface::BODY),
                    iced::widget::tooltip::Position::Top,
                )
                .style(style::dialog)
                .into()
            })))
            .align_right(Fill)
            .align_bottom(Fill)
            .into(),
        );
    }
    iced::widget::hover(frame, iced::widget::Stack::with_children(layers))
}

/// One card as `buildCard` builds one: its frame over the lines `written`
/// answers for each line the footer pushes, empty answers dropped and the rest
/// capped and filled out to what the footer writes.
// reference: card-box
// reference: card-visual
// reference: card-text-lines
// reference: card-footer-element
pub fn card<'a>(
    drawing: card::Drawing,
    room: Room,
    poster: Poster,
    written: impl Fn(card::Line) -> String,
) -> Element<'a, Message> {
    let counted = drawing.footer.written();
    let mut said: Vec<String> = drawing
        .footer
        .pushed()
        .iter()
        .map(|line| written(*line))
        .filter(|text| !text.is_empty())
        .take(counted)
        .collect();
    said.resize(counted, String::new());
    let mut lines = said.into_iter();
    let mut drawn: Vec<Element<'a, Message>> = lines.next().map(named).into_iter().collect();
    drawn.extend(lines.map(beneath));

    let frame = hovered(
        marked(
            progressed(
                framed(
                    drawing.card,
                    room,
                    poster.face,
                    &poster.name,
                    drawing.backing,
                ),
                poster.elapsed,
            ),
            poster.timer,
        ),
        poster.hovered,
        drawing.backing,
    );
    let footer = (!drawn.is_empty()).then(|| {
        footed(
            drawn,
            drawing.setting,
            drawing.footing,
            poster.logo,
            None,
            room.viewport().layout(),
        )
    });
    let body = reserving(
        boxed(drawing.card, room, frame, footer, drawing.backing),
        room,
        drawing.bottom,
    );
    let Some(press) = poster.press else {
        return body;
    };
    button(body).style(style::flat).on_press(press).into()
}

/// Cards broken into rows of `card.across(room)`, each row laid where `wrap`
/// lays it.
// reference: card-container
// reference: page-centering
pub fn wall<'a>(
    card: card::Card,
    room: Room,
    wrap: card::Wrap,
    cards: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    let across = card.across(room).count();
    let gutter = style::drawn(space::GUTTER.drawn());
    let mut cards = cards.into_iter().peekable();
    column(std::iter::from_fn(move || {
        cards.peek()?;
        let laid = row(cards.by_ref().take(across)).spacing(gutter);
        Some(match wrap {
            card::Wrap::Leading => laid.into(),
            card::Wrap::Centred => container(laid).center_x(Fill).into(),
        })
    }))
    .spacing(gutter)
    .into()
}

/// The banner the reference draws where a page carries no logo of its own, in
/// the slot its own rule gives it.
// reference: scheme-logo
pub fn logo<'a>() -> Element<'a, Message> {
    const BANNER: &[u8] = include_bytes!("../branding/banner-light.png");
    container(
        image(image::Handle::from_bytes(BANNER)).height(style::drawn(space::LOGO.height.drawn())),
    )
    .width(style::drawn(space::LOGO.width.drawn()))
    .height(style::drawn(space::LOGO.height.drawn()))
    .into()
}

/// The page a standalone screen is drawn on: the header slot the reference's
/// banner stands in, its own side padding, its top and bottom, and the body
/// scrolled and centered in what is left. Every page carrying no title of its
/// own carries the banner.
// reference: page-centering
// reference: page-padded
// reference: page-default-title
pub fn page<'a>(viewport: Viewport, body: Element<'a, Message>) -> Element<'a, Message> {
    let side = style::drawn(space::page_side(viewport.canvas()));
    let header = container(logo())
        .padding(style::drawn(space::HEADER_PAD.drawn()))
        .width(Fill)
        .style(style::header);
    let body: Element<'a, Message> = column![header, scrolled(container(body).center_x(Fill))]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .into();
    container(body)
        .padding(iced::Padding {
            top: style::drawn(space::PAGE_TOP.drawn()),
            right: side,
            bottom: style::drawn(space::PAGE_BOTTOM.drawn()),
            left: side,
        })
        .center_x(Fill)
        .height(Fill)
        .style(style::page)
        .into()
}

/// A library item's card: its poster, the lines its section writes under it,
/// and the hover menu's four controls, each standing where its own predicate
/// answers for the item.
// reference: card-hover-menu
// reference: item-can-play
// reference: item-can-mark-played
// reference: item-can-rate
pub fn poster<'a>(
    drawing: card::Drawing,
    item: &'a BaseItemDto,
    room: Room,
    images: &'a Cache,
    now: chrono::DateTime<chrono::Utc>,
    writes: Writes,
) -> Element<'a, Message> {
    let name = item.name.clone().unwrap_or_default();
    let said = subtitle(item);
    let mut controls = Vec::new();
    if let Some(id) = item.id.filter(|_| writes == Writes::Offered) {
        let played = item::played(item);
        let favorite = item::favorited(item);
        if item::markable(item) {
            controls.push(Control {
                glyph: Icon::Check,
                tint: match played {
                    Mark::Set => style::Tint::Played,
                    Mark::Cleared => style::Tint::Plain,
                },
                label: match played {
                    Mark::Set => Text::CardWatched,
                    Mark::Cleared => Text::CardMarkPlayed,
                },
                press: Message::PlayedToggled(id, played.flipped()),
            });
        }
        if item::ratable(item) {
            controls.push(Control {
                glyph: Icon::Favorite,
                tint: match favorite {
                    Mark::Set => style::Tint::Favorite,
                    Mark::Cleared => style::Tint::Plain,
                },
                label: match favorite {
                    Mark::Set => Text::CardFavorite,
                    Mark::Cleared => Text::CardAddToFavorites,
                },
                press: Message::FavoriteToggled(id, favorite.flipped()),
            });
        }
        controls.push(Control {
            glyph: Icon::MoreVert,
            tint: style::Tint::Plain,
            label: Text::OverflowOpen,
            press: Message::OverflowAction(crate::screen::overflow::Action::Open {
                item: id,
                played,
                favorite,
            }),
        });
    }
    let plays = item.id.filter(|_| item::playable(item, now)).map(|id| {
        Message::PlayPressed(crate::player::Intent::Item {
            item: id,
            resume: true,
        })
    });
    card(
        drawing,
        room,
        Poster {
            face: faced(item, drawing.card, images),
            name: name.clone(),
            logo: None,
            timer: None,
            elapsed: None,
            press: item.id.map(|id| Message::Navigated(opens(item, id))),
            hovered: Hovered { plays, controls },
        },
        move |line| match line {
            card::Line::Name => name.clone(),
            card::Line::Subtitle => said.clone(),
            _ => String::new(),
        },
    )
}

fn cards<'a>(
    drawing: card::Drawing,
    items: impl IntoIterator<Item = &'a BaseItemDto>,
    room: Room,
    images: &'a Cache,
    now: chrono::DateTime<chrono::Utc>,
    writes: Writes,
) -> Vec<Element<'a, Message>> {
    items
        .into_iter()
        .map(|item| poster(drawing, item, room, images, now, writes))
        .collect()
}

/// `emby-scroller`: a run of cards scrolled sideways at the height one card of
/// `drawing` pitches down the page.
// reference: card-container
pub fn scroller<'a>(
    drawing: card::Drawing,
    room: Room,
    cards: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    sideways(row(cards).spacing(style::drawn(space::GUTTER.drawn())))
        .height(style::drawn(drawing.row(room)))
        .into()
}

/// A rail of cards, scrolled sideways under whatever title the section it
/// stands in carries. Items arrive as an iterator rather than a slice, because
/// a section's items are as often a group borrowed out of one list as a list of
/// their own.
pub fn rail<'a>(
    drawing: card::Drawing,
    items: impl IntoIterator<Item = &'a BaseItemDto>,
    room: Room,
    images: &'a Cache,
    now: chrono::DateTime<chrono::Utc>,
    writes: Writes,
) -> Element<'a, Message> {
    scroller(
        drawing,
        room,
        cards(drawing, items, room, images, now, writes),
    )
}

/// Whether a navigation is showing what one of its entries names, and what
/// pressing it sends where it is not. This is the cap on a `Message`: a
/// payload that outgrows the unit variant beside it stops the build, and
/// crosses behind a `Box` instead.
#[derive(Debug, Clone)]
pub enum Showing {
    Shown,
    Offered(Message),
}

/// One entry of a navigation: what it is labelled, and whether the navigation
/// is showing what it names.
#[derive(Debug, Clone)]
pub struct Entry {
    pub label: Text,
    pub showing: Showing,
}

/// What pressing an entry sends: what its `Showing` carries, and the press
/// that changes nothing where the navigation is already showing what it names.
fn pressed(showing: Showing) -> Message {
    match showing {
        Showing::Shown => Message::Unchanged,
        Showing::Offered(press) => press,
    }
}

/// The rail the reference's own scrollbar stands in, which is as wide as its
/// scroller.
// reference: scrollbar-size
fn scrollbar() -> scrollable::Scrollbar {
    let width = style::drawn(space::SCROLLBAR.drawn());
    scrollable::Scrollbar::default()
        .width(width)
        .scroller_width(width)
        .margin(style::drawn(Drawn::ZERO))
}

/// A surface scrolled up and down, on the rail the reference's scheme draws.
// reference: scheme-scrollbar
// reference: scrollbar-size
pub fn scrolled<'a>(
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Scrollable<'a, Message> {
    scrollable(content)
        .direction(scrollable::Direction::Vertical(scrollbar()))
        .style(style::scrollbar)
}

/// A surface scrolled sideways, on that same rail.
// reference: scheme-scrollbar
// reference: scrollbar-size
pub fn sideways<'a>(
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Scrollable<'a, Message> {
    scrollable(content)
        .direction(scrollable::Direction::Horizontal(scrollbar()))
        .style(style::scrollbar)
}

/// A surface scrolled sideways under no rail at all, which is what the
/// reference's own tab strip scrolls under.
// reference: tab-scroll
pub fn hidden<'a>(
    content: impl Into<Element<'a, Message>>,
) -> iced::widget::Scrollable<'a, Message> {
    let bare = scrollable::Scrollbar::default()
        .width(style::drawn(Drawn::ZERO))
        .scroller_width(style::drawn(Drawn::ZERO))
        .margin(style::drawn(Drawn::ZERO));
    scrollable(content).direction(scrollable::Direction::Horizontal(bare))
}

/// The reference's tab strip: its tabs shoulder to shoulder at the middle of
/// the page, in the bolder face and the tighter line box the strip writes,
/// scrolled sideways under no bar and no scroller where they do not fit.
/// Every tab presses, including the one being shown, so every tab lights under
/// the pointer the way the reference's own hover rule lights it.
// reference: control-tab
// reference: tab-strip
// reference: tab-strip-centred
// reference: tab-scroll
pub fn tabs<'a>(
    viewport: Viewport,
    entries: impl IntoIterator<Item = Entry>,
) -> Element<'a, Message> {
    let strip = row(entries.into_iter().map(|entry| {
        let face: fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style =
            match entry.showing {
                Showing::Shown => style::tab_shown,
                Showing::Offered(_) => style::tab_offered,
            };
        button(tinted(
            strings::lookup(entry.label),
            typeface::tab(viewport),
            typeface::TAB_WEIGHT,
            typeface::TAB_LEADING,
            iced::widget::text::default,
        ))
        .padding(style::padding(space::tab_pad(viewport)))
        .style(face)
        .on_press(pressed(entry.showing))
        .into()
    }));
    container(hidden(strip)).center_x(Fill).into()
}

/// The reference's `.localnav`: its controls abutting in one group at the
/// page's leading edge, each laid over the one before it, the group's radius
/// at its two ends, over the room the row reserves under itself.
// reference: control-localnav
// reference: control-localnav-group
// reference: localnav-row
pub fn localnav<'a>(entries: impl IntoIterator<Item = Entry>) -> Element<'a, Message> {
    let mut entries = entries.into_iter().enumerate().peekable();
    let group = row(std::iter::from_fn(move || {
        let (at, entry) = entries.next()?;
        let ends = match (at == 0, entries.peek().is_none()) {
            (true, true) => style::Ends::Both,
            (true, false) => style::Ends::Leading,
            (false, true) => style::Ends::Trailing,
            (false, false) => style::Ends::Neither,
        };
        let face: fn(
            &iced::Theme,
            iced::widget::button::Status,
            style::Ends,
        ) -> iced::widget::button::Style = match entry.showing {
            Showing::Shown => style::localnav_shown,
            Showing::Offered(_) => style::localnav_offered,
        };
        let label = strings::lookup(entry.label);
        Some(
            button(prose(label, typeface::BODY))
                .padding(style::padding(space::LOCALNAV_PAD))
                .style(move |theme, status| face(theme, status, ends))
                .on_press(pressed(entry.showing))
                .into(),
        )
    }))
    .spacing(style::drawn(space::LOCALNAV_OVERLAP.drawn()));
    column![
        group,
        Space::new().height(style::drawn(space::LOCALNAV_BOTTOM.drawn())),
    ]
    .into()
}

/// One segment of the top toolbar's group: what it reads, and whether the
/// screen is already showing what it names.
#[derive(Debug, Clone)]
pub struct Segment {
    pub label: Text,
    pub showing: Showing,
}

/// MUI's `ToggleButtonGroup` at its small size: the segments in one bar, each
/// laid over the edge of the one before it, the group's radius falling at its
/// two ends alone.
// reference: mui-toggle-button
// reference: mui-toggle-group
pub fn toggles<'a>(
    segments: impl IntoIterator<Item = Segment>,
    layout: Layout,
) -> Element<'a, Message> {
    let mut segments = segments.into_iter().enumerate().peekable();
    row(std::iter::from_fn(move || {
        let (at, segment) = segments.next()?;
        let ends = match (at == 0, segments.peek().is_none()) {
            (true, true) => style::Ends::Both,
            (true, false) => style::Ends::Leading,
            (false, true) => style::Ends::Trailing,
            (false, false) => style::Ends::Neither,
        };
        let face: fn(
            &iced::Theme,
            iced::widget::button::Status,
            style::Ends,
            Layout,
        ) -> iced::widget::button::Style = match segment.showing {
            Showing::Shown => style::toggle_shown,
            Showing::Offered(_) => style::toggle_offered,
        };
        Some(
            button(tinted(
                strings::lookup(segment.label),
                typeface::TOGGLE,
                typeface::Weight::Regular,
                typeface::BUTTON_LEADING,
                iced::widget::text::default,
            ))
            .padding(style::drawn(space::TOGGLE_PAD.drawn(layout)))
            .style(move |theme, status| face(theme, status, ends, layout))
            .on_press(pressed(segment.showing))
            .into(),
        )
    }))
    .spacing(style::drawn(space::TOGGLE_OVERLAP.drawn(layout)))
    .into()
}

/// The metadata manager's own page: its parts stacked in a sidebar with the
/// one being shown in the accent, the rule down that sidebar's trailing edge,
/// the gutter the reference leaves beside it and the part shown, at the two
/// widths the reference writes; on a narrower page the part shown alone, and
/// the parts alone while no part is shown.
// reference: metadata-tree
// reference: metadata-sidebar
// reference: metadata-sidebar-wide
// reference: metadata-sidebar-hidden
// reference: metadata-sidebar-selected
pub fn editor<'a>(
    parts: impl IntoIterator<Item = Entry>,
    body: Option<Element<'a, Message>>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let stacked: Element<'a, Message> = column(parts.into_iter().map(|entry| {
        let face: fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style =
            match entry.showing {
                Showing::Shown => style::tree_shown,
                Showing::Offered(_) => style::tree_offered,
            };
        button(prose(strings::lookup(entry.label), typeface::BODY))
            .padding(style::padding(space::LIST_ITEM_PAD))
            .width(Fill)
            .style(face)
            .on_press(pressed(entry.showing))
            .into()
    }))
    .into();

    if !viewport.matches(space::EDITOR_BESIDE_AT) {
        return match body {
            Some(body) => body,
            None => stacked,
        };
    }
    let shown: Element<'a, Message> = match body {
        Some(body) => body,
        None => Space::new().into(),
    };
    let (sidebar, gap, content) = match viewport.matches(space::EDITOR_WIDE_AT) {
        true => (
            space::EDITOR_SIDEBAR_WIDE,
            space::EDITOR_GAP_WIDE,
            space::EDITOR_CONTENT_WIDE,
        ),
        false => (
            space::EDITOR_SIDEBAR,
            space::EDITOR_GAP,
            space::EDITOR_CONTENT,
        ),
    };
    row![
        container(stacked).width(Length::FillPortion(portion(sidebar))),
        container(Space::new())
            .width(style::drawn(space::EDITOR_RULE.drawn(viewport.layout())))
            .height(Fill)
            .style(style::editor_rule),
        Space::new().width(Length::FillPortion(portion(gap))),
        container(shown).width(Length::FillPortion(portion(content))),
    ]
    .into()
}

/// One destination the navigation drawer reaches: the glyph it stands behind,
/// what it is labelled, and whether the drawer is showing what it names.
#[derive(Debug, Clone)]
pub struct Link {
    pub glyph: Icon,
    pub label: Text,
    pub showing: Showing,
}

/// Whether a group is standing over the destinations it holds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Held {
    Shown,
    Hidden,
}

/// One row of the navigation drawer: a destination, or the group of
/// destinations standing under its own glyph and label.
#[derive(Debug, Clone)]
pub enum Rung {
    Reached(Link),
    Group {
        glyph: Icon,
        label: Text,
        /// What pressing the group's own row sends, which opens it or closes
        /// it.
        press: Message,
        showing: Held,
        held: Vec<Link>,
    },
}

/// The reference's navigation drawer: its rows in one column on its own
/// surface, the row whose screen is shown carrying the accent, a group
/// carrying the glyph that says which way it opens, and the rows a group holds
/// standing at the group's own inset while it is open.
// reference: drawer-paper
// reference: mui-list-item-button
// reference: dashboard-list-icon-slot
pub fn drawer<'a>(rungs: impl IntoIterator<Item = Rung>, layout: Layout) -> Element<'a, Message> {
    let mut rows: Vec<Element<'a, Message>> = Vec::new();
    for standing in rungs {
        match standing {
            Rung::Reached(link) => rows.push(mui::row(
                mui::Row {
                    lead: Some(mui::Lead::Glyph(link.glyph)),
                    primary: mui::Primary::Said(strings::lookup(link.label).into()),
                    beneath: None,
                    within: None,
                    showing: Some(link.showing.clone()),
                    trailing: None,
                },
                layout,
            )),
            Rung::Group {
                glyph,
                label,
                press,
                showing,
                held,
            } => {
                let arrow = match showing {
                    Held::Shown => Icon::ExpandLess,
                    Held::Hidden => Icon::ExpandMore,
                };
                rows.push(mui::row(
                    mui::Row {
                        lead: Some(mui::Lead::Glyph(glyph)),
                        primary: mui::Primary::Said(strings::lookup(label).into()),
                        beneath: None,
                        within: Some(arrow),
                        showing: Some(Showing::Offered(press)),
                        trailing: None,
                    },
                    layout,
                ));
                if showing == Held::Shown {
                    for link in held {
                        rows.push(mui::row(
                            mui::Row {
                                lead: Some(mui::Lead::Nested),
                                primary: mui::Primary::Said(strings::lookup(link.label).into()),
                                beneath: None,
                                within: None,
                                showing: Some(link.showing.clone()),
                                trailing: None,
                            },
                            layout,
                        ));
                    }
                }
            }
        }
    }
    container(scrolled(column(rows)))
        .width(style::drawn(space::DRAWER.drawn(layout)))
        .height(Fill)
        .padding(iced::Padding::ZERO.bottom(style::drawn(space::DRAWER_BOTTOM.drawn())))
        .style(style::drawer)
        .into()
}

/// A section's title over its body, the title padded above by the one length
/// `.sectionTitleContainer-cards` gives it.
// reference: section-title-cards
pub fn section<'a>(
    title: impl Into<Cow<'a, str>>,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    column![
        container(prose(title, typeface::HEADING_2))
            .padding(iced::Padding::ZERO.top(style::drawn(space::SECTION_TITLE_TOP.drawn()))),
        body,
    ]
    .into()
}

/// A wall of posters, laid out at the count the shape's own ladder puts in a
/// row at this page.
pub fn posters<'a>(
    drawing: card::Drawing,
    items: impl IntoIterator<Item = &'a BaseItemDto>,
    room: Room,
    images: &'a Cache,
    now: chrono::DateTime<chrono::Utc>,
    writes: Writes,
) -> Element<'a, Message> {
    scrolled(
        grid(cards(drawing, items, room, images, now, writes))
            .columns(drawing.card.across(room).count())
            .spacing(style::drawn(space::GUTTER.drawn())),
    )
    .height(Fill)
    .into()
}

/// A library's tile: the view's own image, or its collection's glyph over the
/// background its name picks where the cache holds no image for it, over its
/// centred, elided name.
// reference: home-library-tiles
// reference: card-image-primary
// reference: card-no-image
// reference: card-default-glyph
pub fn library_tile<'a>(
    library: &'a BaseItemDto,
    room: Room,
    images: &'a Cache,
    press: Message,
) -> Element<'a, Message> {
    let name = library.name.clone().unwrap_or_default();
    let said = name.clone();
    card(
        TILE,
        room,
        Poster {
            face: faced(library, TILE.card, images),
            name,
            logo: None,
            timer: None,
            elapsed: None,
            press: Some(press),
            hovered: Hovered::default(),
        },
        move |line| match line {
            card::Line::Name => said.clone(),
            _ => String::new(),
        },
    )
}

/// The box `resolveCardBoxCssClasses` gives a card that carries a footer and no
/// paper, which is what the My Media section's tiles stand on.
// reference: card-box-classes
// reference: home-library-tiles
pub const TILE: card::Drawing = card::Drawing {
    card: card::Card::LIBRARY,
    footer: card::Footer::Name,
    backing: card::Backing::Padder,
    footing: card::Footing::Bare,
    setting: card::Setting::Centred,
    bottom: card::Bottom::Padded,
};

/// The portions a bar is divided into, which is the resolution a share the
/// reference writes is exact to.
const PORTIONS: u16 = 10_000;

/// `share` of what it is laid in, as iced's own fill portions.
fn portion(share: Share) -> u16 {
    style::drawn(share.of(Drawn::of(f64::from(PORTIONS)))) as u16
}

/// A bar filled to `elapsed`, which is how far through a program `now` is.
pub fn elapsed_bar<'a>(elapsed: Share) -> Element<'a, Message> {
    let filled = portion(elapsed);
    let height = style::drawn(space::PROGRESS.drawn());
    container(
        row![
            container(Space::new())
                .width(Length::FillPortion(filled))
                .height(height)
                .style(|theme: &iced::Theme| container::Style::default()
                    .background(theme.palette().primary)),
            container(Space::new())
                .width(Length::FillPortion(PORTIONS - filled))
                .height(height),
        ]
        .height(height),
    )
    .width(Length::Fill)
    .height(height)
    .into()
}

/// The box the On Now row's cards stand in: an outer footer and no paper, on
/// the backdrop rail the section's own `defaultShape` names, its lines set
/// where `centerText` sets them.
// reference: card-box-classes
// reference: livetv-program-sections
pub const ON_NOW: card::Drawing = card::Drawing {
    card: card::Card::Rail(card::Rail::Backdrop),
    footer: card::Footer::NameAndSubtitle,
    backing: card::Backing::Padder,
    footing: card::Footing::Bare,
    setting: card::Setting::Centred,
    bottom: card::Bottom::Padded,
};

/// One on-now card: the channel's logo, its number, and its current program's
/// title with an elapsed bar, on the card the reference's on-now rail draws.
// reference: indicator-timer
pub fn channel_card<'a>(
    channel: &'a Channel,
    room: Room,
    now: chrono::DateTime<chrono::Utc>,
    image: Option<image::Handle>,
) -> Element<'a, Message> {
    let name = format!("{} {}", channel.number, channel.name);
    let said = channel
        .current
        .as_ref()
        .map(|program| program.title.clone())
        .unwrap_or_default();
    let written = name.clone();
    card(
        ON_NOW,
        room,
        Poster {
            face: image.map(Face::Image),
            name,
            logo: None,
            timer: channel.current.as_ref().and_then(Program::recording),
            elapsed: channel.current.as_ref().map(|program| program.elapsed(now)),
            press: Some(Message::LiveTvAction(
                crate::screen::livetv::Action::PlayChannel(channel.id),
            )),
            hovered: Hovered::default(),
        },
        move |line| match line {
            card::Line::Name => written.clone(),
            card::Line::Subtitle => said.clone(),
            _ => String::new(),
        },
    )
}

/// The channels on now, capped at `home::ON_NOW`, the section that holds them
/// drawing their title.
pub fn on_now_row<'a>(
    channels: &'a [Channel],
    room: Room,
    now: chrono::DateTime<chrono::Utc>,
    images: &'a Cache,
) -> Element<'a, Message> {
    let cards = channels
        .iter()
        .take(crate::screen::home::ON_NOW as usize)
        .map(|channel| {
            let handle = images
                .handle(images::Key {
                    item: channel.id,
                    kind: Kind::Primary,
                    index: None,
                    card: ON_NOW.card,
                })
                .clone();
            channel_card(channel, room, now, handle)
        });

    scroller(ON_NOW, room, cards)
}

/// Which control ends a failure report.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    /// Dismisses the report, the session standing.
    Dismissed,
    /// Opens a fresh login screen, which is what a session that is gone ends
    /// with.
    SignedInAgain,
}

/// One failure report as it is shown: its sentence, the Jellyfin server's own
/// message beneath it as quoted server output under the `serverSaid` label,
/// and the control that ends it.
pub fn reported<'a>(failure: &'a crate::failure::Failure, ending: Ending) -> Element<'a, Message> {
    let mut shown = column![prose(failure.sentence.clone(), typeface::BODY)]
        .spacing(style::drawn(space::BLOCK_GAP.drawn()));
    if let Some(server) = &failure.server {
        shown = shown
            .push(prose(
                strings::lookup(Text::ServerSaid),
                typeface::SECONDARY,
            ))
            .push(prose(format!("> {server}"), typeface::SECONDARY));
    }
    let (label, press) = match ending {
        Ending::Dismissed => (Text::FailureDismiss, Message::FailureDismissed),
        Ending::SignedInAgain => (Text::FailureSignInAgain, Message::SignInAgain),
    };
    shown = shown.push(
        button(prose(strings::lookup(label), typeface::BODY))
            .style(style::raised)
            .on_press(press),
    );
    container(shown)
        .padding(style::padding(space::PAGE_PAD))
        .width(Length::Fill)
        .into()
}

/// Every screen under the failure surfaces: the report shown above it until it
/// is dismissed, the control opening the session's failure list, and the list
/// itself while it is open.
pub fn shell<'a>(
    failures: &'a crate::failure::Log,
    listing: Listing,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    let mut page = column![].spacing(style::drawn(space::SECTION_GAP.drawn()));
    if let Some(showing) = failures.showing() {
        page = page.push(reported(showing, Ending::Dismissed));
    }
    page = page.push(
        button(prose(strings::lookup(Text::FailuresOpen), typeface::BODY))
            .style(style::flat)
            .on_press(match listing {
                Listing::Open => Message::FailuresClosed,
                Listing::Closed => Message::FailuresOpened,
            }),
    );
    if listing == Listing::Open {
        let mut listed = column![prose(strings::lookup(Text::FailuresTitle), typeface::BODY)]
            .spacing(style::drawn(space::SECTION_GAP.drawn()))
            .padding(style::padding(space::PAGE_PAD));
        if failures.raised().is_empty() {
            listed = listed.push(prose(strings::lookup(Text::FailuresEmpty), typeface::BODY));
        }
        for raised in failures.raised() {
            let mut one = column![prose(raised.sentence.clone(), typeface::BODY)];
            if let Some(server) = &raised.server {
                one = one.push(prose(format!("> {server}"), typeface::SECONDARY));
            }
            listed = listed.push(one.spacing(style::drawn(space::BLOCK_GAP.drawn())));
        }
        page = page.push(scrolled(listed));
    }
    page.push(body).into()
}

/// Whether the session's failure list is drawn under the control that opens it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Listing {
    Open,
    Closed,
}

/// The nav row above `body`, both inside the page's own padding: the page's
/// own above and below, and `page_side`'s share at each edge.
/// `nav` is `Settings` while a settings route is on top, which is what the
/// settings column replaces; Back and Logout stand either way.
// reference: page-padded
// reference: page-side
pub fn chrome<'a>(
    session: &'a Session,
    back: Back,
    nav: Nav,
    viewport: Viewport,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    let mut controls = row![].spacing(style::drawn(space::CONTROL_GAP.drawn()));

    if back == Back::Offered {
        controls = controls.push(
            button(prose(strings::lookup(Text::NavBack), typeface::BODY))
                .style(style::flat)
                .on_press(Message::WentBack),
        );
    }
    if nav == Nav::Browse {
        controls = controls
            .push(
                button(prose(strings::lookup(Text::NavHome), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::Navigated(Route::Home)),
            )
            .push(
                button(prose(strings::lookup(Text::NavSearch), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::Navigated(Route::Search {
                        term: String::new(),
                    })),
            )
            .push(
                button(prose(strings::lookup(Text::NavSettings), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::Navigated(Route::Settings {
                        screen: crate::screen::settings::Screen::Menu,
                    })),
            )
            .push(
                button(prose(strings::lookup(Text::NavRemote), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::Navigated(Route::Remote)),
            );

        if session.sync_play != SyncAccess::None {
            controls = controls.push(
                button(prose(strings::lookup(Text::NavSyncPlay), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::Navigated(Route::SyncPlay)),
            );
        }
        if session.administrator {
            controls = controls.push(
                button(prose(strings::lookup(Text::NavDashboard), typeface::BODY))
                    .style(style::flat)
                    .on_press(Message::Navigated(Route::Dashboard {
                        screen: crate::screen::dashboard::Screen::Plugins,
                    })),
            );
        }
    }
    controls = controls.push(
        button(prose(strings::lookup(Text::NavSwitch), typeface::BODY))
            .style(style::flat)
            .on_press(Message::SwitchPressed),
    );
    if !session.read_only {
        controls = controls.push(
            button(prose(strings::lookup(Text::NavLogout), typeface::BODY))
                .style(style::flat)
                .on_press(Message::LogoutPressed),
        );
    }

    let side = style::drawn(space::page_side(viewport.canvas()));
    let page = column![controls]
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .padding(style::padding(space::PAGE_PAD).left(side).right(side));

    page.push(body).into()
}

/// One notice as the reference draws one: a card carrying its header over its
/// sentence, or its sentence alone where a notice has no header, on a surface
/// no narrower than `.toast`'s own floor and as wide as its content asks.
// reference: toast-face
pub fn toast<'a>(header: Option<String>, sentence: String) -> Element<'a, Message> {
    let mut written = column![Space::new().width(style::drawn(space::TOAST_MIN_INSIDE.drawn()))];
    if let Some(header) = header {
        written = written.push(prose(header, typeface::TOAST));
    }
    container(written.push(prose(sentence, typeface::TOAST)))
        .padding(style::padding(space::TOAST_PAD))
        .style(style::toast)
        .into()
}

/// `notices` standing over `body` at the foot of its leading edge, at the gap
/// the reference leaves between two of them and the container's own inset.
/// `body` alone where nothing is raised.
// reference: toast-container
pub fn toasted<'a>(
    body: Element<'a, Message>,
    notices: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    let mut raised = notices.into_iter().peekable();
    if raised.peek().is_none() {
        return body;
    }
    let stack = container(column(raised).spacing(style::drawn(space::TOAST_GAP.drawn())))
        .padding(
            iced::Padding::ZERO
                .left(style::drawn(space::TOAST_INSET.drawn()))
                .bottom(style::drawn(space::TOAST_INSET.drawn())),
        )
        .align_bottom(Fill);
    iced::widget::stack![body, stack].into()
}

/// Every notice the session itself raises: read-only access, a server whose
/// version the snapshot does not name, the group being followed, that group
/// waiting, and live updates being down.
pub fn notices<'a>(
    session: &'a Session,
    group: Option<&'a Joined>,
    live: live::Link,
) -> Vec<Element<'a, Message>> {
    let mut raised = Vec::new();
    if session.read_only {
        raised.push(toast(
            None,
            strings::lookup(Text::DashboardReadOnly).to_string(),
        ));
    }
    if session.off_snapshot() {
        raised.push(toast(
            None,
            strings::format(
                Text::WarningOffSnapshot,
                &[&session.server_version, &session.snapshot_version],
            ),
        ));
    }
    if let Some(joined) = group {
        raised.push(toast(
            None,
            strings::lookup(Text::SyncPlayActive).to_string(),
        ));
        if joined.waiting() {
            raised.push(toast(
                None,
                strings::lookup(Text::SyncPlayWaiting).to_string(),
            ));
        }
    }
    if live.down() {
        raised.push(toast(
            None,
            strings::lookup(Text::LiveUnavailable).to_string(),
        ));
    }
    raised
}

/// Whether the chrome offers the control that steps back.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Back {
    Offered,
    Withheld,
}

/// Which column the chrome's nav draws: the browse row, or the settings column
/// that replaces it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Nav {
    Browse,
    Settings,
}

/// A column of rows held to the width the reference caps a form and
/// `.readOnlyContent` at and centred in the page it is drawn on, its rows at
/// the margin `gap` each carries under itself. The page's own padding belongs
/// to the page, not to the column.
// reference: page-bottom
// reference: page-centering
pub fn capped<'a>(
    viewport: Viewport,
    gap: style::Length,
    rows: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    let held = container(column(rows).spacing(style::drawn(gap.drawn())));
    let held = match viewport.matches(space::FORM_WIDTH_AT) {
        true => held.max_width(style::drawn(space::FORM_WIDTH.drawn())),
        false => held,
    };
    container(held).center_x(Fill).into()
}

/// `.paper-icon-button-light`: a disc carrying a glyph and no face of its own,
/// naming itself where the reference writes a title and standing bare where it
/// writes none.
// reference: control-icon-button
// reference: control-icon-glyph
pub fn icon_button<'a>(
    glyph: crate::icon::Icon,
    size: style::Length,
    label: Option<Text>,
    press: Message,
) -> Element<'a, Message> {
    let control = button(crate::icon::icon(glyph, size))
        .style(style::icon_control)
        .padding(style::drawn(space::PAPER_ICON_BUTTON_PAD.drawn()))
        .on_press(press);
    let Some(label) = label else {
        return control.into();
    };
    iced::widget::tooltip(
        control,
        prose(strings::lookup(label), typeface::BODY),
        iced::widget::tooltip::Position::Top,
    )
    .style(style::dialog)
    .into()
}

/// `.fab.submit`: a glyph on the submit face's disc, naming itself where the
/// reference writes a title.
// reference: control-fab
// reference: type-button-icon
pub fn fab<'a>(glyph: crate::icon::Icon, label: Text, press: Message) -> Element<'a, Message> {
    let control = button(crate::icon::icon(glyph, typeface::BUTTON_ICON))
        .style(style::fab)
        .padding(style::drawn(space::FAB_PAD.drawn()))
        .on_press(press);
    iced::widget::tooltip(
        control,
        prose(strings::lookup(label), typeface::BODY),
        iced::widget::tooltip::Position::Top,
    )
    .style(style::dialog)
    .into()
}

/// `.sectionTitleContainer`: an `h2.sectionTitle` with the control the section
/// carries beside it, at the margin the container leaves above and below and
/// the margin `.sectionTitleButton` leaves beside the title.
// reference: section-title
// reference: section-title-button
pub fn titled<'a>(
    title: impl Into<Cow<'a, str>>,
    control: Option<Element<'a, Message>>,
) -> Element<'a, Message> {
    let written = heading(typeface::Rank::Second, title);
    let held = match control {
        None => row![written],
        Some(control) => {
            row![written, control].spacing(style::drawn(space::SECTION_TITLE_BUTTON.drawn()))
        }
    }
    .align_y(iced::Center);
    container(held)
        .padding(
            iced::Padding::ZERO
                .top(style::drawn(space::SECTION_GAP.drawn()))
                .bottom(style::drawn(space::SECTION_GAP.drawn())),
        )
        .into()
}

/// One account's card as `UserCardBox` draws one: the image the server holds
/// for it, or `.cardImageIcon`'s own person over the background its name
/// picks, the whole box on the scheme's paper, and under it the account's name
/// over when it was last seen, both at the leading edge with the overflow
/// control on the trailing one.
// the image alone is the control, which is where the reference puts its link
// the reference greys a disabled account's card through a css filter, which
// this canvas does not apply
// reference: user-card
// reference: user-card-box
pub fn user_card<'a>(
    room: Room,
    name: String,
    last_seen: Option<String>,
    face: Option<image::Handle>,
    opens: Message,
    menu: Message,
) -> Element<'a, Message> {
    let card = card::Card::USER;
    let frame = framed(
        card,
        room,
        Some(face.map_or(Face::Icon(Icon::Person), Face::Image)),
        &name,
        card::Backing::Paper,
    );
    let secondary: Element<'a, Message> = container(tinted(
        last_seen.unwrap_or_default(),
        typeface::SECONDARY,
        typeface::Weight::Regular,
        typeface::LINE_HEIGHT,
        style::description,
    ))
    .height(style::drawn(space::USER_CARD_SECONDARY.drawn()))
    .into();
    boxed(
        card,
        room,
        button(frame).style(style::flat).on_press(opens).into(),
        Some(footed(
            vec![
                named(name),
                carded(
                    secondary,
                    typeface::SECONDARY,
                    space::card_text(typeface::SECONDARY).top,
                ),
            ],
            card::Setting::Leading,
            card::Footing::Padded,
            None,
            Some(icon_button(
                Icon::MoreVert,
                typeface::ICON_BUTTON,
                None,
                menu,
            )),
            room.viewport().layout(),
        )),
        card::Backing::Paper,
    )
}

/// The header standing over the top of the video: the control that leaves, and
/// the two the reference keeps beside it.
// reference: osd-header
// reference: osd-header-buttons
// reference: header-back
// reference: header-cast
// reference: header-sync
pub fn osd_header<'a>(sync_play: SyncAccess) -> Element<'a, Message> {
    let mut controls = row![
        icon_button(
            Icon::ArrowBack,
            typeface::ICON_BUTTON,
            Some(Text::PlayerLeave),
            Message::PlayerAction(crate::player::Action::Leave),
        ),
        icon_button(
            Icon::Cast,
            typeface::ICON_BUTTON,
            Some(Text::PlayerRemote),
            Message::Navigated(crate::route::Route::Remote),
        ),
    ]
    .align_y(iced::Center)
    .height(style::drawn(space::OSD_HEADER_TOP.drawn()));
    if sync_play != SyncAccess::None {
        controls = controls.push(icon_button(
            Icon::Groups,
            typeface::ICON_BUTTON,
            Some(Text::PlayerSyncPlay),
            Message::Navigated(crate::route::Route::SyncPlay),
        ));
    }
    container(controls)
        .width(Fill)
        .height(style::drawn(space::OSD_HEADER.drawn()))
        .style(style::osd_header)
        .into()
}

/// `control` under `.filterIndicator`: the reference's own mark that a control
/// is narrowing what is shown, laid on its top trailing corner.
/// `layout` is what the inset, written in css pixels, crosses to the canvas
/// through.
// reference: filter-indicator
// reference: filter-indicator-face
pub fn indicated<'a>(control: Element<'a, Message>, layout: Layout) -> Element<'a, Message> {
    let circle = style::drawn(space::INDICATOR.drawn());
    let inset = style::drawn(space::INDICATOR_INSET.drawn(layout));
    let mark = container(
        container(line(
            strings::lookup(Text::FilterIndicator),
            typeface::INDICATOR,
            typeface::Weight::Bold,
            typeface::LINE_HEIGHT,
        ))
        .center_x(circle)
        .center_y(circle)
        .style(style::indicator),
    )
    .padding(iced::Padding::ZERO.top(inset).right(inset))
    .align_right(Fill);
    iced::widget::stack![control, mark].into()
}

/// `body` with `letters` laid over the page's trailing edge, at the letter
/// picker's own insets from the edge and the foot of the page.
// reference: alpha-picker
// reference: alpha-picker-right
// reference: alpha-picker-fixed
pub fn lettered<'a>(
    body: Element<'a, Message>,
    letters: Element<'a, Message>,
    viewport: Viewport,
) -> Element<'a, Message> {
    let held = container(letters)
        .padding(
            iced::Padding::ZERO
                .right(style::drawn(space::letters_right(viewport)))
                .bottom(style::drawn(space::letters_bottom(viewport))),
        )
        .align_right(Fill)
        .align_bottom(Fill);
    iced::widget::stack![body, held].into()
}

/// The reference's own `h1`, `h2` and `h3`, which its pages write over
/// themselves and its `fieldset`s over their rows.
pub fn heading<'a>(rank: typeface::Rank, content: impl Into<Cow<'a, str>>) -> Element<'a, Message> {
    prose(content, rank.size())
}

/// Whether a field shows what is typed into it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Secrecy {
    Shown,
    Hidden,
}

/// A labelled field over its description, which is the reference's own
/// `.inputContainer`: an `emby-input` carrying its label, and the
/// `.fieldDescription` beneath it.
// reference: control-input
// reference: control-field
// reference: control-input-label
// reference: control-field-description
pub fn field<'a>(
    label: impl Into<Cow<'a, str>>,
    value: &str,
    description: Option<Text>,
    unit: Option<Text>,
    edited: impl Fn(String) -> Message + 'a,
    submitted: Message,
    secrecy: Secrecy,
) -> Element<'a, Message> {
    let typed = iced::widget::text_input("", value)
        .style(style::input)
        .size(style::drawn(typeface::FIELD.drawn()))
        .padding(style::padding(space::INPUT_PAD))
        .secure(secrecy == Secrecy::Hidden)
        .on_input(edited)
        .on_submit(submitted);
    let control = match unit {
        Some(sentence) => stack![typed, unitted(sentence)].into(),
        None => Element::from(typed),
    };

    let mut held = column![labelled(label, control)];
    if let Some(sentence) = description {
        held = held.push(self::description(sentence, space::DESCRIPTION_INSET));
    }
    held.into()
}

/// The unit the reference writes at an `emby-input`'s trailing edge, in the
/// secondary lettering, inside the field's own trailing padding.
// reference: control-input
// reference: scheme-secondary-text
fn unitted<'a>(sentence: Text) -> Element<'a, Message> {
    container(tinted(
        strings::lookup(sentence),
        typeface::FIELD,
        typeface::Weight::Regular,
        typeface::LINE_HEIGHT,
        style::description,
    ))
    .padding(style::padding(space::INPUT_PAD))
    .align_right(Fill)
    .center_y(Fill)
    .into()
}

/// `.fieldDescription`: the sentence the reference writes under a control, in
/// the secondary lettering, at the inset the control it stands under gives it.
// reference: field-description
// reference: scheme-secondary-text
pub fn description<'a>(sentence: Text, inset: style::Length) -> Element<'a, Message> {
    container(tinted(
        strings::lookup(sentence),
        typeface::SECONDARY,
        typeface::Weight::Regular,
        typeface::LINE_HEIGHT,
        style::description,
    ))
    .padding(
        iced::Padding::ZERO
            .top(style::drawn(space::DESCRIPTION_GAP.drawn()))
            .left(style::drawn(inset.drawn())),
    )
    .into()
}

/// `.inputLabel`, `.selectLabel` and `.checkboxListLabel`: the name the
/// reference writes over the control it addresses.
// reference: control-input-label
// reference: control-select-label
// reference: control-checkbox-list-label
pub fn labelled<'a>(
    label: impl Into<Cow<'a, str>>,
    control: Element<'a, Message>,
) -> Element<'a, Message> {
    column![
        container(tinted(
            label,
            typeface::BODY,
            typeface::Weight::Regular,
            typeface::LINE_HEIGHT,
            style::label,
        ))
        .padding(iced::Padding::ZERO.bottom(style::drawn(space::LABEL_GAP.drawn()))),
        control,
    ]
    .into()
}

// the closed field is `.emby-select`, its chevron laid over the trailing edge
// rather than drawn inside the field's own padding
// the reference places that chevron by an absolute offset from the top of the
// label; here it is centred against the field
// the label is never repainted while the field is open, which the reference
// does through `.selectLabelFocused`
// nothing is drawn in the field when no offered option carries `held`
// reference: control-select
// reference: control-select-container
// reference: control-select-label
// reference: control-select-arrow
// reference: control-select-description
pub fn select<'a, T>(
    label: impl Into<Cow<'a, str>>,
    description: Option<Text>,
    offered: Vec<Choice<T>>,
    held: &T,
    chosen: impl Fn(T) -> Message + 'a,
) -> Element<'a, Message>
where
    T: Clone + PartialEq + 'a,
{
    for choice in &offered {
        crate::fonts::observed(&choice.label, typeface::Weight::Regular);
    }
    let standing = offered.iter().find(|choice| &choice.value == held).cloned();
    let field = iced::widget::pick_list(offered, standing, move |choice| chosen(choice.value))
        .style(style::select)
        .menu_style(style::menu)
        .handle(iced::widget::pick_list::Handle::None)
        .font(style::font(typeface::Weight::Regular))
        .text_size(style::drawn(typeface::FIELD.drawn()))
        .text_line_height(style::leading(typeface::LINE_HEIGHT))
        .padding(style::padding(space::SELECT_PAD))
        .width(Fill);
    let chevron = container(crate::icon::icon(
        Icon::KeyboardArrowDown,
        typeface::SELECT_ARROW,
    ))
    .padding(iced::Padding::ZERO.right(style::drawn(space::SELECT_ARROW_INSET.drawn())))
    .align_right(Fill)
    .center_y(Fill);

    let mut held = column![labelled(label, iced::widget::stack![field, chevron].into())];
    if let Some(sentence) = description {
        held = held.push(self::description(sentence, space::DESCRIPTION_INSET));
    }
    held.into()
}

// the box is centred against the label, where the reference offsets it three
// pixels from the top of the label's own line box
// reference: control-checkbox
// reference: control-checkbox-label
// reference: control-checkbox-container
// reference: control-checkbox-description
// reference: control-checkbox-mark
pub fn flag<'a>(
    label: impl Into<Cow<'a, str>>,
    description: Option<Text>,
    held: bool,
    toggled: impl Fn(bool) -> Message + 'a,
) -> Element<'a, Message> {
    let written = label.into();
    crate::fonts::observed(&written, typeface::Weight::Regular);
    let outline = iced::widget::checkbox(held)
        .label(written)
        .on_toggle(toggled)
        .size(style::drawn(space::CHECKBOX.drawn()))
        .spacing(style::drawn(space::CHECKBOX_GAP.drawn()))
        .font(style::font(typeface::Weight::Regular))
        .text_size(style::drawn(typeface::BODY.drawn()))
        .text_line_height(style::leading(typeface::LINE_HEIGHT))
        .style(style::checkbox);
    let outline = match Icon::Check
        .glyph()
        .and_then(crate::fonts::Codepoint::character)
    {
        Some(mark) => outline.icon(iced::widget::checkbox::Icon {
            font: style::ICONS,
            code_point: mark,
            size: Some(iced::Pixels(style::drawn(typeface::CHECKBOX_MARK.drawn()))),
            line_height: style::leading(typeface::ICON_LEADING),
            shaping: iced::widget::text::Shaping::Advanced,
        }),
        None => outline,
    };

    let standing = style::drawn(space::CHECKBOX_ROW.drawn());
    let mut stacked = column![container(outline).center_y(standing)];
    if let Some(sentence) = description {
        stacked = stacked.push(self::description(sentence, space::CHECKBOX_INSET));
    }
    stacked.into()
}

/// `.verticalSection`: the heading the reference writes over a group of a
/// form's controls, over the rows it groups, at that heading's own rank.
// reference: section-vertical
// reference: section-title
// reference: control-field
pub fn fields<'a>(
    rank: typeface::Rank,
    title: Text,
    rows: impl IntoIterator<Item = Element<'a, Message>>,
) -> Element<'a, Message> {
    column![
        container(heading(rank, strings::lookup(title)))
            .padding(iced::Padding::ZERO.bottom(style::drawn(space::SECTION_GAP.drawn()))),
        column(rows).spacing(style::drawn(space::FIELD_GAP.drawn())),
    ]
    .into()
}

/// `.emby-button`: a control at the reference's own padding, no wider than
/// what it carries.
// reference: control-button
pub fn control<'a>(
    label: impl Into<Cow<'a, str>>,
    press: Option<Message>,
    emphasis: Emphasis,
) -> Element<'a, Message> {
    let face: fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style =
        match emphasis {
            Emphasis::Submit => style::submit,
            Emphasis::Raised => style::raised,
        };
    let mut held = button(prose(label, typeface::BODY))
        .style(face)
        .padding(style::padding(space::BUTTON_PAD));
    if let Some(message) = press {
        held = held.on_press(message);
    }
    held.into()
}

/// `.button-link`: the reference's anchor, its lettering in the scheme's own
/// anchor colour on no face, underlined while the pointer stands over it.
// reference: control-button-link
// reference: scheme-button-link
pub fn anchor<'a>(label: impl Into<Cow<'a, str>>, press: Message) -> Element<'a, Message> {
    let written = label.into();
    crate::fonts::observed(&written, typeface::Weight::Regular);
    rich_text([span(written)
        .color(style::color(scheme::ANCHOR))
        .link(press)])
    .size(style::drawn(typeface::BODY.drawn()))
    .font(style::font(typeface::Weight::Regular))
    .line_height(style::leading(typeface::LINE_HEIGHT))
    .on_link_click(|link| link)
    .into()
}

/// The search field as the reference draws it: its glyph, then the field
/// itself, centred together in `SEARCH_FIELD` of width.
// reference: search-field
// reference: search-field-page
pub fn searching<'a>(term: &str, viewport: Viewport) -> Element<'a, Message> {
    let glyph = container(crate::icon::icon(Icon::Search, typeface::SEARCH_ICON)).padding(
        iced::Padding::ZERO
            .right(style::drawn(space::SEARCH_ICON_GAP.drawn()))
            .bottom(style::drawn(space::SEARCH_ICON_LIFT.drawn())),
    );
    let typed = iced::widget::text_input(strings::lookup(Text::SearchPlaceholder), term)
        .style(style::input)
        .size(style::drawn(typeface::FIELD.drawn()))
        .padding(style::padding(space::INPUT_PAD))
        .on_input(Message::SearchEdited)
        .on_submit(Message::SearchSubmitted);
    let side = style::drawn(space::page_side(viewport.canvas()));
    container(
        container(row![glyph, typed].align_y(iced::Alignment::End))
            .max_width(style::drawn(space::SEARCH_FIELD.drawn())),
    )
    .padding(iced::Padding::ZERO.right(side).left(side))
    .center_x(Fill)
    .into()
}

/// Which of the reference's two block faces a control draws.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Emphasis {
    Submit,
    Raised,
}

/// A control filling the width it is given, which is the reference's own
/// `.block`.
// reference: control-button-block
pub fn block<'a>(
    label: impl Into<Cow<'a, str>>,
    press: Option<Message>,
    emphasis: Emphasis,
) -> Element<'a, Message> {
    let face: fn(&iced::Theme, iced::widget::button::Status) -> iced::widget::button::Style =
        match emphasis {
            Emphasis::Submit => style::submit,
            Emphasis::Raised => style::raised,
        };
    let mut control = button(container(prose(label, typeface::BODY)).center_x(Fill))
        .style(face)
        .padding(style::padding(space::BUTTON_PAD))
        .width(Fill);
    if let Some(message) = press {
        control = control.on_press(message);
    }
    control.into()
}

/// `.centerMessage`: one sentence in the middle of the page, in a column three
/// tenths of it wide.
// reference: center-message
pub fn centered<'a>(sentence: String) -> Element<'a, Message> {
    let held = portion(space::CENTER_MESSAGE);
    let beside = (PORTIONS - held) / 2;
    row![
        Space::new().width(Length::FillPortion(beside)),
        container(prose(sentence, typeface::BODY))
            .width(Length::FillPortion(held))
            .padding(
                iced::Padding::ZERO
                    .top(style::drawn(space::CENTER_MESSAGE_PAD.drawn()))
                    .bottom(style::drawn(space::CENTER_MESSAGE_PAD.drawn())),
            )
            .center_x(Fill),
        Space::new().width(Length::FillPortion(beside)),
    ]
    .into()
}

/// The warning raised before leaving a screen holding unsaved edits: what is
/// lost, and the two controls.
pub fn leaving<'a>() -> Element<'a, Message> {
    column![
        prose(strings::lookup(Text::DashboardUnsaved), typeface::BODY),
        row![
            button(prose(
                strings::lookup(Text::DashboardLeaveAnyway),
                typeface::BODY
            ))
            .style(style::raised)
            .on_press(Message::LeaveAnyway),
            button(prose(
                strings::lookup(Text::DashboardStayHere),
                typeface::BODY
            ))
            .style(style::raised)
            .on_press(Message::StayHere),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn())),
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()))
    .padding(style::padding(space::PAGE_PAD))
    .into()
}
