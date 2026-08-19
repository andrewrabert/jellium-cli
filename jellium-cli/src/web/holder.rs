use std::path::PathBuf;
use std::sync::Arc;

use jellium_protocol::Failure;

use super::link::Link;
use super::login::Login;
use super::setup::Setup;
use super::upstream::Upstream;

/// The one upstream the local server holds.
#[derive(Clone)]
pub enum Held {
    /// The saved server whose login screen is open, reached with no
    /// credential and admitting no relayed route at all.
    Login(Arc<Login>),
    /// A Jellyfin server in startup mode, reached with no credential.
    Setup(Arc<Setup>),
    Signed(Arc<Upstream>),
}

impl Held {
    /// The link this upstream's requests are issued over.
    pub fn link(&self) -> &Link {
        match self {
            Held::Login(login) => login.link(),
            Held::Setup(setup) => setup.link(),
            Held::Signed(upstream) => upstream.link(),
        }
    }

    /// Which stage the local server is in.
    pub fn admits(&self) -> jellium_protocol::Admits {
        match self {
            Held::Login(_) => jellium_protocol::Admits::Login,
            Held::Setup(_) => jellium_protocol::Admits::Setup,
            Held::Signed(_) => jellium_protocol::Admits::Signed,
        }
    }
}

/// What removing a saved server did.
pub enum Removed {
    Deleted,
    DeletedUnrevoked,
    Unknown,
}

/// The one Jellyfin session and the session file it shares with the command
/// line, held together so the two never disagree.
pub struct Holder {
    path: PathBuf,
    current: tokio::sync::RwLock<Option<Held>>,
    /// Every change to the pair is taken under this, so a sign-in and a revoke
    /// never interleave.
    transition: tokio::sync::Mutex<()>,
}

fn report(outcome: Result<(), crate::session::SessionError>) {
    if let Err(e) = outcome {
        eprintln!("jellium-cli web: {e}");
    }
}

/// A new session starts with no media control; the event socket declares it
/// when a tab brings one up.
async fn declare(upstream: &Upstream) {
    if let Err(e) = upstream
        .declare_capabilities(false, upstream.state.live_tv.allowed())
        .await
    {
        eprintln!("jellium-cli web: {e:?}");
    }
}

/// The outcome of installing a session resumed from the session file.
pub enum Resumed {
    /// The resumed session is the held one.
    Installed(Arc<Upstream>),

    /// The pair moved while the resume ran, so the resumed session was
    /// dropped; the session held now, when there is one.
    Superseded(Option<Arc<Upstream>>),
}

impl Holder {
    pub fn new(path: PathBuf) -> Holder {
        Holder {
            path,
            current: tokio::sync::RwLock::new(None),
            transition: tokio::sync::Mutex::new(()),
        }
    }

    /// Whatever is held now, and `None` when nothing is.
    /// No lock is held once it returns, so the caller may await with the
    /// upstream in hand.
    pub async fn held(&self) -> Option<Held> {
        self.current.read().await.clone()
    }

    /// The signed session held now, and `None` while a setup upstream is held
    /// or nothing is.
    pub async fn signed(&self) -> Option<Arc<Upstream>> {
        match self.current.read().await.clone() {
            Some(Held::Signed(upstream)) => Some(upstream),
            Some(Held::Login(_) | Held::Setup(_)) | None => None,
        }
    }

    /// The setup upstream held now, and `None` while a session is held or
    /// nothing is.
    pub async fn setup(&self) -> Option<Arc<Setup>> {
        match self.current.read().await.clone() {
            Some(Held::Setup(setup)) => Some(setup),
            Some(Held::Login(_) | Held::Signed(_)) | None => None,
        }
    }

    /// The login target held now, and `None` while anything else is held or
    /// nothing is.
    pub async fn login(&self) -> Option<Arc<Login>> {
        match self.current.read().await.clone() {
            Some(Held::Login(login)) => Some(login),
            Some(Held::Setup(_) | Held::Signed(_)) | None => None,
        }
    }

