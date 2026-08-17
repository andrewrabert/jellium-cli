//! Every package the configured repositories offer, and the installs running
//! now.

use iced::widget::{button, column, row};
use iced::{Element, Fill};

use crate::app::Message;
use crate::error::Answer;
use crate::style::Drawn;
use crate::style::typeface;
use crate::text::{self as strings, Text};
use crate::theme;
use crate::widget::prose;
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

pub async fn load(api: std::rc::Rc<crate::api::Api>, height: Drawn) -> Answer<State> {
    Answer::of(async {
        Ok(State {
            packages: api.packages().await.bubbled()?,
            window: window::Window::new(window::Id::Catalog, Drawn::of(theme::ROW_HEIGHT), height),
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
        prose(
            strings::lookup(Text::CatalogTitle).to_owned(),
            typeface::HEADING_2
        ),
        window::list(state.window, state.packages.len(), move |index| {
            let Some(package) = state.packages.get(index) else {
                return prose(String::new(), typeface::BODY);
            };
            let name = package.name.clone().unwrap_or_default();
            let chosen = version(state, package);
            let repository = package
                .versions
                .first()
                .and_then(|version| version.repository_name.clone())
                .unwrap_or_default();

            let mut held = column![
                prose(name.clone(), typeface::BODY),
                prose(
                    package.description.clone().unwrap_or_default(),
                    typeface::BODY
                ),
                prose(
                    strings::format(Text::CatalogVersions, &[&chosen]),
                    typeface::BODY
                ),
            ]
            .spacing(theme::CARD_SPACING);

            if state.installing.contains_key(&name) {
                let mut running = row![prose(
                    strings::lookup(Text::CatalogInstalling).to_owned(),
                    typeface::BODY
                )]
                .spacing(theme::CARD_SPACING);
                if !read_only && let Some(plugin) = state.installing[&name].plugin {
                    running = running.push(
                        button(prose(
                            strings::lookup(Text::CatalogCancel).to_owned(),
                            typeface::BODY,
                        ))
                        .on_press(Message::DashboardAction(
                            super::Action::Write(super::Written::CancelInstall {
                                package: plugin,
                                name: name.clone(),
                            }),
                        )),
                    );
                }
                held = held.push(running);
            } else if !read_only {
                held = held.push(
                    button(prose(
                        strings::lookup(Text::CatalogInstall).to_owned(),
                        typeface::BODY,
                    ))
                    .on_press(Message::DashboardAction(super::Action::Ask(
                        crate::screen::confirm::Pending::of(
                            crate::screen::confirm::Destructive::InstallPackage {
                                name: name.clone(),
                                version: chosen.clone(),
                                repository: repository.clone(),
                            },
                            name.clone(),
                        ),
                    ))),
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
