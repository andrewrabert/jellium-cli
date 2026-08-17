use std::collections::HashSet;

use iced::widget::{button, column, container, scrollable};
use iced::{Element, Fill};
use jellium_protocol::{SyncAccess, Target};

use crate::app::Message;
use crate::images::{self, Cache};
use crate::player::osd::{self, Transport};
use crate::player::remote::{self, Bound};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

/// One target the picker offers, named by its device name with its client
/// name beneath.
fn offered(target: &Target) -> Element<'_, Message> {
    button(
        column![
            prose(target.device_name.clone(), typeface::BODY),
            prose(target.client_name.clone(), typeface::SECONDARY),
        ]
        .spacing(style::drawn(space::BLOCK_GAP.drawn())),
    )
    .on_press(Message::RemoteAction(remote::Action::Take(
        target.session.clone(),
    )))
    .width(Fill)
    .into()
}

fn picker<'a>(targets: &'a [Target]) -> Element<'a, Message> {
    if targets.is_empty() {
        return prose(strings::lookup(Text::RemoteEmpty), typeface::BODY);
    }
    let listed = targets.iter().fold(
        column![].spacing(style::drawn(space::GUTTER.drawn())),
        |listed, target| listed.push(offered(target)),
    );
    scrollable(listed).height(Fill).into()
}

/// The device picker when nothing is bound — each target named by its device
/// name with its client name beneath — and the remote panel when one is:
/// play/pause, stop, a scrub bar with elapsed and total time, skip back, skip
/// forward, next, previous, volume with mute, repeat, shuffle and leaving.
pub fn view<'a>(
    bound: Option<&'a Bound>,
    targets: &'a [Target],
    device: crate::prefs::Device,
    images: &'a Cache,
    viewport: Viewport,
) -> Element<'a, Message> {
    let body: Element<'a, Message> = match bound {
        None => picker(targets),
        Some(bound) => column![
            prose(bound.target.device_name.clone(), typeface::BODY),
            prose(bound.target.client_name.clone(), typeface::SECONDARY),
            osd::bar(
                Transport::Remote(bound),
                SyncAccess::None,
                device,
                images,
                viewport,
            ),
        ]
        .spacing(style::drawn(space::GUTTER.drawn()))
        .into(),
    };

    container(
        column![
            prose(strings::lookup(Text::RemoteTitle), typeface::HEADING_2),
            body
        ]
        .spacing(style::drawn(space::GUTTER.drawn())),
    )
    .padding(style::drawn(space::GUTTER.drawn()))
    .into()
}

pub fn images(bound: Option<&Bound>) -> HashSet<images::Key> {
    bound
        .map(|bound| osd::bar_images(Transport::Remote(bound)))
        .unwrap_or_default()
}
