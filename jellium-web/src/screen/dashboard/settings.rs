//! The server's configuration, one section per screen, read whole and written
//! whole.

use iced::Element;
use iced::widget::column;

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, Viewport, space};
use crate::text::{self as strings, Text};
use crate::widget::{self, Choice, Emphasis, Secrecy};
use jellium_model::form::{Field, Form};

use super::{Controls, Group, frame};

/// One configuration screen: the section it edits, held as the server answered
/// it with the edits made against it.
#[derive(Debug, Clone)]
pub struct State {
    pub section: super::Section,
    pub form: Form,
    /// True once a save has landed and nothing has been edited since.
    pub saved: bool,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>, section: super::Section) -> Answer<State> {
    Answer::of(async {
        let read = match section.key() {
            Some(key) => api.section(key).await.bubbled()?,
            None => api.server_configuration().await.bubbled()?,
        };
        Ok(State {
            section,
            form: Form::of(read),
            saved: false,
        })
    })
    .await
}

/// What editing `field` to `value` sends.
fn edited(field: Field, value: String) -> Message {
    Message::DashboardAction(super::Action::Edited(field, value))
}

/// The options a `Field::Choice` offers, and the one standing now.
fn offered(
    options: &'static [&'static str],
    held: &str,
) -> (Vec<Choice<&'static str>>, &'static str) {
    let standing = options
        .iter()
        .find(|option| **option == held)
        .copied()
        .unwrap_or_default();
    let offered = options
        .iter()
        .map(|option| Choice {
            label: (*option).to_owned(),
            value: *option,
        })
        .collect();
    (offered, standing)
}

/// MUI's own control for `field`, carrying what the form holds for it.
fn filled<'a>(field: Field, form: &'a Form, viewport: Viewport) -> Element<'a, Message> {
    let held = form.value(field);
    let band = viewport.band();
    match field {
        Field::Flag { .. } => widget::mui::flag(
            field.key(),
            held == "true",
            move |on| edited(field, on.to_string()),
            band,
        ),
        Field::Choice { options, .. } => {
            let (offered, standing) = offered(options, &held);
            widget::mui::chosen(
                field.key(),
                offered,
                &standing,
                move |option: &'static str| edited(field, option.to_owned()),
                viewport,
            )
        }
        Field::Text { .. }
        | Field::Number { .. }
        | Field::Lines { .. }
        | Field::Listed { .. }
        | Field::Named { .. } => {
            widget::mui::field(field.key(), &held, move |typed| edited(field, typed), band)
        }
    }
}

/// One group of a MUI section: its heading, then its controls.
fn group<'a>(group: Group, form: &'a Form, viewport: Viewport) -> Vec<Element<'a, Message>> {
    let mut standing: Vec<Element<'a, Message>> = Vec::new();
    if let Some(heading) = group.heading {
        standing.push(widget::mui::heading(
            heading.rank,
            strings::lookup(heading.title),
        ));
    }
    standing.extend(
        group
            .fields
            .iter()
            .map(|field| filled(*field, form, viewport)),
    );
    standing
}

/// The reference's own control for `field`, which its legacy views draw.
fn emby<'a>(field: Field, form: &'a Form) -> Element<'a, Message> {
    let held = form.value(field);
    match field {
        Field::Flag { .. } => widget::flag(field.key(), None, held == "true", move |on| {
            edited(field, on.to_string())
        }),
        Field::Choice { options, .. } => {
            let (offered, standing) = offered(options, &held);
            widget::select(
                field.key(),
                None,
                offered,
                &standing,
                move |option: &'static str| edited(field, option.to_owned()),
            )
        }
        Field::Text { .. }
        | Field::Number { .. }
        | Field::Lines { .. }
        | Field::Listed { .. }
        | Field::Named { .. } => widget::field(
            field.key(),
            &held,
            None,
            move |typed| edited(field, typed),
            Message::Unchanged,
            Secrecy::Shown,
        ),
    }
}

/// One group of a legacy section, which the reference writes as a `fieldset`.
fn fieldset<'a>(group: Group, form: &'a Form) -> Element<'a, Message> {
    let rows: Vec<Element<'a, Message>> = group
        .fields
        .iter()
        .map(|field| emby(*field, form))
        .collect();
    match group.heading {
        Some(heading) => widget::fields(heading.rank, heading.title, rows),
        None => column(rows)
            .spacing(style::drawn(space::FIELD_GAP.drawn()))
            .into(),
    }
}

/// The section's fields in their groups, each group under the heading it
/// carries, the notice a landed save raises above them, and the save control
/// at the foot.
// reference: dashboard-content
pub fn view<'a>(state: &'a State, read_only: bool, viewport: Viewport) -> frame::Filling<'a> {
    let controls = state.section.controls();
    let band = viewport.band();
    let mut rows: Vec<Element<'a, Message>> = Vec::new();

    for held in state.section.groups() {
        match controls {
            Controls::Mui => rows.extend(group(*held, &state.form, viewport)),
            Controls::Emby => rows.push(fieldset(*held, &state.form)),
        }
    }

    if state.saved {
        rows.push(widget::mui::succeeded(Text::DashboardSaved, band));
    }

    if !read_only {
        let press = state
            .form
            .dirty()
            .then_some(Message::DashboardAction(super::Action::Save));
        rows.push(match controls {
            Controls::Mui => widget::mui::contained(Text::DashboardSave, press, band),
            Controls::Emby => widget::block(
                strings::lookup(Text::DashboardSave),
                press,
                Emphasis::Submit,
            ),
        });
    }

    frame::Filling::Capped(rows)
}
