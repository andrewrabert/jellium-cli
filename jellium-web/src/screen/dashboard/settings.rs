//! The server's configuration, one section per screen, read whole and written
//! whole.

use iced::widget::{button, checkbox, column, text_input};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;
use jellium_model::form::{Field, Form};

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

/// One control per field, an explicit save, and the unsaved-edit indicator.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut tabs = iced::widget::row![].spacing(style::drawn(space::GUTTER.drawn()));
    for section in super::Section::ALL {
        let control = button(prose(
            strings::lookup(section.label()).to_owned(),
            typeface::BODY,
        ));
        tabs = tabs.push(if section == state.section {
            control
        } else {
            control.on_press(Message::DashboardAction(super::Action::Open(
                super::Screen::Settings { section },
            )))
        });
    }

    let mut page = column![
        tabs,
        prose(
            strings::lookup(state.section.label()).to_owned(),
            typeface::HEADING_2
        )
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .padding(style::drawn(space::GUTTER.drawn()));

    for field in state.section.fields() {
        let field = *field;
        let held = state.form.value(field);
        page = page.push(match field {
            Field::Flag { key } => Element::from(
                iced::widget::row![
                    checkbox(held == "true").on_toggle(move |held| {
                        Message::DashboardAction(super::Action::Edited(field, held.to_string()))
                    }),
                    prose(key.to_owned(), typeface::BODY),
                ]
                .spacing(style::drawn(space::GUTTER.drawn()))
                .align_y(iced::Center),
            ),
            _ => Element::from(
                column![
                    prose(field.key().to_owned(), typeface::BODY),
                    text_input(field.key(), &held)
                        .style(style::input)
                        .on_input(move |held| {
                            Message::DashboardAction(super::Action::Edited(field, held))
                        }),
                ]
                .spacing(style::drawn(space::GUTTER.drawn())),
            ),
        });
    }

    if state.form.dirty() {
        page = page.push(prose(
            strings::lookup(Text::DashboardUnsaved).to_owned(),
            typeface::BODY,
        ));
    } else if state.saved {
        page = page.push(prose(
            strings::lookup(Text::DashboardSaved).to_owned(),
            typeface::BODY,
        ));
    }

    if !read_only {
        let mut save = button(prose(
            strings::lookup(Text::DashboardSave).to_owned(),
            typeface::BODY,
        ));
        if state.form.dirty() {
            save = save.on_press(Message::DashboardAction(super::Action::Save));
        }
        page = page.push(save);
    }

    iced::widget::scrollable(page)
        .width(Fill)
        .height(Fill)
        .into()
}
