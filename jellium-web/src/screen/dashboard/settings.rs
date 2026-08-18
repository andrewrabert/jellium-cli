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

use super::{Control, Controls, Group, Offered, frame};

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

/// The options a control offers, and the one standing now.
fn offered(options: &'static [Offered], held: &str) -> (Vec<Choice<&'static str>>, &'static str) {
    let standing = options
        .iter()
        .find(|option| option.value == held)
        .map(|option| option.value)
        .unwrap_or_default();
    let offered = options
        .iter()
        .map(|option| Choice {
            label: strings::lookup(option.label).to_owned(),
            value: option.value,
        })
        .collect();
    (offered, standing)
}

/// MUI's own control for `control`, carrying what the form holds for it.
fn filled<'a>(control: Control, form: &'a Form, viewport: Viewport) -> Element<'a, Message> {
    let field = control.field;
    let held = form.value(field);
    let band = viewport.band();
    match control.offered {
        Some(options) => {
            let (offered, standing) = self::offered(options, &held);
            widget::mui::chosen(
                control.label,
                control.helper,
                offered,
                &standing,
                move |option: &'static str| edited(field, option.to_owned()),
                viewport,
            )
        }
        None => match field {
            Field::Flag { .. } => widget::mui::flag(
                control.label,
                control.helper,
                held == "true",
                move |on| edited(field, on.to_string()),
                band,
            ),
            Field::Text { .. }
            | Field::Number { .. }
            | Field::Choice { .. }
            | Field::Lines { .. }
            | Field::Listed { .. }
            | Field::Named { .. } => widget::mui::field(
                control.label,
                control.helper,
                &held,
                move |typed| edited(field, typed),
                band,
            ),
        },
    }
}

/// One group of a MUI section: its heading, then its controls.
fn group<'a>(group: Group, form: &'a Form, viewport: Viewport) -> Vec<Element<'a, Message>> {
    let band = viewport.band();
    let mut standing: Vec<Element<'a, Message>> = Vec::new();
    if let Some(heading) = group.heading {
        standing.push(widget::mui::heading(
            heading.rank,
            strings::lookup(heading.title),
        ));
    }
    if let Some(note) = group.note {
        standing.push(widget::mui::helper(note, widget::mui::Helper::Flush, band));
    }
    standing.extend(
        group
            .controls
            .iter()
            .map(|control| filled(*control, form, viewport)),
    );
    standing
}

/// The reference's own control for `control`, which its legacy views draw.
fn emby<'a>(control: Control, form: &'a Form) -> Element<'a, Message> {
    let field = control.field;
    let held = form.value(field);
    let label = strings::lookup(control.label);
    match control.offered {
        Some(options) => {
            let (offered, standing) = self::offered(options, &held);
            widget::select(
                label,
                control.helper,
                offered,
                &standing,
                move |option: &'static str| edited(field, option.to_owned()),
            )
        }
        None => match field {
            Field::Flag { .. } => widget::flag(label, control.helper, held == "true", move |on| {
                edited(field, on.to_string())
            }),
            Field::Text { .. }
            | Field::Number { .. }
            | Field::Choice { .. }
            | Field::Lines { .. }
            | Field::Listed { .. }
            | Field::Named { .. } => widget::field(
                label,
                &held,
                control.helper,
                move |typed| edited(field, typed),
                Message::Unchanged,
                Secrecy::Shown,
            ),
        },
    }
}

/// One group of a legacy section, which the reference writes as a `fieldset`.
fn fieldset<'a>(group: Group, form: &'a Form) -> Element<'a, Message> {
    let mut rows: Vec<Element<'a, Message>> = Vec::new();
    if let Some(note) = group.note {
        rows.push(widget::description(note, space::DESCRIPTION_INSET));
    }
    rows.extend(group.controls.iter().map(|control| emby(*control, form)));
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
