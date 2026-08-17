//! One confirmation rule with two tiers, serving both the dashboard and the
//! settings region.

use iced::Element;
use iced::widget::{button, column, row, text_input};
use uuid::Uuid;

use crate::app::Message;
use crate::style::{self, space, typeface};
use crate::text::{self as strings, Text};
use crate::widget::prose;

/// Which region's action a confirmation's controls raise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Region {
    Dashboard,
    Settings,
    Metadata,
}

impl Region {
    fn typed(self, typed: String) -> Message {
        match self {
            Region::Dashboard => {
                Message::DashboardAction(crate::screen::dashboard::Action::Typed(typed))
            }
            Region::Settings => {
                Message::SettingsAction(crate::screen::settings::Action::Typed(typed))
            }
            Region::Metadata => {
                Message::MetadataAction(crate::screen::metadata::Action::Typed(typed))
            }
        }
    }

    fn confirm(self) -> Message {
        match self {
            Region::Dashboard => {
                Message::DashboardAction(crate::screen::dashboard::Action::Confirm)
            }
            Region::Settings => Message::SettingsAction(crate::screen::settings::Action::Confirm),
            Region::Metadata => Message::MetadataAction(crate::screen::metadata::Action::Confirm),
        }
    }

    fn close(self) -> Message {
        match self {
            Region::Dashboard => Message::DashboardAction(crate::screen::dashboard::Action::Close),
            Region::Settings => Message::SettingsAction(crate::screen::settings::Action::Close),
            Region::Metadata => Message::MetadataAction(crate::screen::metadata::Action::Close),
        }
    }
}

/// What a destructive action must be confirmed with.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Tier {
    /// A second press.
    Press,
    /// The object's own name typed exactly.
    Typed,
}

/// Every destructive action, each behind a confirmation naming its object and
/// stating what is lost.
#[derive(Debug, Clone, PartialEq)]
pub enum Destructive {
    /// Requires the user's name typed.
    DeleteUser {
        id: Uuid,
    },
    /// Requires the virtual folder's name typed.
    DeleteLibrary {
        name: String,
    },
    /// Requires the item's name typed; the item leaves the library and the
    /// filesystem, which re-adding it does not undo.
    DeleteItem {
        id: Uuid,
    },
    DeletePath {
        library: String,
        path: String,
    },
    StopTask {
        id: String,
    },
    InstallPackage {
        name: String,
        version: String,
        repository: String,
    },
    AddRepository {
        name: String,
        url: String,
    },
    RemoveRepository {
        url: String,
    },
    DeleteDevice {
        id: String,
        /// True for this installation's own device, whose deletion ends this
        /// session.
        own: bool,
    },
    RevokeKey {
        key: String,
    },
    DeleteTuner {
        id: String,
    },
    DeleteProvider {
        id: String,
    },
    RemoveUserImage {
        id: Uuid,
    },
    UninstallPlugin {
        id: Uuid,
        version: String,
    },
    /// Authorizes a Quick Connect code; the sentence names the code and states
    /// that another device gains full access to this account.
    AuthorizeQuickConnect {
        code: String,
    },
    Restart,
    Shutdown,
}

impl Destructive {
    /// What confirming this action takes: an object whose loss cannot be
    /// undone by re-adding it demands its name typed, and everything else a
    /// second press.
    fn tier(&self) -> Tier {
        match self {
            Destructive::DeleteUser { .. }
            | Destructive::DeleteLibrary { .. }
            | Destructive::DeleteItem { .. } => Tier::Typed,
            Destructive::DeletePath { .. }
            | Destructive::StopTask { .. }
            | Destructive::InstallPackage { .. }
            | Destructive::AddRepository { .. }
            | Destructive::RemoveRepository { .. }
            | Destructive::DeleteDevice { .. }
            | Destructive::RevokeKey { .. }
            | Destructive::DeleteTuner { .. }
            | Destructive::DeleteProvider { .. }
            | Destructive::RemoveUserImage { .. }
            | Destructive::UninstallPlugin { .. }
            | Destructive::AuthorizeQuickConnect { .. }
            | Destructive::Restart
            | Destructive::Shutdown => Tier::Press,
        }
    }
}

