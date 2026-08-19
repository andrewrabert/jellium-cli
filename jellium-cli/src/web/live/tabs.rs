use std::sync::atomic::{AtomicU64, Ordering};

use jellium_protocol::{Event, Feed};
use tokio::sync::RwLock;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender, unbounded_channel};

/// One connected browser tab.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct TabId(u64);

/// One tab and what the local server knows about it.
struct Tab {
    id: TabId,
    events: UnboundedSender<Event>,
    /// The playback session this tab reported holding.
    play_session: Option<String>,
    /// The feeds screens open in it consume.
    feeds: std::collections::HashSet<Feed>,
}

/// The connected tabs, in the order they connected.
pub struct Tabs {
    held: RwLock<Vec<Tab>>,
    next: AtomicU64,
}

impl Tabs {
    pub fn new() -> Tabs {
        Tabs {
            held: RwLock::new(Vec::new()),
            next: AtomicU64::new(0),
        }
    }

    /// Adds a tab and returns its id, the queue its events arrive on, and how
    /// many tabs are connected now.
    pub async fn add(&self) -> (TabId, UnboundedReceiver<Event>, usize) {
        let id = TabId(self.next.fetch_add(1, Ordering::SeqCst));
        let (events, arriving) = unbounded_channel();
        let mut held = self.held.write().await;
        held.push(Tab {
            id,
            events,
            play_session: None,
            feeds: std::collections::HashSet::new(),
        });
        (id, arriving, held.len())
    }

    /// Drops a tab and returns how many are connected now.
    pub async fn remove(&self, tab: TabId) -> usize {
        let mut held = self.held.write().await;
        held.retain(|held| held.id != tab);
        held.len()
    }

    pub async fn broadcast(&self, event: &Event) {
        for held in self.held.read().await.iter() {
            let _ = held.events.send(event.clone());
        }
    }

    pub async fn send(&self, tab: TabId, event: Event) {
        if let Some(held) = self.held.read().await.iter().find(|held| held.id == tab) {
            let _ = held.events.send(event);
        }
    }

    /// The tab holding `play_session`, and the most recently added tab when
    /// that is `None` or no tab holds it.
    pub async fn destination(&self, play_session: Option<&str>) -> Option<TabId> {
        let held = self.held.read().await;
        let holding = play_session.and_then(|wanted| {
            held.iter()
                .find(|held| held.play_session.as_deref() == Some(wanted))
        });
        holding.or_else(|| held.last()).map(|held| held.id)
    }

    pub async fn holding(&self, play_session: &str) -> Option<TabId> {
        self.held
            .read()
            .await
            .iter()
            .find(|held| held.play_session.as_deref() == Some(play_session))
            .map(|held| held.id)
    }

    pub async fn playing(&self, tab: TabId, play_session: Option<String>) {
        if let Some(held) = self
            .held
            .write()
            .await
            .iter_mut()
            .find(|held| held.id == tab)
        {
            held.play_session = play_session;
        }
    }

    /// Records whether `tab` holds `feed` open, and answers whether any tab
    /// holds it now.
    pub async fn watch(&self, tab: TabId, feed: Feed, watching: bool) -> bool {
        let mut held = self.held.write().await;
        if let Some(held) = held.iter_mut().find(|held| held.id == tab) {
            if watching {
                held.feeds.insert(feed);
            } else {
                held.feeds.remove(&feed);
            }
        }
        held.iter().any(|held| held.feeds.contains(&feed))
    }

    /// True while some tab holds `feed` open.
    pub async fn watched(&self, feed: Feed) -> bool {
        self.held
            .read()
            .await
            .iter()
            .any(|held| held.feeds.contains(&feed))
    }

    /// Every connected tab, in the order they connected.
    pub async fn every(&self) -> Vec<TabId> {
        self.held.read().await.iter().map(|held| held.id).collect()
    }

