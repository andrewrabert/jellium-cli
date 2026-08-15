use iced::Element;
use iced::widget::{button, checkbox, column, row, text, text_input};
use jellium_model::item;

use crate::app::Message;
use crate::text::{self as strings, Text};
use crate::theme;

use super::{Action, State};

/// The label one field is shown under; every spelling is this client's own.
fn label(field: jellium_model::form::Field) -> String {
    let key = field.key();
    let mut shown = String::new();
    for (at, character) in key.char_indices() {
        if character.is_uppercase() && at > 0 {
            shown.push(' ');
        }
        shown.push(character);
    }
    shown
}

/// Every field `state.item`'s kind exposes, each labelled by this client's own
/// text, with a lock control beside the nine Jellyfin models and none beside
/// the rest.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page = column![].spacing(theme::CARD_SPACING);

    for field in item::fields_of(state.item.type_) {
        let held = state.form.value(field);
        let mut line = row![text(label(field))]
            .spacing(theme::CARD_SPACING)
            .align_y(iced::Alignment::Center);

        line = line.push(if read_only {
            Element::from(text(held))
        } else {
            text_input("", &held)
                .on_input(move |value| Message::MetadataAction(Action::Edited(field, value)))
                .padding(8)
                .into()
        });

        if let Some(lock) = item::lock_of(field) {
            let on = item::locked(&state.form, lock);
            line = line.push(if read_only {
                Element::from(text(strings::lookup(if on {
                    Text::MetadataLocked
                } else {
                    Text::MetadataUnlocked
                })))
            } else {
                checkbox(on)
                    .on_toggle(move |on| Message::MetadataAction(Action::Locked(lock, on)))
                    .into()
            });
        }

        page = page.push(line);
    }

    page.into()
}

/// The cast and crew control: one row per person with a name, a kind and a
/// role, added and removed one at a time.
pub fn people<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page =
        column![text(strings::lookup(Text::MetadataPeople)).size(20)].spacing(theme::CARD_SPACING);

    for (at, person) in state.people.iter().enumerate() {
        if read_only {
            page = page.push(text(format!(
                "{} — {} {}",
                person.name, person.kind, person.role
            )));
            continue;
        }

        let held = person.clone();
        let by_name = held.clone();
        let by_kind = held.clone();
        let by_role = held;

        page = page.push(
            row![
                text_input("", &person.name)
                    .on_input(move |value| {
                        let mut edited = by_name.clone();
                        edited.name = value;
                        Message::MetadataAction(Action::PersonEdited { at, person: edited })
                    })
                    .padding(8),
                text_input("", &person.kind)
                    .on_input(move |value| {
                        let mut edited = by_kind.clone();
                        edited.kind = value;
                        Message::MetadataAction(Action::PersonEdited { at, person: edited })
                    })
                    .padding(8),
                text_input("", &person.role)
                    .on_input(move |value| {
                        let mut edited = by_role.clone();
                        edited.role = value;
                        Message::MetadataAction(Action::PersonEdited { at, person: edited })
                    })
                    .padding(8),
                button(text(strings::lookup(Text::MetadataPersonRemove)))
                    .on_press(Message::MetadataAction(Action::PersonRemoved { at })),
            ]
            .spacing(theme::CARD_SPACING)
            .align_y(iced::Alignment::Center),
        );
    }

    if !read_only {
        page = page.push(
            button(text(strings::lookup(Text::MetadataPersonAdd)))
                .on_press(Message::MetadataAction(Action::PersonAdded)),
        );
    }

    page.into()
}

/// The provider id control: one row per provider the server named.
pub fn providers<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page = column![text(strings::lookup(Text::MetadataProviders)).size(20)]
        .spacing(theme::CARD_SPACING);

    for (at, (provider, id)) in state.providers.iter().enumerate() {
        let mut line = row![text(provider.as_str())]
            .spacing(theme::CARD_SPACING)
            .align_y(iced::Alignment::Center);
        line = line.push(if read_only {
            Element::from(text(id.as_str()))
        } else {
            text_input("", id)
                .on_input(move |value| {
                    Message::MetadataAction(Action::ProviderEdited { at, id: value })
                })
                .padding(8)
                .into()
        });
        page = page.push(line);
    }

    page.into()
}
