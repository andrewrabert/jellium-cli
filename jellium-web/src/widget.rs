use std::collections::HashSet;

use iced::widget::{button, column, container, grid as grid_widget, image, row, scrollable, text};
use iced::{Element, Fill, Length};
use jellium_protocol::Session;
use jellyfin_api::types::BaseItemDto;

use crate::app::Message;
use crate::error::Trouble;
use crate::images::{self, Cache, Kind};
use crate::route::Route;
use crate::screen::library::Step;
use crate::text::{self as strings, Text};
use crate::theme;

fn poster_key(item: &BaseItemDto) -> Option<images::Key> {
    Some(images::Key {
        item: item.id?,
        kind: Kind::Primary,
        width: theme::IMAGE_WIDTH,
    })
}

pub fn card_images(items: &[BaseItemDto]) -> HashSet<images::Key> {
    items.iter().filter_map(poster_key).collect()
}

fn subtitle(item: &BaseItemDto) -> String {
    match (&item.series_name, item.production_year) {
        (Some(series), _) => series.clone(),
        (None, Some(year)) => year.to_string(),
        (None, None) => String::new(),
    }
}

pub fn card<'a>(item: &'a BaseItemDto, image: Option<image::Handle>) -> Element<'a, Message> {
    let poster: Element<'a, Message> = match image {
        Some(handle) => iced::widget::image(handle).width(theme::CARD_WIDTH).into(),
        None => container(text(""))
            .width(theme::CARD_WIDTH)
            .height(theme::CARD_WIDTH * 1.5)
            .into(),
    };

    let body = column![
        poster,
        text(item.name.clone().unwrap_or_default()).size(15),
        text(subtitle(item)).size(13),
    ]
    .spacing(4)
    .width(theme::CARD_WIDTH);

    let pressed = item.id.map(|id| Message::Navigated(Route::Detail { id }));
    let mut control = button(body).style(button::text);
    if let Some(message) = pressed {
        control = control.on_press(message);
    }
    control.into()
}

fn cards<'a>(items: &'a [BaseItemDto], images: &'a Cache) -> Vec<Element<'a, Message>> {
    items
        .iter()
        .map(|item| card(item, poster_key(item).and_then(|key| images.handle(key))))
        .collect()
}

pub fn rail<'a>(title: Text, items: &'a [BaseItemDto], images: &'a Cache) -> Element<'a, Message> {
    column![
        text(strings::lookup(title)).size(22),
        scrollable(row(cards(items, images)).spacing(theme::CARD_SPACING))
            .direction(scrollable::Direction::Horizontal(
                scrollable::Scrollbar::default(),
            ))
            .height(theme::RAIL_HEIGHT),
    ]
    .spacing(8)
    .into()
}

pub fn grid<'a>(items: &'a [BaseItemDto], images: &'a Cache) -> Element<'a, Message> {
    scrollable(
        grid_widget(cards(items, images))
            .fluid(theme::CARD_WIDTH)
            .spacing(theme::CARD_SPACING),
    )
    .height(Fill)
    .into()
}

pub fn library_row<'a>(libraries: &'a [BaseItemDto]) -> Element<'a, Message> {
    let entries = libraries.iter().filter_map(|library| {
        let id = library.id?;
        Some(
            button(text(library.name.clone().unwrap_or_default()))
                .on_press(Message::Navigated(Route::Library {
                    id,
                    sort: crate::screen::library::Sort::Name,
                    start: 0,
                }))
                .into(),
        )
    });

    scrollable(row(entries).spacing(theme::CARD_SPACING))
        .direction(scrollable::Direction::Horizontal(
            scrollable::Scrollbar::default(),
        ))
        .into()
}

pub fn step_button<'a>(label: Text, step: Step, enabled: bool) -> Element<'a, Message> {
    let mut control = button(text(strings::lookup(label)));
    if enabled {
        control = control.on_press(Message::PageStepped(step));
    }
    control.into()
}

pub fn notice<'a>(message: String) -> Element<'a, Message> {
    container(text(message))
        .padding(theme::CARD_SPACING)
        .width(Length::Fill)
        .into()
}

pub fn chrome<'a>(
    session: &'a Session,
    back: bool,
    notice: Option<&'a Trouble>,
    body: Element<'a, Message>,
) -> Element<'a, Message> {
    let mut nav = row![].spacing(theme::CARD_SPACING);

    if back {
        nav = nav.push(button(text(strings::lookup(Text::NavBack))).on_press(Message::WentBack));
    }
    nav = nav
        .push(
            button(text(strings::lookup(Text::NavHome))).on_press(Message::Navigated(Route::Home)),
        )
        .push(
            button(text(strings::lookup(Text::NavSearch))).on_press(Message::Navigated(
                Route::Search {
                    term: String::new(),
                    start: 0,
                },
            )),
        )
        .push(button(text(strings::lookup(Text::NavLogout))).on_press(Message::LogoutPressed));

    let mut page = column![nav]
        .spacing(theme::CARD_SPACING)
        .padding(theme::CARD_SPACING);

    if session.off_snapshot() {
        page = page.push(self::notice(strings::format(
            Text::WarningOffSnapshot,
            &[&session.server_version, &session.snapshot_version],
        )));
    }

    if let Some(trouble) = notice {
        page = page.push(self::notice(trouble.message()));
    }

    page.push(body).into()
}
