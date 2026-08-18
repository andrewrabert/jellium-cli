//! The reference's Programs tab: what is on now, and the five upcoming rails
//! it draws beneath it.

use std::rc::Rc;

use iced::Element;
use iced::widget::column;
use jellium_model::construct::Construct;
use jellium_model::livetv::Section;
use jellium_protocol::Session;
use jellyfin_api::types::BaseItemDto;

use crate::api::Api;
use crate::app::Message;
use crate::error::Answer;
use crate::images::{Cache, Wanted};
use crate::route::Route;
use crate::style::space::Room;
use crate::style::{self, Viewport, card, space};
use crate::text::{self as strings, Text};
use crate::widget;

/// One rail of the Programs tab: the section's own card shape over the two
/// lines a programme rail writes.
// reference: programs-shapes
fn railed(section: Section) -> card::Drawing {
    card::Drawing {
        card: section.card(),
        footer: card::Footer::NameAndSubtitle,
        backing: card::Backing::Padder,
        footing: card::Footing::Bare,
        setting: card::Setting::Centred,
        bottom: card::Bottom::Padded,
        touch: card::Touch::Plays,
    }
}

/// What one section reads as.
pub fn label(section: Section) -> Text {
    match section {
        Section::OnNow => Text::HomeOnNow,
        Section::Shows => Text::ProgramsShows,
        Section::Movies => Text::ProgramsMovies,
        Section::Sports => Text::ProgramsSports,
        Section::Kids => Text::ProgramsKids,
        Section::News => Text::ProgramsNews,
    }
}

/// The list a section's title opens.
pub fn opens(section: Section) -> Route {
    Route::LiveTv {
        tab: match section {
            Section::OnNow => super::Tab::Guide,
            Section::Shows | Section::Movies | Section::Sports | Section::Kids | Section::News => {
                super::Tab::Programs
            }
        },
    }
}

/// Each section's programmes, in the order the server answered them; a section
/// the server answered nothing for is absent rather than empty.
#[derive(Debug, Clone, Default)]
pub struct State {
    pub sections: Vec<(Section, Vec<BaseItemDto>)>,
}

pub async fn load(api: Rc<Api>, viewport: Viewport) -> Answer<State> {
    Answer::of(async {
        let mut sections = Vec::new();
        for section in Section::ALL {
            let asked = jellium_model::livetv::asked(section, viewport.layout());
            let items = api
                .section_programs(section, asked)
                .await
                .or_default(Text::FailureProgramsUnread);
            if items.is_empty() {
                continue;
            }
            sections.push((section, items));
        }
        Ok(State { sections })
    })
    .await
}

pub fn view<'a>(
    state: &'a State,
    now: chrono::DateTime<chrono::Utc>,
    viewport: Viewport,
    images: &'a Cache,
    session: &'a Session,
) -> Element<'a, Message> {
    let mut page = column![].spacing(style::drawn(space::SECTION_GAP.drawn()));
    for (section, items) in &state.sections {
        page = page.push(widget::section(
            crate::construct::navigation(
                Construct::SectionTitleCards,
                Some(label(*section)),
                Message::Navigated(opens(*section)),
                widget::prose(strings::lookup(label(*section)), style::typeface::HEADING_2),
            ),
            widget::rail(
                railed(*section),
                widget::Rail::of(Construct::ItemsContainer),
                items.iter(),
                Room::content(viewport),
                images,
                now,
                session,
            ),
        ));
    }
    page.into()
}

pub fn images(state: &State) -> Wanted {
    let mut held = Wanted::new();
    for (section, items) in &state.sections {
        held.extend(widget::card_images(items, section.card()));
    }
    held
}