    /// The tabs holding `feed` open.
    pub async fn watchers(&self, feed: Feed) -> Vec<TabId> {
        self.held
            .read()
            .await
            .iter()
            .filter(|held| held.feeds.contains(&feed))
            .map(|held| held.id)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use jellium_protocol::{Control, Marked};

    fn refresh() -> Event {
        Event::Marked { items: Vec::new() }
    }

    fn commanded() -> Event {
        Event::Control(Control::PlayPause)
    }

    #[tokio::test]
    async fn a_control_command_reaches_the_tab_holding_the_playback_session() {
        let tabs = Tabs::new();
        let (first, mut to_first, _) = tabs.add().await;
        let (second, mut to_second, _) = tabs.add().await;
        tabs.playing(first, Some("session".to_owned())).await;

        let destination = tabs.destination(Some("session")).await;
        assert_eq!(destination, Some(first));
        tabs.send(destination.expect("a destination"), commanded())
            .await;

        assert_eq!(to_first.try_recv(), Ok(commanded()));
        assert!(to_second.try_recv().is_err());
        let _ = second;
    }

    #[tokio::test]
    async fn a_control_command_with_no_playback_session_reaches_the_newest_tab() {
        let tabs = Tabs::new();
        let (_first, mut to_first, _) = tabs.add().await;
        let (second, mut to_second, _) = tabs.add().await;

        assert_eq!(tabs.destination(None).await, Some(second));
        tabs.send(second, commanded()).await;

        assert_eq!(to_second.try_recv(), Ok(commanded()));
        assert!(to_first.try_recv().is_err());
    }

    #[tokio::test]
    async fn a_refresh_reaches_every_tab() {
        let tabs = Tabs::new();
        let mut queues = Vec::new();
        for _ in 0..8 {
            let (_, arriving, _) = tabs.add().await;
            queues.push(arriving);
        }
        tabs.broadcast(&refresh()).await;
        for mut arriving in queues {
            assert_eq!(arriving.try_recv(), Ok(refresh()));
        }
    }

    #[tokio::test]
    async fn a_playback_session_no_tab_holds_falls_to_the_newest_tab() {
        let tabs = Tabs::new();
        let (_first, _, _) = tabs.add().await;
        let (second, _, _) = tabs.add().await;
        assert_eq!(tabs.destination(Some("absent")).await, Some(second));
        assert_eq!(tabs.holding("absent").await, None);
    }

    #[tokio::test]
    async fn a_removed_tab_is_no_longer_a_destination() {
        let tabs = Tabs::new();
        let (only, _, count) = tabs.add().await;
        assert_eq!(count, 1);
        assert_eq!(tabs.remove(only).await, 0);
        assert_eq!(tabs.destination(None).await, None);
    }

    #[tokio::test]
    async fn watching_lasts_exactly_as_long_as_some_tab_has_a_panel_open() {
        let tabs = Tabs::new();
        let (first, _, _) = tabs.add().await;
        let (second, _, _) = tabs.add().await;
        assert!(tabs.watch(first, Feed::Targets, true).await);
        assert!(tabs.watch(second, Feed::Targets, true).await);
        assert_eq!(tabs.watchers(Feed::Targets).await, vec![first, second]);
        assert!(tabs.watch(first, Feed::Targets, false).await);
        assert!(!tabs.watch(second, Feed::Targets, false).await);
        assert!(tabs.watchers(Feed::Targets).await.is_empty());
        assert!(!tabs.watched(Feed::Targets).await);
    }

    #[tokio::test]
    async fn a_refresh_carries_its_items_to_every_tab() {
        let tabs = Tabs::new();
        let (_, mut arriving, _) = tabs.add().await;
        let items = vec![Marked {
            item: uuid::Uuid::nil(),
            played: true,
            favorite: false,
            play_count: 1,
            position_ticks: 0,
        }];
        tabs.broadcast(&Event::Marked {
            items: items.clone(),
        })
        .await;
        assert_eq!(arriving.try_recv(), Ok(Event::Marked { items }));
    }
}
