use std::rc::Rc;

use iced::Element;
use iced::widget::{column, row};
use jellium_model::paged::Limit;
use jellium_protocol::Session;
use jellyfin_api::types::{BaseItemDto, CollectionType, MediaType};

use crate::api::Api;
use crate::app::Message;
use crate::construct;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::route::Route;
use crate::screen::arrival::Arrival;
use crate::style::space::Room;
use crate::style::{self, Viewport, card};
use crate::text::{self as strings, Said, Template, Text};
use crate::widget;
use jellium_model::construct::{Construct, Page};

/// Which paragraph the reference writes under its empty heading: the one that
/// opens the libraries dashboard for an administrator, and the one that asks for
/// one otherwise.
fn empty_paragraph(session: &Session) -> Text {
    match session.administrator {
        true => Text::HomeEmptyAdministrator,
        false => Text::HomeEmptyUser,
    }
}

/// One section title the reference writes plainly, which is a title its own
/// section's list cannot be opened from.
fn titled<'a>(said: Text) -> Element<'a, Message> {
    construct::stated(
        Construct::SectionTitleCards,
        Said::Plain(said),
        widget::prose(strings::lookup(said), style::typeface::HEADING_2),
    )
}

/// One section title the reference wraps in a link, which carries the trailing
/// chevron `.sectionTitleTextButton` writes and opens what that link opens.
fn opened<'a>(said: Said, spoken: String, opens: Message) -> Element<'a, Message> {
    construct::navigation(
        Construct::SectionTitleTextButton,
        None,
        opens,
        row![
            construct::navigation(
                Construct::SectionTitleCards,
                Some(said),
                Message::Unchanged,
                widget::prose(spoken, style::typeface::HEADING_2),
            ),
            crate::icon::icon(
                crate::icon::Icon::ChevronRight,
                style::typeface::ICON_BUTTON
            ),
        ]
        .align_y(iced::Center)
        .into(),
    )
}

/// Which of the reference's three resumed sections a media type falls in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Resumed {
    Watching,
    Listening,
    Reading,
}

impl Resumed {
    /// The section a media type falls in, and None for a media type none of the
    /// three names.
    pub fn of(media: Option<MediaType>) -> Option<Resumed> {
        match media {
            Some(MediaType::Video) => Some(Resumed::Watching),
            Some(MediaType::Audio) => Some(Resumed::Listening),
            Some(MediaType::Book) => Some(Resumed::Reading),
            Some(MediaType::Unknown | MediaType::Photo) | None => None,
        }
    }

    pub fn label(self) -> Text {
        match self {
            Resumed::Watching => Text::HomeContinueWatching,
            Resumed::Listening => Text::HomeContinueListening,
            Resumed::Reading => Text::HomeContinueReading,
        }
    }
}

/// The collection types the reference draws no Latest row for.
/// The reference names five and this names four: its fifth is `channels`, which
/// the server's own `CollectionType` does not carry, so no view this client
/// reads can report it.
// reference: home-latest-excludes
const NO_LATEST: [CollectionType; 4] = [
    CollectionType::Playlists,
    CollectionType::Livetv,
    CollectionType::Boxsets,
    CollectionType::Folders,
];

/// Whether the reference draws a Latest row for a library: not for a view whose
/// collection type `NO_LATEST` names, and not for one the user's
/// `LatestItemsExcludes` names.
// reference: home-latest-excludes
pub fn latest_shown(library: &BaseItemDto, excluded: &[uuid::Uuid]) -> bool {
    let named = library
        .collection_type
        .is_some_and(|held| NO_LATEST.contains(&held));
    let hidden = library.id.is_some_and(|id| excluded.contains(&id));
    !named && !hidden
}

/// Which of the reference's two home tabs is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Tab {
    #[default]
    Home,
    Favorites,
}

impl Tab {
    pub const ALL: [Tab; 2] = [Tab::Home, Tab::Favorites];

    pub fn label(self) -> Text {
        match self {
            Tab::Home => Text::HomeTabHome,
            Tab::Favorites => Text::HomeTabFavorites,
        }
    }
}

