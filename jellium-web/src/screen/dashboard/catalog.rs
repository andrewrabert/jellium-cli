//! Every package the configured repositories offer, and the installs running
//! now.

use iced::widget::{button, column, row, text};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::window;
use jellium_protocol::{Event, Packaged};

/// Every package the configured repositories offer, windowed.
#[derive(Debug, Clone)]
pub struct State {
    pub packages: Vec<jellyfin_api::types::PackageInfo>,
    pub window: window::Window,
    /// The version chosen for each package the user has opened.
    pub versions: std::collections::HashMap<String, String>,
    /// The installs running now, by package name.
    pub installing: std::collections::HashMap<String, Packaged>,
}

pub async fn load(api: std::rc::Rc<crate::api::Api>, height: f32) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            packages: api.packages().await.bubbled()?,
            window: window::Window::new(window::Id::Catalog, theme::ROW_HEIGHT, height),
            versions: std::collections::HashMap::new(),
            installing: std::collections::HashMap::new(),
        })
    })
    .await
}

/// Applies one of the five package messages in place.
pub fn packaged(state: &mut State, event: &Event) {
    match event {
        Event::PackageInstalling { package } => {
            state
                .installing
                .insert(package.name.clone(), package.clone());
        }
        Event::PackageInstalled { package }
        | Event::PackageFailed { package }
        | Event::PackageCancelled { package }
        | Event::PackageUninstalled { package } => {
            state.installing.remove(&package.name);
        }
        _ => {}
    }
}

/// The newest version a package offers, and the one chosen when the user has
/// chosen one.
fn version<'a>(state: &'a State, package: &'a jellyfin_api::types::PackageInfo) -> String {
    let name = package.name.clone().unwrap_or_default();
    state
        .versions
        .get(&name)
        .cloned()
        .or_else(|| {
            package
                .versions
                .first()
                .and_then(|version| version.version.clone())
        })
        .unwrap_or_default()
}

/// Each package with its name, description and versions, its install control
/// behind a confirmation, and a running install's cancel.
pub fn view<'a>(state: &'a State, read_only: bool) -> Element<'a, Message> {
    column![
        text(strings::lookup(Text::CatalogTitle)).size(22),
        window::list(state.window, state.packages.len(), move |index| {
            let Some(package) = state.packages.get(index) else {
                return text("").into();
            };
            let name = package.name.clone().unwrap_or_default();
            let chosen = version(state, package);
            let repository = package
                .versions
                .first()
                .and_then(|version| version.repository_name.clone())
                .unwrap_or_default();

            let mut held = column![
                text(name.clone()),
                text(package.description.clone().unwrap_or_default()),
                text(strings::format(Text::CatalogVersions, &[&chosen])),
            ]
            .spacing(theme::CARD_SPACING);

            if state.installing.contains_key(&name) {
                let mut running = row![text(strings::lookup(Text::CatalogInstalling))]
                    .spacing(theme::CARD_SPACING);
                if !read_only && let Some(plugin) = state.installing[&name].plugin {
                    running =
                        running.push(button(text(strings::lookup(Text::CatalogCancel))).on_press(
                            Message::DashboardAction(super::Action::Write(
                                super::Written::CancelInstall {
                                    package: plugin,
                                    name: name.clone(),
                                },
                            )),
                        ));
                }
                held = held.push(running);
            } else if !read_only {
                held = held.push(
                    button(text(strings::lookup(Text::CatalogInstall))).on_press(
                        Message::DashboardAction(super::Action::Ask(
                            crate::screen::confirm::Pending::of(
                                crate::screen::confirm::Destructive::InstallPackage {
                                    name: name.clone(),
                                    version: chosen.clone(),
                                    repository: repository.clone(),
                                },
                                name.clone(),
                            ),
                        )),
                    ),
                );
            }
            held.into()
        }),
    ]
    .spacing(theme::CARD_SPACING)
    .padding(theme::CARD_SPACING)
    .width(Fill)
    .height(Fill)
    .into()
}
