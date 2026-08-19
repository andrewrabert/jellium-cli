use std::rc::Rc;

use iced::Element;
use iced::widget::{column, row};
use jellium_protocol::Session;
use jellyfin_api::types::{BaseItemDto, CollectionType, MediaType};

use crate::api::Api;
use crate::app::Message;
use crate::construct;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::livetv::Channel;
use crate::route::Route;
use crate::screen::arrival::Arrival;
use crate::style::space::Room;
use crate::style::{self, Viewport, card};
use crate::text::{self as strings, Text};
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
        said,
        widget::prose(strings::lookup(said), style::typeface::HEADING_2),
    )
}

/// One section title the reference wraps in a link, which carries the trailing
/// chevron `.sectionTitleTextButton` writes and opens what that link opens.
fn opened<'a>(said: Text, spoken: String, opens: Message) -> Element<'a, Message> {
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
                Some(tab.label()),
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

/// One home rail's card: the shape the section asks for, over the two lines a
/// rail writes under it.
// reference: card-box-classes
fn railed(card: card::Card) -> card::Drawing {
    card::Drawing {
        card,
        footer: card::Footer::NameAndSubtitle,
        backing: card::Backing::Padder,
        footing: card::Footing::Bare,
        setting: card::Setting::Centred,
        bottom: card::Bottom::Padded,
        // reference: home-resume
        // reference: home-next-up
        // reference: home-latest
        touch: card::Touch::Plays,
    }
}

/// The most channels the on-now row shows.
pub const ON_NOW: i32 = 20;

/// Which home rail one answer fills.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Section {
    /// The three resumed rails, which this client reaches in one request where
    /// the reference reaches them in three.
    Resumed,
    NextUp,
    /// The Latest rail of the library named.
    Latest(uuid::Uuid),
}

impl Section {
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
            Section::Latest(_) => Text::FailureLatestUnread,
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub libraries: Vec<BaseItemDto>,
    pub continue_watching: Arrival<BaseItemDto>,
    pub next_up: Arrival<BaseItemDto>,
    /// One row per library `latest_shown` admits, in the library order.
    pub latest: Vec<Latest>,
    /// The user's favourite channels first and then channels in number order,
    /// capped at `ON_NOW`.
    pub on_now: Arrival<Channel>,
}

impl State {
    /// The screen the libraries answered stand up: every rail awaited, and one
    /// Latest row per library `latest_shown` admits, in the library order.
    pub fn of(libraries: Vec<BaseItemDto>, arrangement: &Arrangement) -> State {
        let latest = libraries
            .iter()
            .filter(|library| latest_shown(library, &arrangement.latest_excluded))
            .map(|library| Latest {
                library: library.clone(),
                items: Arrival::Awaited,
            })
            .collect();
        State {
            libraries,
            continue_watching: Arrival::Awaited,
            next_up: Arrival::Awaited,
            latest,
            on_now: Arrival::Awaited,
        }
    }

    // an answer for a Latest row this screen does not hold is dropped
    /// Takes one rail's answer.
    pub fn took(&mut self, section: Section, items: Vec<BaseItemDto>) {
        match section {
            Section::Resumed => self.continue_watching = Arrival::Arrived(items),
            Section::NextUp => self.next_up = Arrival::Arrived(items),
            Section::Latest(library) => {
                if let Some(row) = self
                    .latest
                    .iter_mut()
                    .find(|row| row.library.id == Some(library))
                {
                    row.items = Arrival::Arrived(items);
                }
            }
        }
    }

    /// Every rail this screen requests: the ones the arrangement draws, and one
    /// per Latest row it holds.
    pub fn sections(&self, arrangement: &Arrangement) -> Vec<Section> {
        let mut asked = Section::stale(arrangement);
        asked.extend(
            self.latest
                .iter()
                .filter_map(|row| row.library.id)
                .map(Section::Latest),
        );
        asked
    }
}

/// One Latest row.
#[derive(Debug, Clone)]
pub struct Latest {
    pub library: BaseItemDto,
    pub items: Arrival<BaseItemDto>,
}

/// The most items one Latest row shows.
pub const LATEST: i32 = 16;

/// What one rail's own request answers.
pub async fn requested(api: Rc<Api>, section: Section) -> Answer<Vec<BaseItemDto>> {
    match section {
        Section::Resumed => api.continue_watching().await,
        Section::NextUp => api.next_up().await,
        Section::Latest(library) => api.latest(library, LATEST).await,
    }
}

/// The channels the on-now row shows, in one channel query carrying their
/// current programs.
pub async fn on_now(api: Rc<Api>) -> Answer<Vec<Channel>> {
    api.live_tv_channels(jellyfin_api::types::ChannelType::Tv, Some(ON_NOW))
        .await
}