/// `.headerTabs`: the reference's own two-tab strip over the home page, each
/// tab opening the route it names.
pub fn tabs<'a>(shown: Tab) -> Element<'a, Message> {
    construct::silent(
        Construct::HeaderTabs,
        iced::widget::row(Tab::ALL.into_iter().map(|tab| {
            construct::navigation(
                Construct::HeaderTabs,
                Some(Said::Plain(tab.label())),
                match tab == shown {
                    true => Message::Unchanged,
                    false => Message::Navigated(Route::Home { tab }),
                },
                widget::prose(strings::lookup(tab.label()), style::typeface::HEADING_2),
            )
        }))
        .spacing(style::drawn(style::space::CONTROL_GAP.drawn()))
        .into(),
    )
}

/// The reference pages this screen draws.
pub const DRAWS: &[Page] = &[Page::Home];

/// The media type a resumed section's cards take their shape from.
fn resumed_media(section: Resumed) -> Option<MediaType> {
    match section {
        Resumed::Watching => Some(MediaType::Video),
        Resumed::Listening => Some(MediaType::Audio),
        Resumed::Reading => Some(MediaType::Book),
    }
}

/// One home rail's card: the shape the section asks for, over the lines that
/// section's own options write under it.
// reference: card-box-classes
fn railed(card: card::Card, footer: card::Footer) -> card::Drawing {
    card::Drawing {
        card,
        footer,
        backing: card::Backing::Padder,
        footing: card::Footing::Bare,
        setting: card::Setting::Centred,
        bottom: card::Bottom::Padded,
        // reference: home-resume
        // reference: home-next-up
        // reference: home-latest
        // reference: livetv-program-sections
        touch: card::Touch::Plays,
    }
}

/// The most programmes the On Now row shows.
// reference: home-on-now-query
pub const ON_NOW: Limit = Limit::of(24);

/// Which home rail one answer fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    /// The three resumed rails, which this client reaches in one request where
    /// the reference reaches them in three.
    Resumed,
    NextUp,
    /// The programmes airing now, which the Live TV section stands on.
    OnNow,
    /// The Latest rail of the library named, at what that library's own
    /// collection type asks for.
    Latest {
        library: uuid::Uuid,
        limit: Limit,
    },
}

impl Section {
    /// Every rail that rests on nothing the library list carries: the resumed
    /// rails and Next Up where the arrangement draws them, and On Now where the
    /// session reaches Live TV.
    // reference: home-live-tv-airing
    pub fn drawn(arrangement: &Arrangement) -> Vec<Section> {
        let mut drawn = Section::stale(arrangement);
        if arrangement.live_tv.allowed() {
            drawn.push(Section::OnNow);
        }
        drawn
    }

    /// The rails a played mark or a cleared resume position makes stale, each
    /// only where the arrangement draws it.
    pub fn stale(arrangement: &Arrangement) -> Vec<Section> {
        let mut stale = Vec::new();
        if arrangement.continue_watching {
            stale.push(Section::Resumed);
        }
        if arrangement.next_up {
            stale.push(Section::NextUp);
        }
        stale
    }

    /// What a failure to read this rail is reported as.
    pub fn unread(self) -> Text {
        match self {
            Section::Resumed | Section::NextUp => Text::FailureHomeUnread,
            Section::OnNow => Text::FailureProgramsUnread,
            Section::Latest { .. } => Text::FailureLatestUnread,
        }
    }
}

/// What the home screen holds: the library list and every rail, each awaited
/// until its own request answers.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub libraries: Arrival<BaseItemDto>,
    pub continue_watching: Arrival<BaseItemDto>,
    pub next_up: Arrival<BaseItemDto>,
    /// The programmes airing now, which the Live TV section stands on.
    pub on_now: Arrival<BaseItemDto>,
    /// One row per library `latest_shown` admits, in the library order.
    pub latest: Vec<Latest>,
}

