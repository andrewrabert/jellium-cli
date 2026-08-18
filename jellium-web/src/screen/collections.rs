use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, row, text_input};
use jellium_model::paged::Paged;
use jellium_model::window;
use jellium_protocol::Session;
use jellyfin_api::types::{BaseItemDto, CollectionType};
use uuid::Uuid;

use crate::api::Api;
use crate::app::{Message, Signed};
use crate::error::{Answer, Operation};
use crate::images::{self, Cache};
use crate::route::Listing;
use crate::screen::browse::{self, Browse};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};

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

#[derive(Debug, Clone, PartialEq)]
pub enum Action {
    Typed(String),
    Create,
    Remove { collection: Uuid, item: Uuid },
}

pub async fn listed(api: Rc<Api>, viewport: Viewport) -> Answer<Listed> {
    Answer::of(async {
        let heading = strings::lookup(Text::NavCollections).to_string();
        let mut browse = Browse::new(
            window::Id::Browse,
            heading,
            Listing::default(),
            Some(CollectionType::Boxsets),
            viewport,
        );
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

/// The name control and the control that applies it.
fn naming<'a>(held: &'a str, label: Text, apply: Message) -> Element<'a, Message> {
    row![
        text_input("", held)
            .style(style::input)
            .on_input(|typed| Message::CollectionAction(Action::Typed(typed)))
            .padding(style::drawn(space::CONTROL_GAP.drawn())),
        button(prose(strings::lookup(label), typeface::BODY))
            .style(style::submit)
            .on_press(apply),
    ]
    .spacing(style::drawn(space::CONTROL_GAP.drawn()))
    .align_y(iced::Alignment::Center)
    .into()
}

pub fn view_listed<'a>(
    state: &'a Listed,
    viewport: Viewport,
    images: &'a Cache,
    now: chrono::DateTime<chrono::Utc>,
    session: &'a Session,
) -> Element<'a, Message> {
    let mut page = column![].spacing(style::drawn(space::SECTION_GAP.drawn()));
    if !session.read_only {
        page = page.push(naming(
            &state.naming,
            Text::CollectionCreate,
            Message::CollectionAction(Action::Create),
        ));
    }
    page.push(browse::view(
        &state.browse,
        viewport,
        images,
        now,
        session,
        None,
    ))
    .into()
}

/// Applies one control, and re-reads the surface the write changed.
pub fn act(signed: &mut Signed, action: Action) -> Task<Message> {
    let api = signed.api.clone();
    match action {
        Action::Typed(typed) => {
            if let crate::app::View::Collections(listed) = &mut signed.view {
                listed.naming = typed;
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
        Action::Remove { collection, item } => Task::perform(
            async move { api.remove_from_collection(collection, &[item]).await },
            |wrote| Message::Wrote(Operation::CollectionRemove, wrote),
        ),
    }
}

pub fn listed_images(state: &Listed) -> images::Wanted {
    browse::images(&state.browse)
}
