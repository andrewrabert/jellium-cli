pub mod artwork;
pub mod fields;
pub mod identify;

use std::collections::HashSet;
use std::rc::Rc;

use iced::widget::{button, column, row, scrollable};
use iced::{Element, Task};
use jellium_model::form::Form;
use jellium_model::item;
use jellyfin_api::types::{BaseItemDto, MetadataField};
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Operation};
use crate::images::{self, Cache, Foreign};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

/// One part of the metadata manager.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Part {
    Fields,
    Identify,
    Images,
    Locks,
    ContentType,
    Deletion,
}

impl Part {
    pub const ALL: [Part; 6] = [
        Part::Fields,
        Part::Identify,
        Part::Images,
        Part::Locks,
        Part::ContentType,
        Part::Deletion,
    ];

    pub fn label(self) -> Text {
        match self {
            Part::Fields => Text::MetadataPartFields,
            Part::Identify => Text::MetadataPartIdentify,
            Part::Images => Text::MetadataPartImages,
            Part::Locks => Text::MetadataPartLocks,
            Part::ContentType => Text::MetadataPartContentType,
            Part::Deletion => Text::MetadataPartDeletion,
        }
    }
}

#[derive(Debug, Clone)]
pub struct State {
    pub part: Part,
    pub item: BaseItemDto,
    /// The item read whole, edited by key and written whole.
    pub form: Form,
    pub people: Vec<item::Person>,
    pub providers: Vec<(String, String)>,
    /// The content types the server offers for this item.
    pub content_types: Vec<jellyfin_api::types::NameValuePair>,
    /// The destructive action awaiting its confirmation.
    pub confirming: Option<crate::screen::confirm::Pending>,
    pub identify: identify::State,
    pub artwork: artwork::State,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Open(Part),
    Edited(jellium_model::form::Field, String),
    /// Sets or clears one of the nine locks Jellyfin models.
    Locked(MetadataField, bool),
    PersonEdited {
        at: usize,
        person: item::Person,
    },
    PersonAdded,
    PersonRemoved {
        at: usize,
    },
    ProviderEdited {
        at: usize,
        id: String,
    },
    ContentType(String),
    /// Raises the confirmation a destructive action stands behind.
    Ask(crate::screen::confirm::Pending),
    /// One keystroke of a `Tier::Typed` confirmation.
    Typed(String),
    /// Carries out the confirmed action.
    Confirm,
    /// Re-reads the item and writes it whole, so every field no control covers
    /// survives.
    Save,
    Identify(identify::Action),
    Artwork(artwork::Action),
    Close,
}

pub async fn load(api: Rc<Api>, item: Uuid, part: Part) -> Answer<State> {
    Answer::of(async {
        let held = api.item(item).await.bubbled()?;
        let whole = api.item_whole(item).await.bubbled()?;
        let form = Form::of(whole);

        let content_types = api
            .metadata_editor(item)
            .await
            .map(|editor| editor.content_type_options)
            .or_default(Text::FailureContentTypesUnread);

        let artwork = artwork::State {
            held: api
                .item_images(item)
                .await
                .or_default(Text::FailureItemImagesUnread),
            kind: Some(artwork::Kind::Primary),
            providers: api
                .remote_image_providers(item)
                .await
                .or_default(Text::FailureImageProvidersUnread),
            ..artwork::State::default()
        };

        Ok(State {
            part,
            identify: identify::State {
                name: held.name.clone().unwrap_or_default(),
                year: held
                    .production_year
                    .map(|year| year.to_string())
                    .unwrap_or_default(),
                ..identify::State::default()
            },
            people: item::people(&form),
            providers: item::providers(&form),
            content_types,
            artwork,
            confirming: None,
            form,
            item: held,
        })
    })
    .await
}