impl State {
    // a row whose library the list still carries keeps what it holds
    /// Takes the library list, standing up one Latest row per library
    /// `latest_shown` admits that names an id, in the library order.
    /// Answers every Latest rail the screen holds after it.
    pub fn took_libraries(
        &mut self,
        libraries: Vec<BaseItemDto>,
        arrangement: &Arrangement,
    ) -> Vec<Section> {
        self.latest = libraries
            .iter()
            .filter(|library| latest_shown(library, &arrangement.latest_excluded))
            .filter_map(|library| Some((library, library.id?)))
            .map(|(library, id)| Latest {
                library: library.clone(),
                id,
                items: match self.latest.iter().find(|row| row.id == id) {
                    Some(row) => row.items.clone(),
                    None => Arrival::Awaited,
                },
            })
            .collect();
        self.libraries = Arrival::Arrived(libraries);
        self.latest
            .iter()
            .map(|row| Section::Latest {
                library: row.id,
                limit: latest_limit(row.library.collection_type),
            })
            .collect()
    }

    // an answer for a Latest row this screen does not hold is dropped
    /// Takes one rail's answer.
    pub fn took(&mut self, section: Section, items: Vec<BaseItemDto>) {
        match section {
            Section::Resumed => self.continue_watching = Arrival::Arrived(items),
            Section::NextUp => self.next_up = Arrival::Arrived(items),
            Section::OnNow => self.on_now = Arrival::Arrived(items),
            Section::Latest { library, .. } => {
                if let Some(row) = self.latest.iter_mut().find(|row| row.id == library) {
                    row.items = Arrival::Arrived(items);
                }
            }
        }
    }

    /// Every item the screen draws, for a live refresh to mark in place.
    pub fn items_mut(&mut self) -> Vec<&mut BaseItemDto> {
        self.libraries
            .held_mut()
            .iter_mut()
            .chain(self.continue_watching.held_mut().iter_mut())
            .chain(self.next_up.held_mut().iter_mut())
            .chain(self.on_now.held_mut().iter_mut())
            .chain(
                self.latest
                    .iter_mut()
                    .flat_map(|row| row.items.held_mut().iter_mut()),
            )
            .collect()
    }
}

/// One Latest row, which stands only for a library the server named an id for.
#[derive(Debug, Clone)]
pub struct Latest {
    pub library: BaseItemDto,
    pub id: uuid::Uuid,
    pub items: Arrival<BaseItemDto>,
}

/// The most items one Latest row shows: thirty for a music library and sixteen
/// for every other. The reference scrolls its home rows sideways always, so the
/// three limits under its other branch stand unreached.
// reference: home-latest-query
// reference: home-scroll-x
fn latest_limit(collection: Option<CollectionType>) -> Limit {
    match collection {
        Some(CollectionType::Music) => Limit::of(30),
        _ => Limit::of(16),
    }
}

/// What one rail's own request answers.
pub async fn requested(api: Rc<Api>, section: Section) -> Answer<Vec<BaseItemDto>> {
    match section {
        Section::Resumed => api.continue_watching().await,
        Section::NextUp => api.next_up().await,
        Section::OnNow => api.airing_now(ON_NOW).await,
        Section::Latest { library, limit } => api.latest(library, limit).await,
    }
}

/// What the home screen shows: the library order, the libraries hidden, the
/// two rows, and what Live TV the session offers.
#[derive(Debug, Clone, PartialEq)]
pub struct Arrangement {
    pub order: Vec<uuid::Uuid>,
    pub hidden: Vec<uuid::Uuid>,
    /// The libraries the account's own settings draw no Latest row for.
    pub latest_excluded: Vec<uuid::Uuid>,
    pub continue_watching: bool,
    pub next_up: bool,
    /// What the reference's own `EnableLiveTvAccess` gate reads.
    pub live_tv: jellium_protocol::LiveTvAccess,
}

impl Arrangement {
    /// What the user configuration and the preference bag ask of the home
    /// screen.
    pub fn of(
        configuration: &jellium_model::form::Form,
        held: jellium_model::prefs::Held,
        live_tv: jellium_protocol::LiveTvAccess,
    ) -> Arrangement {
        Arrangement {
            order: jellium_model::user::ids(configuration, jellium_model::user::ORDERED_VIEWS),
            hidden: jellium_model::user::ids(configuration, jellium_model::user::MY_MEDIA_EXCLUDES),
            latest_excluded: jellium_model::user::ids(
                configuration,
                jellium_model::user::LATEST_ITEMS_EXCLUDES,
            ),
            continue_watching: held.continue_watching,
            next_up: held.next_up,
            live_tv,
        }
    }
}

