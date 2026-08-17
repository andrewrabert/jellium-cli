use iced::Element;
use iced::widget::{button, checkbox, column, row, text_input};
use jellium_model::item;

use crate::app::Message;
use crate::text::{self as strings, Text};

use super::{Action, State};
use crate::style::{self, space, typeface};
use crate::widget::prose;

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
    let mut page = column![].spacing(style::drawn(space::GUTTER.drawn()));

    for field in item::fields_of(state.item.type_) {
        let held = state.form.value(field);
        let mut line = row![prose(label(field), typeface::BODY)]
            .spacing(style::drawn(space::GUTTER.drawn()))
            .align_y(iced::Alignment::Center);

        line = line.push(if read_only {
            Element::from(prose(held, typeface::BODY))
        } else {
            text_input("", &held)
                .style(style::input)
                .on_input(move |value| Message::MetadataAction(Action::Edited(field, value)))
                .padding(style::drawn(space::CONTROL_GAP.drawn()))
                .into()
        });

        if let Some(lock) = item::lock_of(field) {
            let on = item::locked(&state.form, lock);
            line = line.push(if read_only {
                Element::from(prose(
                    strings::lookup(if on {
                        Text::MetadataLocked
                    } else {
                        Text::MetadataUnlocked
                    })
                    .to_owned(),
                    typeface::BODY,
                ))
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
    let mut page = column![prose(
        strings::lookup(Text::MetadataPeople),
        typeface::HEADING_3
    )]
    .spacing(style::drawn(space::GUTTER.drawn()));

    for (at, person) in state.people.iter().enumerate() {
        if read_only {
            page = page.push(prose(
                format!("{} — {} {}", person.name, person.kind, person.role),
                typeface::BODY,
            ));
            continue;
        }

        let held = person.clone();
        let by_name = held.clone();
        let by_kind = held.clone();
        let by_role = held;

        page = page.push(
            row![
                text_input("", &person.name)
                    .style(style::input)
                    .on_input(move |value| {
                        let mut edited = by_name.clone();
                        edited.name = value;
                        Message::MetadataAction(Action::PersonEdited { at, person: edited })
                    })
                    .padding(style::drawn(space::CONTROL_GAP.drawn())),
                text_input("", &person.kind)
                    .style(style::input)
                    .on_input(move |value| {
                        let mut edited = by_kind.clone();
                        edited.kind = value;
                        Message::MetadataAction(Action::PersonEdited { at, person: edited })
                    })
                    .padding(style::drawn(space::CONTROL_GAP.drawn())),
                text_input("", &person.role)
                    .style(style::input)
                    .on_input(move |value| {
                        let mut edited = by_role.clone();
                        edited.role = value;
                        Message::MetadataAction(Action::PersonEdited { at, person: edited })
                    })
                    .padding(style::drawn(space::CONTROL_GAP.drawn())),
                button(prose(
                    strings::lookup(Text::MetadataPersonRemove),
                    typeface::BODY
                ))
                .on_press(Message::MetadataAction(Action::PersonRemoved { at })),
            ]
            .spacing(style::drawn(space::GUTTER.drawn()))
            .align_y(iced::Alignment::Center),
        );
    }

    if !read_only {
        page = page.push(
            button(prose(
                strings::lookup(Text::MetadataPersonAdd),
                typeface::BODY,
            ))
            .on_press(Message::MetadataAction(Action::PersonAdded)),
        );
    }

    page.into()
}

/// The provider id control: one row per provider the server named.
pub fn providers<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut page = column![prose(
        strings::lookup(Text::MetadataProviders),
        typeface::HEADING_3
    )]
    .spacing(style::drawn(space::GUTTER.drawn()));

    for (at, (provider, id)) in state.providers.iter().enumerate() {
        let mut line = row![prose(provider.clone(), typeface::BODY)]
            .spacing(style::drawn(space::GUTTER.drawn()))
            .align_y(iced::Alignment::Center);
        line = line.push(if read_only {
            Element::from(prose(id.clone(), typeface::BODY))
        } else {
            text_input("", id)
                .style(style::input)
                .on_input(move |value| {
                    Message::MetadataAction(Action::ProviderEdited { at, id: value })
                })
                .padding(style::drawn(space::CONTROL_GAP.drawn()))
                .into()
        });
        page = page.push(line);
    }

    page.into()
}
