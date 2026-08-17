use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, row, text_input};
use jellium_model::paged::Paged;
use jellium_model::window;
use jellyfin_api::types::BaseItemDto;
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Operation};
use crate::images::{self, Cache};
use crate::route::Listing;
use crate::screen::browse::{self, Browse};
use crate::style::Viewport;
use crate::text::{self as strings, Text};
use crate::theme;

use crate::style::typeface;
use crate::widget::prose;
use iced::Task;

/// The collections destination: every collection, windowed, with the create
/// control absent under read-only.
#[derive(Debug, Clone)]
pub struct Listed {
    pub browse: Browse,
    /// The name typed into the create control.
    pub naming: String,
}

/// One collection: its items as a windowed grid.
#[derive(Debug, Clone)]
pub struct State {
    pub collection: BaseItemDto,
    pub browse: Browse,
    /// The name typed into the rename control.
    pub naming: String,
}

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Typed(String),
    Create,
    Rename { id: Uuid },
    Delete { id: Uuid },
    Remove { collection: Uuid, item: Uuid },
    PlayAll { id: Uuid, shuffle: bool },
}

pub async fn listed(api: Rc<Api>, viewport: Viewport) -> Answer<Listed> {
    Answer::of(async {
        let heading = strings::lookup(Text::NavCollections).to_string();
        let mut browse = Browse::new(window::Id::Browse, heading, Listing::default(), viewport);
        let answered = api
            .collections(0, Paged::<BaseItemDto>::PAGE as i32)
            .await
            .bubbled()?;
        browse.items = Paged::new(answered.total.max(0) as usize);
        browse.filled(0..answered.items.len(), answered.items);

        Ok(Listed {
            browse,
            naming: String::new(),
        })
    })
    .await
}

pub async fn load(
    api: Rc<Api>,
    collection: Uuid,
    listing: Listing,
    viewport: Viewport,
) -> Answer<State> {
    Answer::of(async {
        let held = api.item(collection).await.bubbled()?;
        let heading = held.name.clone().unwrap_or_default();
        let mut browse = Browse::new(window::Id::Browse, heading, listing.clone(), viewport);
        let answered = api
            .browse(
                Some(collection),
                None,
                &listing,
                0,
                Paged::<BaseItemDto>::PAGE as i32,
            )
            .await
            .bubbled()?;
        browse.items = Paged::new(answered.total.max(0) as usize);
        browse.filled(0..answered.items.len(), answered.items);

        Ok(State {
            collection: held,
            browse,
            naming: String::new(),
        })
    })
    .await
}

/// The name control and the control that applies it.
fn naming<'a>(held: &'a str, label: Text, apply: Message) -> Element<'a, Message> {
    row![
        text_input("", held)
            .on_input(|typed| Message::CollectionAction(Action::Typed(typed)))
            .padding(8),
        button(prose(strings::lookup(label).to_owned(), typeface::BODY)).on_press(apply),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Alignment::Center)
    .into()
}

pub fn view_listed<'a>(
    state: &'a Listed,
    images: &'a Cache,
    read_only: bool,
) -> Element<'a, Message> {
    let mut page = column![].spacing(theme::CARD_SPACING);
    if !read_only {
        page = page.push(naming(
            &state.naming,
            Text::CollectionCreate,
            Message::CollectionAction(Action::Create),
        ));
    }
    page.push(browse::view(&state.browse, images, read_only))
        .into()
}

pub fn view<'a>(state: &'a State, images: &'a Cache, read_only: bool) -> Element<'a, Message> {
    let Some(id) = state.collection.id else {
        return column![].into();
    };
    let mut page = column![].spacing(theme::CARD_SPACING);

    if !read_only {
        page = page
            .push(naming(
                &state.naming,
                Text::CollectionRename,
                Message::CollectionAction(Action::Rename { id }),
            ))
            .push(
                row![
                    button(prose(
                        strings::lookup(Text::DetailPlayAll).to_owned(),
                        typeface::BODY
                    ))
                    .on_press(Message::CollectionAction(Action::PlayAll {
                        id,
                        shuffle: false
                    })),
                    button(prose(
                        strings::lookup(Text::DetailShuffle).to_owned(),
                        typeface::BODY
                    ))
                    .on_press(Message::CollectionAction(Action::PlayAll {
                        id,
                        shuffle: true
                    })),
                    button(prose(
                        strings::lookup(Text::CollectionDelete).to_owned(),
                        typeface::BODY
                    ))
                    .on_press(Message::CollectionAction(Action::Delete { id })),
                ]
                .spacing(theme::CARD_SPACING),
            );
    }

    page.push(browse::view(&state.browse, images, read_only))
        .into()
}

/// Applies one control, and re-reads the surface the write changed.
pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    let api = signed.api.clone();
    match action {
        Action::Typed(typed) => {
            match &mut signed.view {
                crate::app::View::Collections(listed) => listed.naming = typed,
                crate::app::View::Collection(state) => state.naming = typed,
                _ => {}
            }
            Task::none()
        }
        Action::Create => {
            let name = match &signed.view {
                crate::app::View::Collections(listed) => listed.naming.clone(),
                _ => return Task::none(),
            };
            if name.trim().is_empty() {
                return Task::none();
            }
            Task::perform(
                async move { api.create_collection(&name, &[]).await.map(|_| ()) },
                |wrote| Message::Wrote(Operation::CollectionCreate, wrote),
            )
        }
        Action::Rename { id } => {
            let name = match &signed.view {
                crate::app::View::Collection(state) => state.naming.clone(),
                _ => return Task::none(),
            };
            if name.trim().is_empty() {
                return Task::none();
            }
            Task::perform(async move { api.rename_item(id, &name).await }, |wrote| {
                Message::Wrote(Operation::CollectionRename, wrote)
            })
        }
        Action::Delete { id } => Task::perform(async move { api.delete_item(id).await }, |wrote| {
            Message::Wrote(Operation::CollectionDelete, wrote)
        }),
        Action::Remove { collection, item } => Task::perform(
            async move { api.remove_from_collection(collection, &[item]).await },
            |wrote| Message::Wrote(Operation::CollectionRemove, wrote),
        ),
        Action::PlayAll { id, shuffle } => {
            Task::done(Message::PlayPressed(crate::player::Intent::All {
                item: id,
                shuffle,
            }))
        }
    }
}

pub fn images(state: &State) -> HashSet<images::Key> {
    browse::images(&state.browse)
}

pub fn listed_images(state: &Listed) -> HashSet<images::Key> {
    browse::images(&state.browse)
}
