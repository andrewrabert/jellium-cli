use iced::Element;
use iced::widget::{column, container};
use jellium_protocol::{Group, GroupState, SyncAccess};

use crate::app::Message;
use crate::icon::Icon;
use crate::player::group::{self, Joined};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Template, Text};
use crate::widget::prose;
use crate::widget::sheet::{Entry, Item, sheet};

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
        Template::SyncPlayParticipants,
        &[&group.participants.join(", ")],
    )
}

/// The joinable groups, as the sheet the reference raises: each group under the
/// person glyph with its participants beneath, and New Group under the add
/// glyph where this installation may create one.
// reference: group-picker-sheet
fn picker<'a>(groups: &'a [Group], access: SyncAccess, viewport: Viewport) -> Element<'a, Message> {
    if groups.is_empty() && access != SyncAccess::CreateAndJoin {
        return prose(strings::lookup(Text::SyncPlayEmpty), typeface::BODY);
    }
    let mut entries: Vec<Entry<'a>> = groups
        .iter()
        .map(|group| {
            Entry::Item(Item {
                glyph: Some(Icon::Person),
                name: group.name.as_str().into(),
                secondary: Some(participants(group).into()),
                aside: None,
                press: Message::GroupAction(group::Action::Join(group.id)),
            })
        })
        .collect();
    if access == SyncAccess::CreateAndJoin {
        entries.push(Entry::Item(Item {
            glyph: Some(Icon::Add),
            name: strings::lookup(Text::SheetNewGroup).into(),
            secondary: Some(strings::lookup(Text::SheetNewGroupHelp).into()),
            aside: None,
            press: Message::GroupAction(group::Action::Create),
        }));
    }
    sheet(
        Some(strings::lookup(Text::SheetSelectGroup).into()),
        None,
        entries,
        None,
        viewport,
    )
}

/// The group this installation is in, as the sheet the reference raises for it:
/// titled with the group's own name over its participants, offering to halt
/// playback and to leave, each with its own sentence beneath.
// reference: group-sheet
fn active<'a>(joined: &'a Joined, viewport: Viewport) -> Element<'a, Message> {
    let mut standing = column![prose(
        strings::lookup(state_label(joined.group.state)),
        typeface::SECONDARY
    )]
    .spacing(style::drawn(space::BLOCK_GAP.drawn()));

    if joined.waiting() {
        standing = standing.push(prose(
            strings::lookup(Text::SyncPlayWaiting),
            typeface::SECONDARY,
        ));
    }

    let entries = [
        Entry::Item(Item {
            glyph: Some(Icon::PauseCircleFilled),
            name: strings::lookup(Text::SheetHaltPlayback).into(),
            secondary: Some(strings::lookup(Text::SheetHaltPlaybackHelp).into()),
            aside: None,
            press: Message::GroupAction(group::Action::Stop),
        }),
        Entry::Item(Item {
            glyph: Some(Icon::MeetingRoom),
            name: strings::lookup(Text::SheetLeaveGroup).into(),
            secondary: Some(strings::lookup(Text::SheetLeaveGroupHelp).into()),
            aside: None,
            press: Message::GroupAction(group::Action::Leave),
        }),
    ];

    standing
        .push(sheet(
            Some(joined.group.name.as_str().into()),
            Some(participants(&joined.group).into()),
            entries,
            None,
            viewport,
        ))
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
    viewport: Viewport,
) -> Element<'a, Message> {
    let body: Element<'a, Message> = match joined {
        Some(joined) => active(joined, viewport),
        None => picker(groups, access, viewport),
    };

    container(
        column![
            prose(strings::lookup(Text::SyncPlayTitle), typeface::HEADING_2),
            body
        ]
        .spacing(style::drawn(space::SECTION_GAP.drawn())),
    )
    .padding(style::padding(space::PAGE_PAD))
    .into()
}
