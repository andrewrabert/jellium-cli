//! The setup wizard's linear steps, in jellyfin-web's order.

/// One page of the setup wizard.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    Language,
    User,
    Libraries,
    Metadata,
    RemoteAccess,
    Finish,
}

impl Step {
    /// The steps in the order they are reached, which is the only order they
    /// are reachable in.
    pub const ORDER: [Step; 6] = [
        Step::Language,
        Step::User,
        Step::Libraries,
        Step::Metadata,
        Step::RemoteAccess,
        Step::Finish,
    ];

    /// The step Next reaches, and `None` on the last one.
    pub fn next(self) -> Option<Step> {
        Step::ORDER.get(self.position()).copied()
    }

    /// The step Back reaches, and `None` on the first one, where Back leaves
    /// the wizard.
    pub fn back(self) -> Option<Step> {
        self.position()
            .checked_sub(2)
            .map(|index| Step::ORDER[index])
    }

    /// This step's one-based position, which is what the chrome states.
    pub fn position(self) -> usize {
        Step::ORDER
            .iter()
            .position(|step| *step == self)
            .expect("every step is in ORDER")
            + 1
    }

    /// True when this step writes `StartupConfigurationDto`.
    pub fn writes_configuration(self) -> bool {
        matches!(self, Step::Language | Step::Metadata)
    }
}

/// True when the first-administrator step admits Next: the name is non-empty
/// after trimming, and the confirmation matches the password.
/// No password strength rule is applied, and an empty password passes.
pub fn user_ready(name: &str, password: &str, confirmation: &str) -> bool {
    !name.trim().is_empty() && password == confirmation
}

#[cfg(test)]
mod tests {
    use super::{Step, user_ready};

    #[test]
    fn the_steps_walk_forward_and_back_through_one_order() {
        assert_eq!(Step::Language.back(), None);
        assert_eq!(Step::Finish.next(), None);
        for pair in Step::ORDER.windows(2) {
            assert_eq!(pair[0].next(), Some(pair[1]));
            assert_eq!(pair[1].back(), Some(pair[0]));
        }
        for (index, step) in Step::ORDER.iter().enumerate() {
            assert_eq!(step.position(), index + 1);
        }
    }

    #[test]
    fn the_language_and_metadata_steps_alone_write_the_configuration() {
        for step in Step::ORDER {
            assert_eq!(
                step.writes_configuration(),
                matches!(step, Step::Language | Step::Metadata),
                "{step:?}"
            );
        }
    }

    #[test]
    fn the_first_administrator_needs_a_name_and_a_matching_confirmation() {
        assert!(user_ready("root", "", ""));
        assert!(user_ready("root", "secret", "secret"));
        assert!(!user_ready("", "secret", "secret"));
        assert!(!user_ready("   ", "secret", "secret"));
        assert!(!user_ready("root", "secret", "other"));
    }
}
