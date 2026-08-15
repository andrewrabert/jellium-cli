//! What a Quick Connect authorize answered with.

/// What the Jellyfin server made of an authorize.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    Authorized,
    /// The server holds no request under the code and this run authorized none.
    Unknown,
    /// The server holds no request under a code this run authorized, so the
    /// request has aged out of the server's window.
    Expired,
    /// The server holds the request and it is already authorized.
    Used,
    /// The server reports Quick Connect not active.
    Disabled,
}

/// The outcome an authorize answered with: a 2xx is `Authorized`, a 401 is
/// `Disabled`, an answer whose message names an already-authorized request is
/// `Used`, and any other refusal is `Expired` when `authorized_here` and
/// `Unknown` when it is not.
pub fn outcome(status: u16, message: &str, authorized_here: bool) -> Outcome {
    if (200..300).contains(&status) {
        return Outcome::Authorized;
    }
    if status == 401 {
        return Outcome::Disabled;
    }
    if message.to_ascii_lowercase().contains("already authorized") {
        return Outcome::Used;
    }
    if authorized_here {
        Outcome::Expired
    } else {
        Outcome::Unknown
    }
}

/// Where a Quick Connect sign-in stands, as one poll answers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SignIn {
    Pending,
    Authorized,
    /// The Jellyfin server no longer holds the request.
    Expired,
    /// The Jellyfin server reports Quick Connect not active.
    Disabled,
}

/// The state one `GET /QuickConnect/Connect` answered with: a 2xx is
/// `Authorized` when the request reports itself authenticated and `Pending`
/// when it does not, a 401 is `Disabled`, and every other refusal is
/// `Expired`.
pub fn signed_in(status: u16, authenticated: bool) -> SignIn {
    if (200..300).contains(&status) {
        return if authenticated {
            SignIn::Authorized
        } else {
            SignIn::Pending
        };
    }
    if status == 401 {
        return SignIn::Disabled;
    }
    SignIn::Expired
}

/// How often the browser polls one Quick Connect request.
pub const POLL: std::time::Duration = std::time::Duration::from_secs(5);

/// True for a code shaped as the Jellyfin server issues them: six ascii digits.
pub fn shaped(code: &str) -> bool {
    code.len() == 6 && code.chars().all(|c| c.is_ascii_digit())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_success_is_authorized_and_a_401_says_quick_connect_is_off() {
        assert_eq!(outcome(204, "", false), Outcome::Authorized);
        assert_eq!(outcome(200, "", true), Outcome::Authorized);
        assert_eq!(outcome(401, "", false), Outcome::Disabled);
    }

    #[test]
    fn a_refusal_naming_an_already_authorized_request_is_used() {
        assert_eq!(
            outcome(403, "Request is already authorized", false),
            Outcome::Used
        );
    }

    #[test]
    fn a_refusal_is_expired_only_for_a_code_this_run_authorized() {
        assert_eq!(outcome(404, "not found", true), Outcome::Expired);
        assert_eq!(outcome(404, "not found", false), Outcome::Unknown);
    }

    #[test]
    fn an_unauthenticated_request_is_pending_and_an_authenticated_one_is_authorized() {
        assert_eq!(signed_in(200, false), SignIn::Pending);
        assert_eq!(signed_in(200, true), SignIn::Authorized);
    }

    #[test]
    fn a_401_is_disabled_and_every_other_refusal_is_expired() {
        assert_eq!(signed_in(401, false), SignIn::Disabled);
        assert_eq!(signed_in(404, false), SignIn::Expired);
        assert_eq!(signed_in(500, true), SignIn::Expired);
    }

    #[test]
    fn a_code_is_six_ascii_digits() {
        assert!(shaped("123456"));
        assert!(!shaped("12345"));
        assert!(!shaped("1234567"));
        assert!(!shaped("12345a"));
        assert!(!shaped("１２３４５６"));
    }
}