/// What the home screen shows: the library order, the libraries hidden, and the
/// two rows.
#[derive(Debug, Clone, PartialEq)]
pub struct Arrangement {
    pub order: Vec<uuid::Uuid>,
    pub hidden: Vec<uuid::Uuid>,
    /// The libraries the account's own settings draw no Latest row for.
    pub latest_excluded: Vec<uuid::Uuid>,
    pub continue_watching: bool,
    pub next_up: bool,
}

impl Arrangement {
    /// What the user configuration and the preference bag ask of the home
    /// screen.
    pub fn of(
        configuration: &jellium_model::form::Form,
        held: jellium_model::prefs::Held,
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
/// view opens the library screen its id names. A view carrying no id opens
/// nothing.
fn opens(library: &BaseItemDto) -> Option<Route> {
    match library.collection_type {
        Some(CollectionType::Boxsets) => Some(Route::Collections),
        Some(CollectionType::Playlists) => Some(Route::Playlists),
        _ => Some(Route::Library {
            id: library.id?,
            tab: Box::new(crate::screen::library::Tab::Items(Box::default())),
        }),
    }
}

// reference: home-sections
/// The sections a default user sees, in the order the server's own defaults put
/// them: the library tiles in the arrangement's order, what is resumed, what
/// Live TV offers and what is on now, what is next up, and what is latest in
/// each library. A section the arrangement turns off is absent rather than
/// empty, and so is a rail whose own request has not answered.
pub fn view<'a>(
    state: &'a State,
    arrangement: &'a Arrangement,
    now: chrono::DateTime<chrono::Utc>,
    viewport: Viewport,
    images: &'a Cache,
    session: &'a Session,
) -> Element<'a, Message> {
    if state.libraries.is_empty() {
        return construct::silent(
            Construct::CenterMessage,
            column![
                construct::stated(
                    Construct::CenterMessageH2,
                    Text::HomeEmptyHeading,
                    widget::centered(strings::lookup(Text::HomeEmptyHeading).to_string()),
                ),
                construct::stated(
                    Construct::CenterMessageP,
                    empty_paragraph(session),
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
                        Message::Navigated(opens(library)?),
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
                    railed(card::Card::resumed(resumed_media(section))),
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
    if !state.on_now.held().is_empty() {
        // reference: home-live-tv
        page = page.push(widget::section(
            titled(Text::HomeLiveTv),
            // reference: home-live-tv-sections
            row(crate::screen::livetv::Tab::ALL.iter().map(|tab| {
                construct::navigation(
                    Construct::Raised,
                    Some(tab.label()),
                    Message::Navigated(Route::LiveTv { tab: *tab }),
                    widget::block(strings::lookup(tab.label()), None, widget::Emphasis::Raised),
                )
            }))
            .into(),
        ));
        page = page.push(widget::section(
            opened(
                Text::HomeOnNow,
                strings::lookup(Text::HomeOnNow).to_owned(),
                Message::Navigated(Route::LiveTv {
                    tab: crate::screen::livetv::Tab::Guide,
                }),
            ),
            widget::on_now_row(state.on_now.held(), Room::content(viewport), now, images),
        ));
    }

    if arrangement.next_up && !state.next_up.held().is_empty() {
        page = page.push(widget::section(
            opened(
                Text::HomeNextUp,
                strings::lookup(Text::HomeNextUp).to_owned(),
                Message::Navigated(Route::Home { tab: Tab::Home }),
            ),
            widget::rail(
                railed(card::Card::NEXT_UP),
                widget::Rail::of(Construct::ItemsContainer),
                state.next_up.held().iter(),
                Room::content(viewport),
                images,
                now,
                session,
            ),
        ));
    }
    for row in &state.latest {
        page = page.push(widget::section(
            opened(
                Text::HomeLatest,
                strings::format(
                    Text::HomeLatest,
                    &[row.library.name.as_deref().unwrap_or_default()],
                ),
                match opens(&row.library) {
                    Some(route) => Message::Navigated(route),
                    None => Message::Unchanged,
                },
            ),
            widget::rail(
                railed(card::Card::latest(row.library.collection_type)),
                widget::Rail::within(
                    Construct::ItemsContainer,
                    row.library.id.unwrap_or_default(),
                ),
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
    let ids: Vec<uuid::Uuid> = state.libraries.iter().filter_map(|it| it.id).collect();
    jellium_model::user::arranged(&ids, &arrangement.order, &arrangement.hidden)
        .iter()
        .filter_map(|id| state.libraries.iter().find(|it| it.id == Some(*id)))
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
    keys.extend(state.on_now.held().iter().map(|channel| {
        images::Poster::of(images::Key {
            item: channel.id,
            kind: images::Kind::Primary,
            index: None,
            card: widget::ON_NOW.card,
        })
    }));
    keys
}
