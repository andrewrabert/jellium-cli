//! The installed plugins, each with its name, version and status, and the
//! configuration pages it hosts.

use iced::Element;
use iced::widget::{button, column, row};
use uuid::Uuid;

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::{line, prose};

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
pub fn view<'a>(state: &'a State, read_only: bool) -> Vec<Element<'a, Message>> {
    let mut listed: Vec<Element<'a, Message>> = vec![
        row![
            button(prose(strings::lookup(Text::CatalogTitle), typeface::BODY))
                .style(style::link)
                .on_press(Message::DashboardAction(super::Action::Open(
                    super::Screen::Catalog
                ))),
            button(prose(
                strings::lookup(Text::RepositoriesTitle),
                typeface::BODY
            ))
            .style(style::link)
            .on_press(Message::DashboardAction(super::Action::Open(
                super::Screen::Repositories
            ))),
        ]
        .spacing(style::drawn(space::CONTROL_GAP.drawn()))
        .into(),
    ];

    for plugin in &state.plugins {
        let Some(id) = plugin.id else {
            continue;
        };
        let version = plugin.version.clone().unwrap_or_default();
        let mut held = column![
            line(
                plugin.name.clone().unwrap_or_default(),
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            ),
            line(
                strings::format(Text::PluginsVersion, &[&version]),
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            ),
            line(
                strings::format(Text::PluginsStatus, &[&status(plugin)]),
                typeface::BODY,
                typeface::Weight::Regular,
                typeface::LINE_HEIGHT,
            ),
        ]
        .spacing(style::drawn(space::SECTION_GAP.drawn()));

        if !read_only {
            held = held.push(
                row![
                    button(prose(strings::lookup(Text::PluginsEnable), typeface::BODY))
                        .style(style::raised)
                        .on_press(Message::DashboardAction(super::Action::Write(
                            super::Written::EnablePlugin {
                                id,
                                version: version.clone(),
                                name: plugin.name.clone().unwrap_or_default(),
                            }
                        ))),
                    button(prose(strings::lookup(Text::PluginsDisable), typeface::BODY))
                        .style(style::raised)
                        .on_press(Message::DashboardAction(super::Action::Write(
                            super::Written::DisablePlugin {
                                id,
                                version: version.clone(),
                                name: plugin.name.clone().unwrap_or_default(),
                            }
                        ))),
                    button(prose(
                        strings::lookup(Text::PluginsUninstall),
                        typeface::BODY
                    ))
                    .style(style::raised)
                    .on_press(Message::DashboardAction(super::Action::Ask(
                        crate::screen::confirm::Pending::of(
                            crate::screen::confirm::Destructive::UninstallPlugin {
                                id,
                                version: version.clone(),
                            },
                            plugin.name.clone().unwrap_or_default(),
                        )
                    ))),
                ]
                .spacing(style::drawn(space::CONTROL_GAP.drawn())),
            );
        }

        let pages = state.pages_of(id);
        if pages.is_empty() {
            held = held.push(prose(strings::lookup(Text::PluginsNoPages), typeface::BODY));
        } else {
            for page in pages {
                let Some(name) = page.name.clone() else {
                    continue;
                };
                held = held.push(
                    button(prose(name.clone(), typeface::BODY))
                        .style(style::link)
                        .on_press(Message::DashboardAction(super::Action::Open(
                            super::Screen::PluginPage { plugin: id, name },
                        ))),
                );
            }
        }
        listed.push(held.into());
    }

    listed
}
