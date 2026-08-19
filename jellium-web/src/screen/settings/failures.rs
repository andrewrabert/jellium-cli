//! The session's own failure list.
//!
//! The reference has no counterpart for this screen; it stands under the
//! `failure-list` row of `reference/exemptions.tsv`.

use iced::Element;
use iced::widget::column;

use crate::app::Message;
use crate::construct::{self, Own};
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

/// Every failure raised this session, newest first, each over the Jellyfin
/// server's own message where it carried one.
pub fn view<'a>(failures: &'a crate::failure::Log) -> Element<'a, Message> {
    let mut held = column![].spacing(style::drawn(space::SECTION_GAP.drawn()));
    if failures.raised().is_empty() {
        held = held.push(prose(strings::lookup(Text::FailuresEmpty), typeface::BODY));
    }
    for raised in failures.raised() {
        let mut one = column![prose(raised.sentence.clone(), typeface::BODY)];
        if let Some(server) = &raised.server {
            one = one.push(prose(format!("> {server}"), typeface::SECONDARY));
        }
        held = held.push(one.spacing(style::drawn(space::BLOCK_GAP.drawn())));
    }
    construct::own(Own::FailureList, held.into())
}
