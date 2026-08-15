//! The display screen: whether missing episodes are drawn.

use iced::Element;
use iced::widget::column;

use crate::app::Message;
use crate::text::Text;
use crate::theme;

use super::flag;

/// `DisplayMissingEpisodes`, and the save, which is absent under read-only.
pub fn view<'a>(
    configuration: &'a jellium_model::form::Form,
    read_only: bool,
) -> Element<'a, Message> {
    let mut shown = column![flag(
        Text::DisplayMissingEpisodes,
        jellium_model::user::MISSING_EPISODES,
        configuration,
    ),]
    .spacing(theme::CARD_SPACING);

    if !read_only {
        shown = shown.push(super::save());
    }

    shown.into()
}
