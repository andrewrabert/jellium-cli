use std::collections::HashSet;

use iced::widget::{button, column, container, scrollable, text};
use iced::{Element, Fill};
use jellium_protocol::Target;

use crate::app::Message;
use crate::images::{self, Cache};
use crate::player::osd::{self, Transport};
use crate::player::remote::{self, Bound};
use crate::text::{self as strings, Text};
use crate::theme;

/// One target the picker offers, named by its device name with its client
/// name beneath.
fn offered(target: &Target) -> Element<'_, Message> {
    button(
        column![
            text(target.device_name.clone()).size(16),
            text(target.client_name.clone()).size(13),
        ]
        .spacing(2),
    )
    .on_press(Message::RemoteAction(remote::Action::Take(
        target.session.clone(),
    )))
    .width(Fill)
    .into()
}

fn picker<'a>(targets: &'a [Target]) -> Element<'a, Message> {
    if targets.is_empty() {
        return text(strings::lookup(Text::RemoteEmpty)).into();
    }
    let listed = targets
        .iter()
        .fold(column![].spacing(theme::CARD_SPACING), |listed, target| {
            listed.push(offered(target))
        });
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
    quality: jellium_protocol::Quality,
    images: &'a Cache,
) -> Element<'a, Message> {
    let body: Element<'a, Message> = match bound {
        None => picker(targets),
        Some(bound) => column![
            text(bound.target.device_name.clone()).size(16),
            text(bound.target.client_name.clone()).size(13),
            osd::bar(
                Transport::Remote(bound),
                false,
                device,
                quality,
                chrono::Utc::now(),
                images,
            ),
        ]
        .spacing(theme::CARD_SPACING)
        .into(),
    };

    container(
        column![text(strings::lookup(Text::RemoteTitle)).size(24), body]
            .spacing(theme::CARD_SPACING),
    )
    .padding(theme::CARD_SPACING)
    .into()
}

pub fn images(bound: Option<&Bound>) -> HashSet<images::Key> {
    bound
        .map(|bound| osd::bar_images(Transport::Remote(bound)))
        .unwrap_or_default()
}
