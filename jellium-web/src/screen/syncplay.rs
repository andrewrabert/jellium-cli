use iced::widget::{button, column, container, row, scrollable};
use iced::{Element, Fill};
use jellium_protocol::{Group, GroupState, SyncAccess};

use crate::app::Message;
use crate::player::group::{self, Joined};
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

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
            prose(group.name.clone(), typeface::BODY),
            prose(participants(group), typeface::SECONDARY),
            prose(
                strings::lookup(state_label(group.state)),
                typeface::SECONDARY
            ),
        ]
        .spacing(style::drawn(space::BLOCK_GAP.drawn())),
        iced::widget::Space::new().width(Fill),
        button(prose(strings::lookup(Text::SyncPlayJoin), typeface::BODY))
            .style(style::raised)
            .on_press(Message::GroupAction(group::Action::Join(group.id))),
    ]
    .spacing(style::drawn(space::GUTTER.drawn()))
    .align_y(iced::Center)
    .into()
}

fn picker<'a>(groups: &'a [Group], access: SyncAccess) -> Element<'a, Message> {
    let mut body = column![].spacing(style::drawn(space::GUTTER.drawn()));
    if access == SyncAccess::CreateAndJoin {
        body = body.push(
            button(prose(strings::lookup(Text::SyncPlayCreate), typeface::BODY))
                .style(style::submit)
                .on_press(Message::GroupAction(group::Action::Create)),
        );
    }
    if groups.is_empty() {
        return body
            .push(prose(strings::lookup(Text::SyncPlayEmpty), typeface::BODY))
            .into();
    }
    let listed = groups.iter().fold(
        column![].spacing(style::drawn(space::GUTTER.drawn())),
        |listed, group| listed.push(offered(group)),
    );
    body.push(scrollable(listed).height(Fill)).into()
}

fn active<'a>(joined: &'a Joined) -> Element<'a, Message> {
    let mut body = column![
        prose(joined.group.name.clone(), typeface::BODY),
        prose(participants(&joined.group), typeface::SECONDARY),
        prose(
            strings::lookup(state_label(joined.group.state)),
            typeface::SECONDARY
        ),
    ]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()));

    if joined.waiting() {
        body = body.push(prose(
            strings::lookup(Text::SyncPlayWaiting),
            typeface::SECONDARY,
        ));
    }

    body.push(
        button(prose(strings::lookup(Text::SyncPlayStop), typeface::BODY))
            .style(style::raised)
            .on_press(Message::GroupAction(group::Action::Stop)),
    )
    .push(iced::widget::Space::new().height(style::drawn(space::GUTTER.drawn())))
    .push(
        button(prose(strings::lookup(Text::SyncPlayLeave), typeface::BODY))
            .style(style::raised)
            .on_press(Message::GroupAction(group::Action::Leave)),
    )
    .spacing(style::drawn(space::GUTTER.drawn()))
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
        column![
            prose(strings::lookup(Text::SyncPlayTitle), typeface::HEADING_2),
            body
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    )
    .padding(style::drawn(space::GUTTER.drawn()))
    .into()
}