/// The resumed items grouped the way the reference's own `Resume`,
/// `ResumeAudio` and `ResumeBook` sections group them, in that order. The
/// reference reaches those three through three requests where this client holds
/// one list, so this splits what is already fetched by the media type each item
/// carries, and a group the list holds nothing for draws no section at all.
/// An item whose media type none of the three names is drawn by no section,
/// because the reference asks for none.
pub fn resumed(items: &[BaseItemDto]) -> Vec<(Resumed, Vec<&BaseItemDto>)> {
    let mut groups: Vec<(Resumed, Vec<&BaseItemDto>)> = Vec::new();
    for section in [Resumed::Watching, Resumed::Listening, Resumed::Reading] {
        let held: Vec<&BaseItemDto> = items
            .iter()
            .filter(|item| Resumed::of(item.media_type) == Some(section))
            .collect();
        if held.is_empty() {
            continue;
        }
        groups.push((section, held));
    }
    groups
}

/// The screen a library tile opens, which the view's own collection type
/// decides: a box-set view opens the Collections screen and a playlist view the
/// Playlists screen, both of which this client draws itself, and every other
/// view opens the library screen `id` names.
fn opens(library: &BaseItemDto, id: uuid::Uuid) -> Route {
    match library.collection_type {
        Some(CollectionType::Boxsets) => Route::Collections,
        Some(CollectionType::Playlists) => Route::Playlists,
        _ => Route::Library {
            id,
            tab: Box::new(crate::screen::library::Tab::Items(Box::default())),
        },
    }
}

