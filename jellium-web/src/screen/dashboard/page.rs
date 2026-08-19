//! A plugin configuration page renders in a sandboxed frame above the canvas,
//! reachable only through the bridge.

use uuid::Uuid;

use crate::app::{Message, Signed};
use crate::control;
use crate::error::Answer;
use crate::overlay;
use crate::text::Text;
use jellium_model::bridge::{self, Verb};

/// One plugin configuration page open in a frame.
#[derive(Debug)]
pub struct State {
    pub plugin: Uuid,
    pub name: String,
    /// The mounted frame; dropping it removes the frame.
    frame: Option<overlay::Mounted>,
    /// The grant the frame's path carries; leaving the page releases it.
    grant: String,
    /// True while the page has asked for a busy indicator and not released it.
    pub busy: bool,
    /// The notice the page asked to show.
    pub notice: Option<String>,
}

impl State {
    pub fn grant(&self) -> &str {
        &self.grant
    }
}

/// The grant one configuration page was opened under, before its frame is
/// mounted; the mount itself happens where the screen is installed, because a
/// mounted frame is removed by its own `Drop` and so is never carried in a
/// message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Opened {
    pub plugin: Uuid,
    pub name: String,
    pub framed: jellium_protocol::Framed,
}

/// Mints a grant for `name`.
pub async fn load(name: String, plugin: Uuid) -> Answer<Opened> {
    Answer::of(async {
        let framed = control::open_page(name.clone()).await.bubbled()?;
        Ok(Opened {
            plugin,
            name,
            framed,
        })
    })
    .await
}

/// Mounts the frame above the canvas, taking pointer events, under
/// `overlay::PLUGIN_SANDBOX`.
pub fn mounted(opened: Opened) -> State {
    let frame = overlay::Mounted::new(&overlay::Wanted {
        id: overlay::Id::PluginPage,
        kind: overlay::Kind::Frame,
        stacking: overlay::Stacking::Above,
        pointer: true,
        source: Some(opened.framed.path.clone()),
        sandbox: Some(overlay::PLUGIN_SANDBOX),
        accept: None,
        hidden: false,
    });
    State {
        plugin: opened.plugin,
        name: opened.name,
        frame,
        grant: opened.framed.grant,
        busy: false,
        notice: None,
    }
}

/// Releases the grant, which is what leaving the page does.
pub async fn close(grant: String) {
    control::close_page(jellium_protocol::Framed {
        path: String::new(),
        grant,
    })
    .await
    .disregarded(Text::FailurePluginPageRelease);
}

/// Answers one bridge request against the plugin the frame was opened for.
/// A verb outside the nine is refused and named on screen; nothing the payload
/// says about which plugin it addresses is read.
pub fn asked(signed: &mut Signed, payload: &str) -> iced::Task<Message> {
    let asked = match bridge::read(payload) {
        Ok(asked) => asked,
        Err(refused) => {
            crate::failure::raise(crate::error::bridge_refused(&refused));
            return iced::Task::none();
        }
    };

    let Some(state) = shown_mut(signed) else {
        return iced::Task::none();
    };
    let plugin = state.plugin;
    let name = state.name.clone();
    let call = asked.call;

    match asked.verb {
        Verb::Busy => {
            state.busy = true;
            iced::Task::none()
        }
        Verb::Idle => {
            state.busy = false;
            iced::Task::none()
        }
        Verb::Notice { text } => {
            state.notice = Some(text);
            iced::Task::none()
        }
        Verb::SaveOutcome => {
            state.busy = false;
            answered(signed, call, Some(serde_json::Value::Null))
        }
        Verb::ReadConfiguration => {
            let api = signed.api.clone();
            iced::Task::perform(
                async move {
                    api.plugin_configuration(plugin)
                        .await
                        .or_none(Text::FailurePluginPageUnread)
                },
                move |value| Message::Bridged(call, value),
            )
        }
        Verb::WriteConfiguration { body } => {
            let api = signed.api.clone();
            iced::Task::perform(
                async move { api.save_plugin_configuration(plugin, &body).await },
                move |outcome| {
                    Message::BridgedWrote(
                        call,
                        crate::error::Wrote {
                            operation: crate::error::Operation::PluginConfiguration,
                            object: name.clone(),
                        },
                        outcome,
                    )
                },
            )
        }
        Verb::SystemInfo => {
            let api = signed.api.clone();
            iced::Task::perform(
                async move {
                    api.system_info()
                        .await
                        .or_none(Text::FailureBridgeAnswer)
                        .and_then(|info| crate::failure::encoded(Text::FailureBridgeAnswer, &info))
                },
                move |value| Message::Bridged(call, value),
            )
        }
        Verb::Users => {
            let api = signed.api.clone();
            iced::Task::perform(
                async move {
                    api.users()
                        .await
                        .or_none(Text::FailureBridgeAnswer)
                        .and_then(|users| {
                            crate::failure::encoded(Text::FailureBridgeAnswer, &users)
                        })
                },
                move |value| Message::Bridged(call, value),
            )
        }
        Verb::VirtualFolders => {
            let api = signed.api.clone();
            iced::Task::perform(
                async move {
                    api.virtual_folders()
                        .await
                        .or_none(Text::FailureBridgeAnswer)
                        .and_then(|folders| {
                            crate::failure::encoded(Text::FailureBridgeAnswer, &folders)
                        })
                },
                move |value| Message::Bridged(call, value),
            )
        }
    }
}

/// Sends one answer back down the frame's channel.
pub fn answered(
    signed: &mut Signed,
    call: u64,
    value: Option<serde_json::Value>,
) -> iced::Task<Message> {
    if let Some(state) = shown_mut(signed)
        && let Some(frame) = state.frame.as_ref()
    {
        frame.post(&bridge::answer(call, value.as_ref()));
    }
    iced::Task::none()
}

/// The configuration page the dashboard is showing, and nothing where any
/// other screen is.
pub fn shown(signed: &Signed) -> Option<&State> {
    match &signed.view {
        crate::app::View::Dashboard(state) => match &state.body {
            super::Body::Page(page) => Some(page),
            _ => None,
        },
        _ => None,
    }
}

/// The configuration page shown now, and nothing when another screen is.
fn shown_mut(signed: &mut Signed) -> Option<&mut State> {
    match &mut signed.view {
        crate::app::View::Dashboard(state) => match &mut state.body {
            super::Body::Page(page) => Some(page),
            _ => None,
        },
        _ => None,
    }
}