/// The part shown beside the six-part column; every control is absent under
/// read-only.
pub fn view<'a>(
    state: &'a State,
    viewport: Viewport,
    images: &'a Cache,
    foreign: &'a Foreign,
    read_only: bool,
) -> Element<'a, Message> {
    let Some(id) = state.item.id else {
        return column![].into();
    };
    let searchable = identify::Search::of(state.item.type_).is_some();

    let parts = column(Part::ALL.into_iter().filter_map(|part| {
        if part == Part::Identify && !searchable {
            return None;
        }
        let mut control =
            button(prose(strings::lookup(part.label()), typeface::BODY)).style(style::flat);
        if part != state.part {
            control = control.on_press(Message::MetadataAction(Action::Open(part)));
        }
        Some(control.into())
    }))
    .spacing(style::drawn(space::CONTROL_GAP.drawn()));

    let body: Element<'a, Message> = match state.part {
        Part::Fields => column![
            fields::view(state, read_only),
            fields::people(state, read_only),
            fields::providers(state, read_only),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .into(),
        Part::Identify => identify::view(&state.identify, viewport, foreign, read_only),
        Part::Images => artwork::view(&state.artwork, viewport, images, foreign, id, read_only),
        Part::Locks => locks(state, read_only),
        Part::ContentType => content_type(state, read_only),
        Part::Deletion => deletion(state, read_only),
    };

    let mut page = column![prose(
        state.item.name.clone().unwrap_or_default(),
        typeface::HEADING_1
    )]
    .spacing(style::drawn(space::GUTTER.drawn()));

    if !read_only && state.part == Part::Fields {
        page = page.push(
            button(prose(strings::lookup(Text::MetadataSave), typeface::BODY))
                .style(style::submit)
                .on_press(Message::MetadataAction(Action::Save)),
        );
    }

    scrollable(
        column![
            page,
            row![parts, body].spacing(style::drawn(space::GUTTER.drawn())),
            button(prose(strings::lookup(Text::MetadataClose), typeface::BODY))
                .style(style::raised)
                .on_press(Message::MetadataAction(Action::Close)),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .padding(style::drawn(space::GUTTER.drawn())),
    )
    .into()
}

/// The nine locks Jellyfin models, each on its own.
fn locks<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let held = item::LOCKS.into_iter().map(|lock| {
        let on = item::locked(&state.form, lock);
        let shown = prose(lock.to_string(), typeface::BODY);
        if read_only {
            return row![
                shown,
                prose(
                    strings::lookup(if on {
                        Text::MetadataLocked
                    } else {
                        Text::MetadataUnlocked
                    })
                    .to_owned(),
                    typeface::BODY
                ),
            ]
            .spacing(style::drawn(space::GUTTER.drawn()))
            .into();
        }
        row![
            iced::widget::checkbox(on)
                .on_toggle(move |on| Message::MetadataAction(Action::Locked(lock, on))),
            shown,
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .align_y(iced::Alignment::Center)
        .into()
    });
    column(held)
        .spacing(style::drawn(space::CONTROL_GAP.drawn()))
        .into()
}

fn content_type<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    if read_only {
        return prose(
            strings::lookup(Text::MetadataPartContentType),
            typeface::BODY,
        );
    }
    let offered = state.content_types.iter().filter_map(|option| {
        let value = option.value.clone()?;
        Some(
            button(prose(
                option.name.clone().unwrap_or_default(),
                typeface::BODY,
            ))
            .style(style::flat)
            .on_press(Message::MetadataAction(Action::ContentType(value)))
            .into(),
        )
    });
    column(offered)
        .spacing(style::drawn(space::CONTROL_GAP.drawn()))
        .into()
}

/// The deletion part: what is lost, and the control that raises the
/// confirmation. Deleting takes the item's own name typed, so no single press
/// removes an item from the library and from disk.
fn deletion<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    if read_only {
        return prose(strings::lookup(Text::MetadataPartDeletion), typeface::BODY);
    }
    if let Some(pending) = &state.confirming {
        return crate::screen::confirm::view(pending, crate::screen::confirm::Region::Metadata);
    }
    let Some(id) = state.item.id else {
        return prose(strings::lookup(Text::MetadataPartDeletion), typeface::BODY);
    };
    let name = state.item.name.clone().unwrap_or_default();
    column![
        prose(strings::lookup(Text::MetadataDeleteWarning), typeface::BODY),
        button(prose(strings::lookup(Text::MetadataDelete), typeface::BODY))
            .style(style::raised)
            .on_press(Message::MetadataAction(Action::Ask(
                crate::screen::confirm::Pending::of(
                    crate::screen::confirm::Destructive::DeleteItem { id },
                    name,
                )
            ))),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .into()
}

pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    let api = signed.api.clone();
    let Some(state) = metadata_mut(signed) else {
        return Task::none();
    };
    let Some(id) = state.item.id else {
        return Task::none();
    };

    match action {
        Action::Open(part) => {
            state.part = part;
            Task::none()
        }
        Action::Edited(field, value) => {
            state.form.edit(field, value);
            Task::none()
        }
        Action::Locked(lock, on) => {
            item::set_locked(&mut state.form, lock, on);
            Task::none()
        }
        Action::PersonEdited { at, person } => {
            if let Some(held) = state.people.get_mut(at) {
                *held = person;
            }
            item::set_people(&mut state.form, &state.people.clone());
            Task::none()
        }
        Action::PersonAdded => {
            state.people.push(item::Person::default());
            item::set_people(&mut state.form, &state.people.clone());
            Task::none()
        }
        Action::PersonRemoved { at } => {
            if at < state.people.len() {
                state.people.remove(at);
            }
            item::set_people(&mut state.form, &state.people.clone());
            Task::none()
        }
        Action::ProviderEdited { at, id: value } => {
            if let Some(held) = state.providers.get_mut(at) {
                held.1 = value;
            }
            item::set_providers(&mut state.form, &state.providers.clone());
            Task::none()
        }
        Action::ContentType(value) => Task::perform(
            async move { api.set_content_type(id, &value).await },
            |wrote| Message::Wrote(Operation::ItemContentType, wrote),
        ),
        Action::Save => {
            let edits = state.form.edits();
            Task::perform(async move { api.save_item(id, &edits).await }, |wrote| {
                Message::Wrote(Operation::ItemSave, wrote)
            })
        }
        Action::Ask(pending) => {
            state.confirming = Some(pending);
            Task::none()
        }
        Action::Typed(typed) => {
            if let Some(pending) = state.confirming.as_mut() {
                pending.typed = typed;
            }
            Task::none()
        }
        Action::Confirm => {
            let Some(pending) = state.confirming.take() else {
                return Task::none();
            };
            if !pending.ready() {
                state.confirming = Some(pending);
                return Task::none();
            }
            match pending.action {
                crate::screen::confirm::Destructive::DeleteItem { id } => {
                    Task::perform(async move { api.delete_item(id).await }, |wrote| {
                        Message::Wrote(Operation::ItemDelete, wrote)
                    })
                }
                _ => Task::none(),
            }
        }
        Action::Close => {
            if state.confirming.take().is_some() {
                return Task::none();
            }
            Task::done(Message::WentBack)
        }
        Action::Identify(held) => identifying(signed, held),
        Action::Artwork(held) => arting(signed, held),
    }
}

fn identifying(signed: &mut Signed, action: identify::Action) -> Task<Message> {
    let api = signed.api.clone();
    let Some(state) = metadata_mut(signed) else {
        return Task::none();
    };
    let Some(id) = state.item.id else {
        return Task::none();
    };
    let Some(search) = identify::Search::of(state.item.type_) else {
        return Task::none();
    };

    match action {
        identify::Action::Typed(field, value) => {
            let held = &mut state.identify;
            match field {
                identify::Field::Name => held.name = value,
                identify::Field::Year => held.year = value,
                identify::Field::Provider => held.provider = value,
                identify::Field::ProviderId => held.provider_id = value,
            }
            Task::none()
        }
        identify::Action::Run => {
            let query = state.identify.query();
            Task::perform(
                async move { api.identify(search, &query).await },
                Message::Identified,
            )
        }
        identify::Action::Choose { at } => {
            state.identify.applying = Some(at);
            state.identify.replace_images = false;
            Task::none()
        }
        identify::Action::SetReplaceImages(on) => {
            state.identify.replace_images = on;
            Task::none()
        }
        identify::Action::Cancel => {
            state.identify.applying = None;
            Task::none()
        }
        identify::Action::Apply => {
            let Some(at) = state.identify.applying else {
                return Task::none();
            };
            let Some(candidate) = state.identify.candidates.get(at).cloned() else {
                return Task::none();
            };
            let replace = state.identify.replace_images;
            state.identify.applying = None;
            Task::perform(
                async move { api.apply_identity(id, &candidate, replace).await },
                |wrote| Message::Wrote(Operation::ItemIdentify, wrote),
            )
        }
    }
}