// reference: home-sections
/// The sections a default user sees, in the order the server's own defaults put
/// them: the library tiles in the arrangement's order, what is resumed, what
/// Live TV offers and what is on now, what is next up, and what is latest in
/// each library. A section the arrangement turns off is absent rather than
/// empty, and so is a rail holding no items, whether its request has answered
/// or not.
pub fn view<'a>(
    state: &'a State,
    arrangement: &'a Arrangement,
    now: chrono::DateTime<chrono::Utc>,
    viewport: Viewport,
    images: &'a Cache,
    session: &'a Session,
) -> Element<'a, Message> {
    // the empty heading stands once the library list has answered nothing, and
    // not before
    if let Arrival::Arrived(libraries) = &state.libraries
        && libraries.is_empty()
    {
        return construct::silent(
            Construct::CenterMessage,
            column![
                construct::stated(
                    Construct::CenterMessageH2,
                    Said::Plain(Text::HomeEmptyHeading),
                    widget::centered(strings::lookup(Text::HomeEmptyHeading).to_string()),
                ),
                construct::stated(
                    Construct::CenterMessageP,
                    Said::Plain(empty_paragraph(session)),
                    widget::centered(strings::lookup(empty_paragraph(session)).to_string()),
                ),
            ]
            .into(),
        );
    }

    let mut page = column![];

    let libraries = shown(state, arrangement);
    if !libraries.is_empty() {
        page = page.push(widget::section(
            titled(Text::HomeMyMedia),
            widget::scroller(
                widget::TILE,
                widget::Rail::of(Construct::ItemsContainer),
                widget::stepping(Room::content(viewport)),
                Room::content(viewport),
                libraries.iter().filter_map(|library| {
                    Some(widget::library_tile(
                        library,
                        Room::content(viewport),
                        images,
                        Message::Navigated(opens(library, library.id?)),
                    ))
                }),
            ),
        ));
    }

    if arrangement.continue_watching {
        for (section, items) in resumed(state.continue_watching.held()) {
            page = page.push(widget::section(
                titled(section.label()),
                widget::rail(
                    railed(
                        card::Card::resumed(resumed_media(section)),
                        // reference: home-resume
                        card::Footer::of(
                            card::Parent::Shown,
                            card::Title::Shown,
                            &[card::Line::Year],
                        )
                        .lines(card::Lines::TWO),
                    ),
                    widget::Rail::of(Construct::ItemsContainer),
                    items,
                    Room::content(viewport),
                    images,
                    now,
                    session,
                ),
            ));
        }
    }
    // reference: home-live-tv
    // reference: home-live-tv-airing
    if !state.on_now.held().is_empty() {
        page = page.push(widget::section(
            titled(Text::HomeLiveTv),
            // reference: home-live-tv-sections
            row(crate::screen::livetv::Tab::ALL.iter().map(|tab| {
                construct::navigation(
                    Construct::Raised,
                    Some(Said::Plain(tab.label())),
                    Message::Navigated(Route::LiveTv { tab: *tab }),
                    widget::block(strings::lookup(tab.label()), None, widget::Emphasis::Raised),
                )
            }))
            .into(),
        ));
        page = page.push(widget::section(
            opened(
                Said::Plain(Text::HomeOnNow),
                strings::lookup(Text::HomeOnNow).to_owned(),
                Message::Navigated(crate::screen::livetv::programs::opens()),
            ),
            widget::rail(
                railed(
                    jellium_model::livetv::Section::OnNow.card(),
                    // reference: home-on-now-cards
                    card::Footer::of(
                        card::Parent::OrTitle,
                        card::Title::Shown,
                        &[card::Line::AirTime(card::AirTime::Ended)],
                    )
                    .lines(card::Lines::THREE),
                ),
                widget::Rail::of(Construct::ItemsContainer),
                state.on_now.held().iter(),
                Room::content(viewport),
                images,
                now,
                session,
            ),
        ));
    }

    if arrangement.next_up && !state.next_up.held().is_empty() {
        page = page.push(widget::section(
            opened(
                Said::Plain(Text::HomeNextUp),
                strings::lookup(Text::HomeNextUp).to_owned(),
                Message::Navigated(Route::Home { tab: Tab::Home }),
            ),
            widget::rail(
                railed(
                    card::Card::NEXT_UP,
                    // reference: home-next-up
                    card::Footer::of(card::Parent::Shown, card::Title::Shown, &[]),
                ),
                widget::Rail::of(Construct::ItemsContainer),
                state.next_up.held().iter(),
                Room::content(viewport),
                images,
                now,
                session,
            ),
        ));
    }
    for row in state
        .latest
        .iter()
        .filter(|row| !row.items.held().is_empty())
    {
        page = page.push(widget::section(
            opened(
                Said::Filled(Template::HomeLatest),
                strings::format(
                    Template::HomeLatest,
                    &[row.library.name.as_deref().unwrap_or_default()],
                ),
                Message::Navigated(opens(&row.library, row.id)),
            ),
            widget::rail(
                railed(
                    card::Card::latest(row.library.collection_type),
                    card::Footer::latest(row.library.collection_type),
                ),
                widget::Rail::within(Construct::ItemsContainer, row.id),
                row.items.held().iter(),
                Room::content(viewport),
                images,
                now,
                session,
            ),
        ));
    }
    crate::widget::scrolled(page).into()
}

/// The library views My Media draws: the arrangement's own order, with the
/// views it hides dropped.
pub fn shown<'a>(state: &'a State, arrangement: &Arrangement) -> Vec<&'a BaseItemDto> {
    let ids: Vec<uuid::Uuid> = state
        .libraries
        .held()
        .iter()
        .filter_map(|it| it.id)
        .collect();
    jellium_model::user::arranged(&ids, &arrangement.order, &arrangement.hidden)
        .iter()
        .filter_map(|id| state.libraries.held().iter().find(|it| it.id == Some(*id)))
        .collect()
}

pub fn images(state: &State, arrangement: &Arrangement) -> images::Wanted {
    let mut keys = widget::card_images(shown(state, arrangement), widget::TILE.card);
    keys.extend(
        state
            .continue_watching
            .held()
            .iter()
            .filter_map(|item| widget::posted(item, card::Card::resumed(item.media_type))),
    );
    keys.extend(widget::card_images(
        state.next_up.held(),
        card::Card::NEXT_UP,
    ));
    for row in &state.latest {
        keys.extend(widget::card_images(
            row.items.held(),
            card::Card::latest(row.library.collection_type),
        ));
    }
    keys.extend(widget::card_images(
        state.on_now.held(),
        jellium_model::livetv::Section::OnNow.card(),
    ));
    keys
}
