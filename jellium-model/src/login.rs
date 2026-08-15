//! The login stage's screens and what each of Jellyfin's password-reset
//! answers offers.

/// Which screen the login stage is showing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Servers,
    Add,
    Credentials,
    QuickConnect,
    Reset,
}

impl Screen {
    /// The screen Back reaches, and `None` on the server list, where Back
    /// reaches nothing.
    pub fn back(self) -> Option<Screen> {
        match self {
            Screen::Servers => None,
            Screen::Add | Screen::Credentials => Some(Screen::Servers),
            Screen::QuickConnect | Screen::Reset => Some(Screen::Credentials),
        }
    }

    /// True while a login target is held, which is every screen a server was
    /// chosen for.
    pub fn targeted(self) -> bool {
        matches!(
            self,
            Screen::Credentials | Screen::QuickConnect | Screen::Reset
        )
    }
}

/// Which of Jellyfin's three password-reset answers is shown.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Reset {
    PinWritten,
    ContactAdministrator,
    InNetworkRequired,
}

impl Reset {
    /// True for the one answer that takes a pin, which is what puts the pin
    /// field on the screen.
    pub fn takes_pin(self) -> bool {
        matches!(self, Reset::PinWritten)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_screen_but_the_list_walks_back_to_the_one_it_opened_from() {
        assert_eq!(Screen::Servers.back(), None);
        assert_eq!(Screen::Add.back(), Some(Screen::Servers));
        assert_eq!(Screen::Credentials.back(), Some(Screen::Servers));
        assert_eq!(Screen::QuickConnect.back(), Some(Screen::Credentials));
        assert_eq!(Screen::Reset.back(), Some(Screen::Credentials));
    }

    #[test]
    fn a_login_target_is_held_on_every_screen_but_the_list_and_add_server() {
        assert!(!Screen::Servers.targeted());
        assert!(!Screen::Add.targeted());
        assert!(Screen::Credentials.targeted());
        assert!(Screen::QuickConnect.targeted());
        assert!(Screen::Reset.targeted());
    }

    #[test]
    fn the_pin_field_follows_the_pin_answer_alone() {
        assert!(Reset::PinWritten.takes_pin());
        assert!(!Reset::ContactAdministrator.takes_pin());
        assert!(!Reset::InNetworkRequired.takes_pin());
    }
}
