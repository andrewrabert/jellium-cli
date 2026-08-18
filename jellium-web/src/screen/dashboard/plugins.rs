//! The installed plugins, each with its name, version and status, and the
//! configuration pages it hosts.

use iced::Element;
use iced::widget::{button, row};
use uuid::Uuid;

use crate::app::Message;
use crate::error::Answer;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

/// The plugins installed on the server, and the configuration pages they host.
#[derive(Debug, Clone)]
pub struct State {
    pub plugins: Vec<jellyfin_api::types::PluginInfo>,
    /// Every configuration page the server listed, which is what names a
    /// plugin's own pages.
    pub pages: Vec<jellyfin_api::types::ConfigurationPageInfo>,
    /// The plugin whose commands are open.
    pub menu: Option<Uuid>,
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
        Ok(State {
            plugins,
            pages,
            menu: None,
        })
    })
    .await
}

/// The line one plugin's card writes under its name: its version and its
/// status joined by the space the reference joins them with, each dropped
/// where the server names none.
// reference: dashboard-plugin-card
fn described(plugin: &jellyfin_api::types::PluginInfo) -> String {
    [
        plugin.version.clone(),
        plugin.status.map(|status| status.to_string()),
    ]
    .into_iter()
    .flatten()
    .filter(|said| !said.is_empty())
    .collect::<Vec<String>>()
    .join(" ")
}

/// The commands one plugin's card opens: its configuration pages, and its
/// enable, disable and uninstall, the three absent under read-only.
// the reference's own card opens a page for the plugin; here it opens that
// page's commands in the page above the grid
// reference: dashboard-plugin-card
fn menu<'a>(state: &'a State, open: Uuid, read_only: bool) -> Element<'a, Message> {
    let plugin = state.plugins.iter().find(|plugin| plugin.id == Some(open));
    let name = plugin
        .and_then(|plugin| plugin.name.clone())
        .unwrap_or_default();
    let version = plugin
        .and_then(|plugin| plugin.version.clone())
        .unwrap_or_default();

    let mut rows: Vec<crate::widget::list::Row<'a>> = state
        .pages_of(open)
        .into_iter()
        .filter_map(|page| page.name.clone())
        .map(|page| crate::widget::list::Row {
            face: Some(crate::widget::list::Face::Glyph(
                crate::icon::Icon::Settings,
            )),
            index: None,
            title: page.clone().into(),
            secondary: Vec::new(),
            press: crate::widget::list::Press::Whole(Message::DashboardAction(
                super::Action::Open(super::Screen::PluginPage {
                    plugin: open,
                    name: page,
                }),
            )),
            controls: Vec::new(),
        })
        .collect();
    if !read_only {
        rows.push(crate::widget::list::Row {
            face: Some(crate::widget::list::Face::Glyph(
                crate::icon::Icon::CheckCircleOutline,
            )),
            index: None,
            title: strings::lookup(Text::PluginsEnable).into(),
            secondary: Vec::new(),
            press: crate::widget::list::Press::Whole(Message::DashboardAction(
                super::Action::Write(super::Written::EnablePlugin {
                    id: open,
                    version: version.clone(),
                    name: name.clone(),
                }),
            )),
            controls: Vec::new(),
        });
        rows.push(crate::widget::list::Row {
            face: Some(crate::widget::list::Face::Glyph(crate::icon::Icon::Close)),
            index: None,
            title: strings::lookup(Text::PluginsDisable).into(),
            secondary: Vec::new(),
            press: crate::widget::list::Press::Whole(Message::DashboardAction(
                super::Action::Write(super::Written::DisablePlugin {
                    id: open,
                    version: version.clone(),
                    name: name.clone(),
                }),
            )),
            controls: Vec::new(),
        });
        rows.push(crate::widget::list::Row {
            face: Some(crate::widget::list::Face::Glyph(crate::icon::Icon::Delete)),
            index: None,
            title: strings::lookup(Text::PluginsUninstall).into(),
            secondary: Vec::new(),
            press: crate::widget::list::Press::Whole(Message::DashboardAction(super::Action::Ask(
                crate::screen::confirm::Pending::of(
                    crate::screen::confirm::Destructive::UninstallPlugin { id: open, version },
                    name,
                ),
            ))),
            controls: Vec::new(),
        });
    }
    crate::widget::list::listed(space::ListRow::glyph(space::Lines::One), rows)
}

/// Every plugin on the card `PluginCard` draws, in the grid its own ladder
/// lays them in, with the catalog and repository links above and the open
/// plugin's commands under them.
// the reference draws a plugin's own image where the server holds one; this
// client draws the extension glyph for every plugin
// reference: dashboard-plugins-grid
// reference: dashboard-plugin-card
pub fn view<'a>(
    state: &'a State,
    read_only: bool,
    viewport: style::Viewport,
) -> Vec<Element<'a, Message>> {
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

    if let Some(open) = state.menu {
        listed.push(menu(state, open, read_only));
    }

    listed.push(crate::widget::mui::grid(
        space::PLUGIN_CELL,
        state.plugins.iter().filter_map(|plugin| {
            let id = plugin.id?;
            Some(crate::widget::mui::card(
                crate::widget::mui::Card {
                    title: plugin.name.clone().unwrap_or_default().into(),
                    text: Some(described(plugin).into()),
                    media: crate::widget::mui::Media::Glyph(
                        crate::icon::Icon::Extension,
                        typeface::PLUGIN_CARD_ICON,
                    ),
                    height: space::BASE_CARD,
                    opens: Some(Message::DashboardAction(super::Action::PluginMenu(
                        match state.menu == Some(id) {
                            true => None,
                            false => Some(id),
                        },
                    ))),
                    action: None,
                },
                viewport.band(),
            ))
        }),
        viewport,
    ));

    listed
}