fn arting(signed: &mut Signed, action: artwork::Action) -> Task<Message> {
    let api = signed.api.clone();
    let Some(state) = metadata_mut(signed) else {
        return Task::none();
    };
    let Some(id) = state.item.id else {
        return Task::none();
    };

    match action {
        artwork::Action::Select(kind) => {
            state.artwork.kind = Some(kind);
            state.artwork.remote.clear();
            Task::none()
        }
        artwork::Action::SelectProvider(provider) => {
            state.artwork.provider = provider;
            Task::none()
        }
        artwork::Action::Search => {
            let Some(kind) = state.artwork.kind else {
                return Task::none();
            };
            let provider = state.artwork.provider.clone();
            Task::perform(
                async move { api.remote_images(id, kind, provider.as_deref()).await },
                Message::RemoteImagesLoaded,
            )
        }
        artwork::Action::Upload => Task::done(Message::MetadataUploadRequested),
        artwork::Action::Remove { kind, index } => Task::perform(
            async move { api.remove_item_image(id, kind, index).await },
            |wrote| Message::Wrote(Operation::ItemImageRemove, wrote),
        ),
        artwork::Action::Move { index, to } => Task::perform(
            async move {
                api.move_item_image(id, artwork::Kind::Backdrop, index, to)
                    .await
            },
            |wrote| Message::Wrote(Operation::ItemImageMove, wrote),
        ),
        artwork::Action::Download { at } => {
            let Some(kind) = state.artwork.kind else {
                return Task::none();
            };
            let Some(handle) = state
                .artwork
                .remote
                .get(at)
                .and_then(|remote| remote.url.clone().or_else(|| remote.thumbnail_url.clone()))
            else {
                return Task::none();
            };
            Task::perform(
                async move { api.download_remote_image(id, kind, &handle).await },
                |wrote| Message::Wrote(Operation::ItemImageDownload, wrote),
            )
        }
    }
}

/// Applies the file the picker answered with: a type outside the four and a
/// file over 4 MiB are named before anything is sent.
pub fn chosen(signed: &mut Signed, chosen: &crate::overlay::Chosen) -> Task<Message> {
    let refused = jellium_model::upload::refused(&chosen.mime, chosen.size);
    let api = signed.api.clone();
    let Some(state) = metadata_mut(signed) else {
        return Task::none();
    };
    if let Some(refused) = &refused {
        crate::failure::raise(crate::error::upload_refused(refused));
        return Task::none();
    }
    let Some(id) = state.item.id else {
        return Task::none();
    };
    let Some(kind) = state.artwork.kind else {
        return Task::none();
    };
    let mime = chosen.mime.clone();
    let bytes = chosen.bytes();
    Task::perform(
        async move { api.upload_item_image(id, kind, &mime, bytes).await },
        |wrote| Message::Wrote(Operation::ItemImageUpload, wrote),
    )
}

/// The metadata manager the view on top holds, and `None` for a view holding
/// none.
fn metadata_mut(signed: &mut Signed) -> Option<&mut State> {
    match &mut signed.view {
        crate::app::View::Metadata(state) => Some(state),
        _ => None,
    }
}

pub fn images(state: &State) -> HashSet<images::Key> {
    let Some(id) = state.item.id else {
        return HashSet::new();
    };
    artwork::images(&state.artwork, id)
}

/// The foreign handles this screen draws.
pub fn handles(state: &State) -> HashSet<String> {
    let mut wanted = identify::handles(&state.identify);
    wanted.extend(artwork::handles(&state.artwork));
    wanted
}