/// One destructive action, its object and what confirming it takes.
#[derive(Debug, Clone, PartialEq)]
pub struct Pending {
    pub action: Destructive,
    /// The object the sentence names, and what `Tier::Typed` must be matched
    /// against.
    pub object: String,
    pub tier: Tier,
    /// What has been typed so far.
    pub typed: String,
}

impl Pending {
    /// The action `destructive` is asked about as, at the tier its object
    /// demands.
    pub fn of(destructive: Destructive, object: String) -> Pending {
        Pending {
            tier: destructive.tier(),
            action: destructive,
            object,
            typed: String::new(),
        }
    }

    /// True once the confirmation is satisfied and the action may proceed.
    pub fn ready(&self) -> bool {
        match self.tier {
            Tier::Press => true,
            Tier::Typed => self.typed == self.object,
        }
    }

    /// The sentence shown, naming the object and what is lost.
    pub fn sentence(&self) -> String {
        match &self.action {
            Destructive::DeleteUser { .. } => {
                strings::format(Text::ConfirmDeleteUser, &[&self.object])
            }
            Destructive::DeleteLibrary { .. } => {
                strings::format(Text::ConfirmDeleteLibrary, &[&self.object])
            }
            Destructive::DeleteItem { .. } => {
                strings::format(Text::ConfirmDeleteItem, &[&self.object])
            }
            Destructive::DeletePath { library, .. } => {
                strings::format(Text::ConfirmDeletePath, &[&self.object, library])
            }
            Destructive::StopTask { .. } => strings::format(Text::ConfirmStopTask, &[&self.object]),
            Destructive::InstallPackage {
                version,
                repository,
                ..
            } => strings::format(
                Text::ConfirmInstallPackage,
                &[&self.object, version, repository],
            ),
            Destructive::AddRepository { .. } => {
                strings::format(Text::ConfirmAddRepository, &[&self.object])
            }
            Destructive::RemoveRepository { .. } => {
                strings::format(Text::ConfirmRemoveRepository, &[&self.object])
            }
            Destructive::DeleteDevice { own, .. } => strings::format(
                if *own {
                    Text::ConfirmDeleteOwnDevice
                } else {
                    Text::ConfirmDeleteDevice
                },
                &[&self.object],
            ),
            Destructive::RevokeKey { .. } => {
                strings::format(Text::ConfirmRevokeKey, &[&self.object])
            }
            Destructive::DeleteTuner { .. } => {
                strings::format(Text::ConfirmDeleteTuner, &[&self.object])
            }
            Destructive::DeleteProvider { .. } => {
                strings::format(Text::ConfirmDeleteProvider, &[&self.object])
            }
            Destructive::RemoveUserImage { .. } => {
                strings::format(Text::ConfirmRemoveUserImage, &[&self.object])
            }
            Destructive::UninstallPlugin { version, .. } => {
                strings::format(Text::ConfirmUninstallPlugin, &[&self.object, version])
            }
            Destructive::AuthorizeQuickConnect { .. } => {
                strings::format(Text::ConfirmAuthorizeQuickConnect, &[&self.object])
            }
            Destructive::Restart => strings::format(Text::ConfirmRestart, &[&self.object]),
            Destructive::Shutdown => strings::format(Text::ConfirmShutdown, &[&self.object]),
        }
    }
}

/// The confirmation drawn in the acting control's place: its sentence, the name
/// field for `Tier::Typed`, and the two controls, each raising `region`'s own
/// action.
pub fn view<'a>(pending: &'a Pending, region: Region) -> Element<'a, Message> {
    let mut shown = column![prose(pending.sentence(), typeface::BODY)]
        .spacing(style::drawn(space::GUTTER.drawn()));

    if pending.tier == Tier::Typed {
        shown = shown.push(
            text_input(strings::lookup(Text::ConfirmTypeName), &pending.typed)
                .style(style::input)
                .on_input(move |typed| region.typed(typed)),
        );
    }

    let mut proceed =
        button(prose(strings::lookup(Text::ConfirmProceed), typeface::BODY)).style(style::submit);
    if pending.ready() {
        proceed = proceed.on_press(region.confirm());
    }

    shown
        .push(
            row![
                proceed,
                button(prose(strings::lookup(Text::ConfirmCancel), typeface::BODY))
                    .style(style::raised)
                    .on_press(region.close()),
            ]
            .spacing(style::drawn(space::GUTTER.drawn())),
        )
        .into()
}
