//! The server's configuration, one section per screen, read whole and written
//! whole.

use crate::app::Message;
use crate::error::Answer;
use crate::style::Viewport;
use crate::text::Text;
use crate::widget;
use jellium_model::form::Form;

use super::frame;

/// One configuration screen: the section it edits, held as the server answered
/// it with the edits made against it.
#[derive(Debug, Clone)]
pub struct State {
    pub section: super::Section,
    pub form: Form,
    /// True once a save has landed and nothing has been edited since.
    pub saved: bool,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>, section: super::Section) -> Answer<State> {
    Answer::of(async {
        let read = match section.key() {
            Some(key) => api.section(key).await.bubbled()?,
            None => api.server_configuration().await.bubbled()?,
        };
        Ok(State {
            section,
            form: Form::of(read),
            saved: false,
        })
    })
    .await
}

/// The section's fields in their groups, each group under the heading it
/// carries, the notice a landed save raises above them, and the save control
/// at the foot.
// reference: dashboard-content
pub fn view<'a>(state: &'a State, read_only: bool, viewport: Viewport) -> frame::Filling<'a> {
    let controls = state.section.controls();
    let layout = viewport.layout();
    let mut rows = vec![super::controls(
        state.section.groups(),
        &state.form,
        controls,
        viewport,
    )];

    if state.saved {
        rows.push(widget::mui::succeeded(Text::DashboardSaved, layout));
    }

    if !read_only {
        let press = state
            .form
            .dirty()
            .then_some(Message::DashboardAction(super::Action::Save));
        rows.push(super::save(controls, press, layout));
    }

    frame::Filling::Capped { above: None, rows }
}
