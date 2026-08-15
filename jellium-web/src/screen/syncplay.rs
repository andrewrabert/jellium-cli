use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Element, Fill};
use jellium_protocol::{Group, GroupState, SyncAccess};

use crate::app::Message;
use crate::player::group::{self, Joined};
use crate::text::{self as strings, Text};
use crate::theme;

fn state_label(state: GroupState) -> Text {
    match state {
        GroupState::Idle => Text::SyncPlayStateIdle,
        GroupState::Waiting => Text::SyncPlayStateWaiting,
        GroupState::Paused => Text::SyncPlayStatePaused,
        GroupState::Playing => Text::SyncPlayStatePlaying,
    }
}

fn participants(group: &Group) -> String {
    strings::format(
        Text::SyncPlayParticipants,
        &[&group.participants.join(", ")],
    )
}

/// One joinable group, named with its participants and its state.
fn offered(group: &Group) -> Element<'_, Message> {
    row![
        column![
            text(group.name.clone()).size(16),
            text(participants(group)).size(13),
            text(strings::lookup(state_label(group.state))).size(13),
        ]
        .spacing(2),
        iced::widget::Space::new().width(Fill),
        button(text(strings::lookup(Text::SyncPlayJoin)))
            .on_press(Message::GroupAction(group::Action::Join(group.id))),
    ]
    .spacing(theme::CARD_SPACING)
    .align_y(iced::Center)
    .into()
}

fn picker<'a>(groups: &'a [Group], access: SyncAccess) -> Element<'a, Message> {
    let mut body = column![].spacing(theme::CARD_SPACING);
    if access == SyncAccess::CreateAndJoin {
        body = body.push(
            button(text(strings::lookup(Text::SyncPlayCreate)))
                .on_press(Message::GroupAction(group::Action::Create)),
        );
    }
    if groups.is_empty() {
        return body.push(text(strings::lookup(Text::SyncPlayEmpty))).into();
    }
    let listed = groups
        .iter()
        .fold(column![].spacing(theme::CARD_SPACING), |listed, group| {
            listed.push(offered(group))
        });
    body.push(scrollable(listed).height(Fill)).into()
}

fn active<'a>(joined: &'a Joined) -> Element<'a, Message> {
    let mut body = column![
        text(joined.group.name.clone()).size(16),
        text(participants(&joined.group)).size(13),
        text(strings::lookup(state_label(joined.group.state))).size(13),
    ]
    .spacing(2);

    if joined.waiting() {
        body = body.push(text(strings::lookup(Text::SyncPlayWaiting)).size(13));
    }

    body.push(
        button(text(strings::lookup(Text::SyncPlayStop)))
            .on_press(Message::GroupAction(group::Action::Stop)),
    )
    .push(iced::widget::Space::new().height(theme::CARD_SPACING))
    .push(
        button(text(strings::lookup(Text::SyncPlayLeave)))
            .on_press(Message::GroupAction(group::Action::Leave)),
    )
    .spacing(theme::CARD_SPACING)
    .into()
}

/// The joinable groups when this installation is in none — each named with its
/// participants and its state — and the group when it is in one: its name, its
/// participants, its state, the waiting indicator, the action that stops the
/// group and, set apart from it, the action that leaves the group.
/// The create action is drawn only for `SyncAccess::CreateAndJoin`.
pub fn view<'a>(
    joined: Option<&'a Joined>,
    groups: &'a [Group],
    access: SyncAccess,
) -> Element<'a, Message> {
    let body: Element<'a, Message> = match joined {
        Some(joined) => active(joined),
        None => picker(groups, access),
    };

    container(
        column![text(strings::lookup(Text::SyncPlayTitle)).size(24), body]
            .spacing(theme::CARD_SPACING),
    )
    .padding(theme::CARD_SPACING)
    .into()
}
