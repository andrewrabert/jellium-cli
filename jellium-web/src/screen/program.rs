use std::collections::HashSet;
use std::rc::Rc;

use chrono::{DateTime, Utc};
use iced::widget::{Space, button, column, container, image, row, text};
use iced::{Element, Fill};

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{self, Cache};
use crate::livetv::Program;
use crate::screen::livetv::Action;
use crate::text::{self as strings, Text};
use crate::theme;

#[derive(Debug, Clone)]
pub struct State {
    pub program: Program,
}

pub async fn load(api: Rc<Api>, program: String) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            program: api.program(&program).await.bubbled()?,
        })
    })
    .await
}

fn key(program: &Program) -> images::Key {
    images::Key {
        item: program.item,
        kind: images::Kind::Primary,
        index: None,
        width: theme::IMAGE_WIDTH,
    }
}

fn badge<'a>(label: Text) -> Element<'a, Message> {
    container(text(strings::lookup(label)).size(12))
        .padding(2)
        .into()
}

/// The program's title, its channel by name and number, its start and end, its
/// overview, its image, its genres and its live, new, premiere and repeat
/// flags, with Record, Record Series and Cancel according to the timers
/// covering it, and Play while it is airing.
pub fn view<'a>(state: &'a State, now: DateTime<Utc>, images: &'a Cache) -> Element<'a, Message> {
    let program = &state.program;

    let art: Element<'a, Message> = match images.handle(key(program)) {
        Some(handle) => image(handle).width(theme::CARD_WIDTH).into(),
        None => Space::new().width(theme::CARD_WIDTH).into(),
    };

    let mut flags = row![].spacing(8);
    if program.live {
        flags = flags.push(badge(Text::GuideBadgeLive));
    }
    if program.new {
        flags = flags.push(badge(Text::GuideBadgeNew));
    }
    if program.premiere {
        flags = flags.push(badge(Text::GuideBadgePremiere));
    }
    if program.repeat {
        flags = flags.push(badge(Text::GuideBadgeRepeat));
    }

    let mut controls = row![].spacing(theme::CARD_SPACING);
    if program.airing(now) {
        controls = controls.push(
            button(text(strings::lookup(Text::ProgramPlay)))
                .on_press(Message::LiveTvAction(Action::PlayChannel(program.channel))),
        );
    }
    match program.timer.clone() {
        Some(timer) => {
            controls = controls.push(
                button(text(strings::lookup(Text::ProgramCancelRecording)))
                    .on_press(Message::LiveTvAction(Action::CancelTimer(timer))),
            );
        }
        None => {
            controls = controls.push(
                button(text(strings::lookup(Text::ProgramRecord)))
                    .on_press(Message::LiveTvAction(Action::Record(program.id.clone()))),
            );
        }
    }
    match program.series_timer.clone() {
        Some(timer) => {
            controls = controls.push(
                button(text(strings::lookup(Text::ProgramCancelSeries)))
                    .on_press(Message::LiveTvAction(Action::CancelSeriesTimer(timer))),
            );
        }
        None => {
            controls = controls.push(
                button(text(strings::lookup(Text::ProgramRecordSeries))).on_press(
                    Message::LiveTvAction(Action::RecordSeries(program.id.clone())),
                ),
            );
        }
    }

    let described = column![
        text(crate::text::format(Text::ProgramTitle, &[&program.title])).size(22),
        text(crate::text::format(
            Text::ProgramChannel,
            &[&program.channel_name, &program.channel_number]
        )),
        text(crate::livetv::airtime(program)),
        flags,
        text(program.overview.clone()),
        text(crate::text::format(
            Text::ProgramGenres,
            &[&program.genres.join(", ")]
        )),
        controls,
    ]
    .spacing(theme::CARD_SPACING)
    .width(Fill);

    container(row![art, described].spacing(theme::CARD_SPACING))
        .padding(theme::CARD_SPACING)
        .width(Fill)
        .into()
}

pub fn images(state: &State) -> HashSet<images::Key> {
    HashSet::from([key(&state.program)])
}
