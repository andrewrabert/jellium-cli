use std::collections::HashSet;

use iced::Element;
use iced::widget::{column, container};
use jellium_protocol::{SyncAccess, Target};

use crate::app::Message;
use crate::icon::Icon;
use crate::images::{self, Cache};
use crate::player::osd::{self, Transport};
use crate::player::remote::{self, Bound};
use crate::style::{self, Viewport, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;
use crate::widget::sheet::{Entry, Item, sheet};

/// The glyph the reference draws for a target's device type: `tv`, which is
/// what its own switch answers for a target naming none, and no target this
/// client is handed names one.
// reference: device-picker-glyph
const TARGET_GLYPH: Icon = Icon::Tv;

/// The device picker, as the sheet the reference raises for `HeaderPlayOn`:
/// each target named by its device with its client beneath.
// reference: device-picker-sheet
fn picker<'a>(targets: &'a [Target], viewport: Viewport) -> Element<'a, Message> {
    if targets.is_empty() {
        return prose(strings::lookup(Text::RemoteEmpty), typeface::BODY);
    }
    sheet(
        Some(strings::lookup(Text::SheetPlayOn).into()),
        None,
        targets.iter().map(|target| {
            Entry::Item(Item {
                glyph: Some(TARGET_GLYPH),
                name: target.device_name.as_str().into(),
                secondary: Some(target.client_name.as_str().into()),
                aside: None,
                press: Message::RemoteAction(remote::Action::Take(target.session.clone())),
            })
        }),
        None,
        viewport,
    )
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
        None => picker(targets, viewport),
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
        .spacing(style::drawn(space::SECTION_GAP.drawn()))
        .into(),
    };

    container(
        column![
            prose(strings::lookup(Text::RemoteTitle), typeface::HEADING_2),
            body
        ]
        .spacing(style::drawn(space::SECTION_GAP.drawn())),
    )
    .padding(style::padding(space::PAGE_PAD))
    .into()
}

pub fn images(bound: Option<&Bound>) -> HashSet<images::Key> {
    bound
        .map(|bound| osd::bar_images(Transport::Remote(bound)))
        .unwrap_or_default()
}
