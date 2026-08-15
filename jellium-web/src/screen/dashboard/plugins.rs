//! The installed plugins, each with its name, version and status, and the
//! configuration pages it hosts.

use iced::widget::{button, column, row, text};
use iced::{Element, Fill};
use uuid::Uuid;

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::theme;

/// The plugins installed on the server, and the configuration pages they host.
#[derive(Debug, Clone)]
pub struct State {
    pub plugins: Vec<jellyfin_api::types::PluginInfo>,
    /// Every configuration page the server listed, which is what names a
    /// plugin's own pages.
    pub pages: Vec<jellyfin_api::types::ConfigurationPageInfo>,
}

impl State {
    /// The pages `plugin` hosts, which are exactly the listed pages naming it;
    /// a page belonging to no installed plugin is named by none.
    pub fn pages_of(&self, plugin: Uuid) -> Vec<&jellyfin_api::types::ConfigurationPageInfo> {
        self.pages
            .iter()
            .filter(|page| page.plugin_id == Some(plugin))
            .collect()
    }
}

pub async fn load(api: std::rc::Rc<crate::api::Api>) -> Answer<State> {
    Answer::of(async {
        let plugins = api.plugins().await.bubbled()?;
        let pages = api.configuration_pages().await.bubbled()?;
        Ok(State { plugins, pages })
    })
    .await
}

fn status(plugin: &jellyfin_api::types::PluginInfo) -> String {
    plugin
        .status
        .map(|status| status.to_string())
        .unwrap_or_default()
}

/// Each plugin's name, version and status, its enable, disable and uninstall
/// controls, and the configuration pages it hosts; a plugin hosting no page
/// shows none.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    let mut listed = column![text(strings::lookup(Text::PluginsTitle)).size(22)]
        .spacing(theme::CARD_SPACING)
        .padding(theme::CARD_SPACING);

    for plugin in &state.plugins {
        let Some(id) = plugin.id else {
            continue;
        };
        let version = plugin.version.clone().unwrap_or_default();
        let mut held = column![
            text(plugin.name.clone().unwrap_or_default()),
            text(strings::format(Text::PluginsVersion, &[&version])),
            text(strings::format(Text::PluginsStatus, &[&status(plugin)])),
        ]
        .spacing(theme::CARD_SPACING);

        if !read_only {
            held = held.push(
                row![
                    button(text(strings::lookup(Text::PluginsEnable))).on_press(
                        Message::DashboardAction(super::Action::Write(
                            super::Written::EnablePlugin {
                                id,
                                version: version.clone(),
                                name: plugin.name.clone().unwrap_or_default(),
                            }
                        ))
                    ),
                    button(text(strings::lookup(Text::PluginsDisable))).on_press(
                        Message::DashboardAction(super::Action::Write(
                            super::Written::DisablePlugin {
                                id,
                                version: version.clone(),
                                name: plugin.name.clone().unwrap_or_default(),
                            }
                        ))
                    ),
                    button(text(strings::lookup(Text::PluginsUninstall))).on_press(
                        Message::DashboardAction(super::Action::Ask(
                            crate::screen::confirm::Pending::of(
                                crate::screen::confirm::Destructive::UninstallPlugin {
                                    id,
                                    version: version.clone(),
                                },
                                plugin.name.clone().unwrap_or_default(),
                            )
                        ))
                    ),
                ]
                .spacing(theme::CARD_SPACING),
            );
        }

        let pages = state.pages_of(id);
        if pages.is_empty() {
            held = held.push(text(strings::lookup(Text::PluginsNoPages)));
        } else {
            for page in pages {
                let Some(name) = page.name.clone() else {
                    continue;
                };
                held = held.push(
                    button(text(name.clone())).on_press(Message::DashboardAction(
                        super::Action::Open(super::Screen::PluginPage { plugin: id, name }),
                    )),
                );
            }
        }
        listed = listed.push(held);
    }

    iced::widget::scrollable(listed)
        .width(Fill)
        .height(Fill)
        .into()
}
