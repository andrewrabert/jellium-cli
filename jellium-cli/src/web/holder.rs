use std::path::PathBuf;
use std::sync::Arc;

use jellium_protocol::Failure;

use super::upstream::Upstream;

/// The one Jellyfin session and the session file it shares with the command
/// line, held together so the two never disagree.
pub struct Holder {
    path: PathBuf,
    current: tokio::sync::RwLock<Option<Arc<Upstream>>>,
    /// Every change to the pair is taken under this, so a sign-in and a revoke
    /// never interleave.
    transition: tokio::sync::Mutex<()>,
}

fn report(outcome: Result<(), crate::session::SessionError>) {
    if let Err(e) = outcome {
        eprintln!("jellium-cli web: {e}");
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

    /// No lock is held once it returns, so the caller may await with the
    /// session in hand.
    pub async fn held(&self) -> Option<Arc<Upstream>> {
        self.current.read().await.clone()
    }

    /// The record the session file holds, read on the blocking pool. A file
    /// that cannot be read is reported on stderr and reads as absent.
    pub async fn saved(&self) -> Option<crate::session::Session> {
        crate::session::SessionFile::load_async(self.path.clone())
            .await
            .map_err(|e| eprintln!("jellium-cli web: {e}"))
            .ok()?
            .server(0)
    }

    /// Revokes the session it displaces, writes the session file and installs
    /// `upstream`, all under the transition lock. A displaced session the
    /// Jellyfin server will not revoke is reported on stderr and the install
    /// stands. A session file that cannot be written is reported on stderr and
    /// the session is installed anyway.
    pub async fn install(&self, upstream: Upstream) -> Arc<Upstream> {
        let _transition = self.transition.lock().await;
        let displaced = self.current.read().await.clone();
        if let Some(displaced) = displaced
            && let Err(e) = displaced.logout().await
        {
            eprintln!("jellium-cli web: {e:?}");
        }
        let record = upstream.session();
        report(
            crate::session::SessionFile::update_async(self.path.clone(), move |file| {
                file.set_server(0, &record);
            })
            .await,
        );
        let upstream = Arc::new(upstream);
        *self.current.write().await = Some(upstream.clone());
        upstream
    }

    /// Installs `upstream` under the transition lock, and only while the
    /// premise the resume was decided on still holds: nothing is held, and the
    /// session file still carries `resumed`. Writes nothing, since the record
    /// it installs is the record on file.
    pub async fn resumed(&self, resumed: &crate::session::Session, upstream: Upstream) -> Resumed {
        let _transition = self.transition.lock().await;
        if let Some(held) = self.current.read().await.clone() {
            return Resumed::Superseded(Some(held));
        }
        let on_file = crate::session::SessionFile::load_async(self.path.clone())
            .await
            .map_err(|e| eprintln!("jellium-cli web: {e}"))
            .ok()
            .and_then(|file| file.server(0));
        if on_file.as_ref() != Some(resumed) {
            return Resumed::Superseded(None);
        }
        let upstream = Arc::new(upstream);
        *self.current.write().await = Some(upstream.clone());
        Resumed::Installed(upstream)
    }

    /// Revokes the held token on the Jellyfin server, then clears the session
    /// file and the held session, all under the transition lock. A revoke with
    /// nothing held is a no-op that reaches no server and leaves the session
    /// file as found. A revoke that fails changes nothing and reports the
    /// `Failure`.
    pub async fn revoke(&self) -> Result<(), Failure> {
        let _transition = self.transition.lock().await;
        let Some(upstream) = self.current.read().await.clone() else {
            return Ok(());
        };
        upstream.logout().await?;
        self.clear().await;
        Ok(())
    }

    /// Clears the held session and the session file when `rejected` is still
    /// the held session, so a sign-in that landed after it is left alone.
    pub async fn reject(&self, rejected: &Arc<Upstream>) {
        let _transition = self.transition.lock().await;
        let stale = self
            .current
            .read()
            .await
            .as_ref()
            .is_some_and(|held| Arc::ptr_eq(held, rejected));
        if stale {
            self.clear().await;
        }
    }

    async fn clear(&self) {
        report(
            crate::session::SessionFile::update_async(self.path.clone(), |file| {
                file.remove_server(0);
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
        let holder = Holder::new(scratch("resume-after-sign-in"));
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
        let holder = Holder::new(scratch("resume-after-revoke"));
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
            move |file| file.set_server(0, &record)
        })
        .await
        .expect("seed record");

        let holder = Holder::new(path);
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
        let holder = Holder::new(scratch("sign-in-revokes"));
        holder.install(Upstream::stub(&server.base)).await;
        holder.install(Upstream::stub(&server.base)).await;
        assert_eq!(server.requests.load(std::sync::atomic::Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn a_revoke_with_nothing_held_leaves_the_session_file_untouched() {
        let server = answering(200).await;
        let path = scratch("revoke-nothing-held");
        let record = Upstream::stub(&server.base).session();
        crate::session::SessionFile::update_async(path.clone(), {
            let record = record.clone();
            move |file| file.set_server(0, &record)
        })
        .await
        .expect("seed record");

        let holder = Holder::new(path);
        holder.revoke().await.expect("revoke");
        assert!(holder.saved().await.is_some());
        assert!(holder.held().await.is_none());
        assert_eq!(server.requests.load(std::sync::atomic::Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn a_revoke_the_server_refuses_clears_the_session_file() {
        let server = answering(401).await;
        let holder = Holder::new(scratch("revoke-refused"));
        holder.install(Upstream::stub(&server.base)).await;
        holder.revoke().await.expect("revoke");
        assert!(holder.saved().await.is_none());
        assert!(holder.held().await.is_none());
    }

    #[tokio::test]
    async fn a_revoke_a_server_does_not_accept_keeps_the_session_file() {
        let server = answering(500).await;
        let holder = Holder::new(scratch("revoke-unreachable"));
        holder.install(Upstream::stub(&server.base)).await;
        assert!(matches!(
            holder.revoke().await,
            Err(Failure::ServerUnreachable { .. })
        ));
        assert!(holder.saved().await.is_some());
        assert!(holder.held().await.is_some());
    }
}
