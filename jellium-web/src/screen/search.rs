use std::collections::HashSet;
use std::rc::Rc;

use iced::Element;
use iced::widget::{button, column, row, text, text_input};

use crate::api::{Api, Page};
use crate::app::Message;
use crate::error::Trouble;
use crate::images::{self, Cache};
use crate::screen::library::{PAGE_SIZE, Step};
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget;

#[derive(Debug, Clone, Default)]
pub struct State {
    pub term: String,
    pub start: i32,
    pub results: Page,
}

pub async fn load(api: Rc<Api>, term: String, start: i32) -> Result<State, Trouble> {
    if term.trim().is_empty() {
        return Ok(State {
            term,
            start,
            results: Page::default(),
        });
    }

    Ok(State {
        results: api.search(&term, start, PAGE_SIZE).await?,
        term,
        start,
    })
}

pub fn view<'a>(state: &'a State, images: &'a Cache) -> Element<'a, Message> {
    let bar = row![
        text_input(strings::lookup(Text::SearchPlaceholder), &state.term)
            .on_input(Message::SearchEdited)
            .on_submit(Message::SearchSubmitted)
            .padding(8),
        button(text(strings::lookup(Text::SearchSubmit))).on_press(Message::SearchSubmitted),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Alignment::Center);

    let mut page = column![bar]
        .spacing(theme::CARD_SPACING)
        .padding(theme::CARD_SPACING);

    if state.results.items.is_empty() {
        page = page.push(widget::notice(
            strings::lookup(Text::SearchEmpty).to_string(),
        ));
        return page.into();
    }

    let last = state.start + state.results.items.len() as i32;
    page = page
        .push(
            row![
                text(strings::format(
                    Text::PagePosition,
                    &[
                        &(state.start + 1).to_string(),
                        &last.to_string(),
                        &state.results.total.to_string(),
                    ],
                )),
                widget::step_button(Text::PagePrevious, Step::Previous, state.start > 0),
                widget::step_button(Text::PageNext, Step::Next, last < state.results.total),
            ]
            .spacing(theme::CARD_SPACING)
            .align_y(iced::Alignment::Center),
        )
        .push(widget::grid(&state.results.items, images));

    page.into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    widget::card_images(&state.results.items)
}