    /// Revokes and clears the session it displaces, then holds `setup`.
    /// Writes nothing: a setup upstream never reaches the session file.
    pub async fn enter_setup(&self, setup: Setup) -> Arc<Setup> {
        let _transition = self.transition.lock().await;
        if let Some(Held::Signed(displaced)) = self.current.read().await.clone() {
            if let Err(e) = displaced.logout().await {
                eprintln!("jellium-cli web: {e:?}");
            }
            self.clear(&displaced.state.server).await;
        }
        let setup = Arc::new(setup);
        *self.current.write().await = Some(Held::Setup(setup.clone()));
        setup
    }

    /// Drops the setup upstream, leaving the session file as found.
    pub async fn leave_setup(&self) {
        let _transition = self.transition.lock().await;
        let mut current = self.current.write().await;
        if matches!(*current, Some(Held::Setup(_))) {
            *current = None;
        }
    }

    /// Releases whatever is held, tearing a session down the way a switch
    /// does, and holds `login`; writes nothing, since a login target never
    /// reaches the session file.
    pub async fn enter_login(&self, login: Login) -> Arc<Login> {
        let _transition = self.transition.lock().await;
        let login = Arc::new(login);
        *self.current.write().await = Some(Held::Login(login.clone()));
        login
    }

    /// Drops the login target, leaving the session file as found.
    pub async fn leave_login(&self) {
        let _transition = self.transition.lock().await;
        let mut current = self.current.write().await;
        if matches!(*current, Some(Held::Login(_))) {
            *current = None;
        }
    }

    /// Releases the held session without revoking it, so the server left keeps
    /// its credential and is re-entered without a password.
    pub async fn switch(&self) {
        let _transition = self.transition.lock().await;
        *self.current.write().await = None;
    }

    /// Every saved server, read from the session file on the blocking pool; a
    /// file that cannot be read is reported on stderr and reads as empty.
    pub async fn records(&self) -> Vec<crate::session::Saved> {
        crate::session::SessionFile::load_async(self.path.clone())
            .await
            .map_err(|e| eprintln!("jellium-cli web: {e}"))
            .map(|file| file.records())
            .unwrap_or_default()
    }

    /// Applies `change` to the session file on the blocking pool; a write that
    /// fails is reported on stderr and changes nothing.
    pub async fn write<F>(&self, change: F)
    where
        F: FnOnce(&mut crate::session::SessionFile) + Send + 'static,
    {
        report(crate::session::SessionFile::update_async(self.path.clone(), change).await);
    }

    /// Writes the name `server` reported, at the record it is saved at and
    /// only when it differs from the one stored.
    pub async fn named(&self, server: &str, name: &str) {
        let server = server.to_string();
        let name = name.to_string();
        report(
            crate::session::SessionFile::update_async(self.path.clone(), move |file| {
                if let Some(record) = file.find(&server) {
                    file.set_name(record, &name);
                }
            })
            .await,
        );
    }

    /// Revokes the record's token upstream, then deletes the record and
    /// compacts the ones behind it; a revoke the Jellyfin server refuses still
    /// deletes and answers `Removed::DeletedUnrevoked`.
    /// Removing the server held now releases it first.
    /// The revoke is issued under `identity`, which is what the browser
    /// announced.
    pub async fn remove(&self, identity: &super::identity::Identity, server: &str) -> Removed {
        let _transition = self.transition.lock().await;
        let wanted = crate::session::normalized(server);
        let held = self.current.read().await.clone();
        let releases = match &held {
            Some(Held::Signed(upstream)) => {
                crate::session::normalized(&upstream.state.server) == wanted
            }
            Some(Held::Login(login)) => crate::session::normalized(login.server()) == wanted,
            Some(Held::Setup(_)) | None => false,
        };

        let record = {
            let owned = server.to_string();
            match crate::session::SessionFile::load_async(self.path.clone()).await {
                Ok(file) => match file.find(&owned) {
                    Some(record) => file.saved(record),
                    None => None,
                },
                Err(e) => {
                    eprintln!("jellium-cli web: {e}");
                    None
                }
            }
        };
        let Some(record) = record else {
            return Removed::Unknown;
        };

        let mut revoked = true;
        if let Some(session) = record.session() {
            revoked = match &held {
                Some(Held::Signed(upstream)) if releases => upstream.logout().await.is_ok(),
                _ => super::upstream::revoked(identity, &session).await,
            };
        }
        if releases {
            *self.current.write().await = None;
        }

        let owned = server.to_string();
        report(
            crate::session::SessionFile::update_async(self.path.clone(), move |file| {
                if let Some(record) = file.find(&owned) {
                    file.remove_server(record);
                }
            })
            .await,
        );

        if revoked {
            Removed::Deleted
        } else {
            Removed::DeletedUnrevoked
        }
    }

    /// The active server's session, and `None` when record 0 holds no
    /// credential.
    pub async fn saved(&self) -> Option<crate::session::Session> {
        crate::session::SessionFile::load_async(self.path.clone())
            .await
            .map_err(|e| eprintln!("jellium-cli web: {e}"))
            .ok()?
            .active()
    }

    /// Revokes the session it displaces, writes the session file and installs
    /// `upstream`, all under the transition lock. A displaced session the
    /// Jellyfin server will not revoke is reported on stderr and the install
    /// stands. A session file that cannot be written is reported on stderr and
    /// the session is installed anyway.
    /// Declares the client's capabilities on the session it installs; a
    /// refusal is reported on stderr and the install stands.
    pub async fn install(&self, upstream: Upstream) -> Arc<Upstream> {
        let _transition = self.transition.lock().await;
        let displaced = self.current.read().await.clone();
        if let Some(Held::Signed(displaced)) = displaced
            && let Err(e) = displaced.logout().await
        {
            eprintln!("jellium-cli web: {e:?}");
        }
        let record = upstream.session();
        let name = upstream.name().to_string();
        report(
            crate::session::SessionFile::update_async(self.path.clone(), move |file| {
                file.set_server(&record);
                if !name.is_empty() {
                    file.set_name(0, &name);
                }
            })
            .await,
        );
        declare(&upstream).await;
        let upstream = Arc::new(upstream);
        *self.current.write().await = Some(Held::Signed(upstream.clone()));
        upstream
    }

    /// Installs `upstream` under the transition lock, and only while the
    /// premise the resume was decided on still holds: nothing is held, and the
    /// session file still carries `resumed`. Writes nothing, since the record
    /// it installs is the record on file.
    /// Declares the client's capabilities on the session it installs.
    pub async fn resumed(&self, resumed: &crate::session::Session, upstream: Upstream) -> Resumed {
        let _transition = self.transition.lock().await;
        match self.current.read().await.clone() {
            Some(Held::Signed(held)) => return Resumed::Superseded(Some(held)),
            Some(Held::Login(_) | Held::Setup(_)) => return Resumed::Superseded(None),
            None => {}
        }
        let on_file = crate::session::SessionFile::load_async(self.path.clone())
            .await
            .map_err(|e| eprintln!("jellium-cli web: {e}"))
            .ok()
            .and_then(|file| file.active());
        if on_file.as_ref() != Some(resumed) {
            return Resumed::Superseded(None);
        }
        declare(&upstream).await;
        let upstream = Arc::new(upstream);
        *self.current.write().await = Some(Held::Signed(upstream.clone()));
        Resumed::Installed(upstream)
    }

    /// Revokes the held token on the Jellyfin server, then clears the session
    /// file and the held session, all under the transition lock. A revoke with
    /// nothing held is a no-op that reaches no server and leaves the session
    /// file as found. A revoke that fails changes nothing and reports the
    /// `Failure`.
    pub async fn revoke(&self) -> Result<(), Failure> {
        let _transition = self.transition.lock().await;
        let Some(Held::Signed(upstream)) = self.current.read().await.clone() else {
            return Ok(());
        };
        upstream.logout().await?;
        self.clear(&upstream.state.server).await;
        Ok(())
    }

    /// Clears the held session and the session file when `rejected` is still
    /// the held session, so a sign-in that landed after it is left alone.
    pub async fn reject(&self, rejected: &Arc<Upstream>) {
        let _transition = self.transition.lock().await;
        let stale = matches!(
            self.current.read().await.as_ref(),
            Some(Held::Signed(held)) if Arc::ptr_eq(held, rejected)
        );
        if stale {
            self.clear(&rejected.state.server).await;
        }
    }

    /// Clears `server`'s credential and leaves the record saved, then releases
    /// whatever is held.
    async fn clear(&self, server: &str) {
        let server = server.to_string();
        report(
            crate::session::SessionFile::update_async(self.path.clone(), move |file| {
                if let Some(record) = file.find(&server) {
                    file.clear_credential(record);
                }
            })
            .await,
        );
        *self.current.write().await = None;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::web::upstream::{Upstream, answering};

    fn device() -> crate::web::identity::Identity {
        crate::web::identity::Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: uuid::Uuid::nil().to_string(),
        })
    }

    fn holder(path: PathBuf) -> Holder {
        Holder::new(path)
    }

    fn scratch(name: &str) -> PathBuf {
        let path = std::env::temp_dir()
            .join("jellium-cli-holder-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&path);
        std::fs::create_dir_all(&path).expect("scratch directory");
        path.join("session.env")
    }

    #[tokio::test]
    async fn a_resume_landing_after_a_sign_in_is_superseded() {
        let server = answering(200).await;
        let holder = holder(scratch("resume-after-sign-in"));
        holder.install(Upstream::stub(&server.base)).await;

        let resumed = Upstream::stub(&server.base);
        let record = resumed.session();
        assert!(matches!(
            holder.resumed(&record, resumed).await,
            Resumed::Superseded(Some(_))
        ));
    }

    #[tokio::test]
    async fn a_resume_landing_after_a_revoke_leaves_the_session_file_empty() {
        let server = answering(200).await;
        let holder = holder(scratch("resume-after-revoke"));
        holder.install(Upstream::stub(&server.base)).await;
        holder.revoke().await.expect("revoke");

        let resumed = Upstream::stub(&server.base);
        let record = resumed.session();
        assert!(matches!(
            holder.resumed(&record, resumed).await,
            Resumed::Superseded(None)
        ));
        assert!(holder.saved().await.is_none());
    }

    #[tokio::test]
    async fn a_resume_of_the_record_on_file_is_installed() {
        let server = answering(200).await;
        let path = scratch("resume-of-record");
        let record = Upstream::stub(&server.base).session();
        crate::session::SessionFile::update_async(path.clone(), {
            let record = record.clone();
            move |file| file.set_server(&record)
        })
        .await
        .expect("seed record");

        let holder = holder(path);
        let resumed = Upstream::stub(&server.base);
        assert!(matches!(
            holder.resumed(&record, resumed).await,
            Resumed::Installed(_)
        ));
        assert!(holder.held().await.is_some());
    }

    #[tokio::test]
    async fn a_sign_in_revokes_the_session_it_displaces() {
        let server = answering(200).await;
        let holder = holder(scratch("sign-in-revokes"));
        holder.install(Upstream::stub(&server.base)).await;
        holder.install(Upstream::stub(&server.base)).await;
        assert_eq!(server.asked("/Sessions/Logout"), 1);
    }

    #[tokio::test]
    async fn an_installed_session_declares_its_capabilities() {
        let server = answering(204).await;
        let holder = holder(scratch("install-declares"));
        holder.install(Upstream::stub(&server.base)).await;
        assert_eq!(server.asked("/Sessions/Capabilities"), 1);
    }

    #[tokio::test]
    async fn a_revoke_with_nothing_held_leaves_the_session_file_untouched() {
        let server = answering(200).await;
        let path = scratch("revoke-nothing-held");
        let record = Upstream::stub(&server.base).session();
        crate::session::SessionFile::update_async(path.clone(), {
            let record = record.clone();
            move |file| file.set_server(&record)
        })
        .await
        .expect("seed record");

        let holder = holder(path);
        holder.revoke().await.expect("revoke");
        assert!(holder.saved().await.is_some());
        assert!(holder.held().await.is_none());
        assert_eq!(server.requests.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn a_revoke_clears_the_credential_and_leaves_the_server_saved() {
        let server = answering(401).await;
        let holder = holder(scratch("revoke-refused"));
        holder.install(Upstream::stub(&server.base)).await;
        holder.revoke().await.expect("revoke");
        assert!(holder.saved().await.is_none());
        assert!(holder.held().await.is_none());
        assert_eq!(holder.records().await.len(), 1);
        assert!(holder.records().await[0].credential.is_none());
    }

    #[tokio::test]
    async fn a_switch_keeps_the_credential_of_the_server_it_leaves() {
        let server = answering(200).await;
        let holder = holder(scratch("switch-keeps"));
        holder.install(Upstream::stub(&server.base)).await;
        holder.switch().await;
        assert!(holder.held().await.is_none());
        assert!(holder.saved().await.is_some());
        assert_eq!(server.asked("/Sessions/Logout"), 0);
    }

    #[tokio::test]
    async fn a_sign_in_moves_its_record_to_the_front() {
        let server = answering(200).await;
        let holder = holder(scratch("sign-in-fronts"));
        holder
            .write(|file| file.add_server("https://other.test"))
            .await;
        holder.install(Upstream::stub(&server.base)).await;
        let records = holder.records().await;
        assert_eq!(records.len(), 2);
        assert!(records[0].credential.is_some());
        assert_eq!(records[1].server, "https://other.test");
    }

    #[tokio::test]
    async fn removing_a_record_a_server_will_not_revoke_still_deletes_it() {
        let server = answering(500).await;
        let holder = holder(scratch("remove-unrevoked"));
        holder.install(Upstream::stub(&server.base)).await;
        let removed = holder.remove(&device(), &server.base).await;
        assert!(matches!(removed, Removed::DeletedUnrevoked));
        assert!(holder.records().await.is_empty());
        assert!(holder.held().await.is_none());
    }

    #[tokio::test]
    async fn entering_a_login_target_releases_the_session_it_displaces() {
        let server = answering(200).await;
        let holder = holder(scratch("enter-login"));
        holder.install(Upstream::stub(&server.base)).await;
        let device = crate::web::identity::Identity::of(jellium_protocol::Identity {
            device: "Firefox".to_owned(),
            device_id: uuid::Uuid::nil().to_string(),
        });
        let login = crate::web::login::Login::of(
            &device,
            &server.base,
            &crate::web::version::Probed {
                version: "10.10.0".to_string(),
                name: String::new(),
                startup: false,
            },
            "",
            false,
        )
        .expect("a login target");
        holder.enter_login(login).await;
        assert!(holder.signed().await.is_none());
        assert!(holder.login().await.is_some());
        assert!(holder.saved().await.is_some());
        assert_eq!(server.asked("/Sessions/Logout"), 0);
    }

    #[tokio::test]
    async fn a_revoke_a_server_does_not_accept_keeps_the_session_file() {
        let server = answering(500).await;
        let holder = holder(scratch("revoke-unreachable"));
        holder.install(Upstream::stub(&server.base)).await;
        assert!(matches!(
            holder.revoke().await,
            Err(Failure::ServerUnreachable { .. })
        ));
        assert!(holder.saved().await.is_some());
        assert!(holder.held().await.is_some());
    }
}
