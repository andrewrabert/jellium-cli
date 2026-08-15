use jellium_protocol::Repeat;
use jellyfin_api::types::BaseItemDto;

/// The items queued for this playback, never persisted and never reported.
pub struct Queue {
    items: Vec<BaseItemDto>,
    order: Vec<usize>,
    position: usize,
    shuffle: bool,
    repeat: Repeat,
}

/// A cheap deterministic shuffle: no crate is pulled in for a queue order the
/// user re-rolls by pressing the control again.
fn shuffled(rest: &mut [usize]) {
    let mut seed = js_sys::Math::random().to_bits() | 1;
    let mut index = rest.len();
    while index > 1 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        index -= 1;
        let swap = (seed % (index as u64 + 1)) as usize;
        rest.swap(index, swap);
    }
}

impl Queue {
    /// `start` is the index in `items` the queue begins at; shuffling keeps it
    /// first and shuffles the rest.
    pub fn new(items: Vec<BaseItemDto>, start: usize, shuffle: bool) -> Queue {
        let mut queue = Queue {
            order: (0..items.len()).collect(),
            items,
            position: start,
            shuffle: false,
            repeat: Repeat::Off,
        };
        queue.position = queue
            .order
            .iter()
            .position(|index| *index == start)
            .unwrap_or(0);
        if shuffle {
            queue.set_shuffle(true);
        }
        queue
    }

    fn at(&self, position: usize) -> Option<&BaseItemDto> {
        self.items.get(*self.order.get(position)?)
    }

    pub fn current(&self) -> Option<&BaseItemDto> {
        self.at(self.position)
    }

    fn next_position(&self) -> Option<usize> {
        match self.repeat {
            Repeat::One => Some(self.position),
            Repeat::All if self.position + 1 >= self.order.len() => Some(0),
            _ => (self.position + 1 < self.order.len()).then_some(self.position + 1),
        }
    }

    /// What plays next, honouring the repeat mode.
    pub fn peek_next(&self) -> Option<&BaseItemDto> {
        self.at(self.next_position()?)
    }

    /// The items after the current one, in play order.
    pub fn upcoming(&self) -> impl Iterator<Item = (usize, &BaseItemDto)> {
        ((self.position + 1)..self.order.len()).filter_map(|position| {
            let item = self.at(position)?;
            Some((position, item))
        })
    }

    /// Advances, honouring the repeat mode; `None` when the queue is
    /// exhausted.
    pub fn advance(&mut self) -> Option<&BaseItemDto> {
        self.position = self.next_position()?;
        self.current()
    }

    pub fn back(&mut self) -> Option<&BaseItemDto> {
        self.position = self.position.checked_sub(1)?;
        self.current()
    }

    /// Drops the item at `position` in play order; the current item stays.
    pub fn remove(&mut self, position: usize) {
        if position == self.position || position >= self.order.len() {
            return;
        }
        self.order.remove(position);
        if position < self.position {
            self.position -= 1;
        }
    }

    /// Reshuffles the items not yet played; false restores the original order
    /// of the items still queued, and returns none that was removed.
    pub fn set_shuffle(&mut self, shuffle: bool) {
        self.shuffle = shuffle;
        if shuffle {
            let mut rest: Vec<usize> = self.order[(self.position + 1)..].to_vec();
            shuffled(&mut rest);
            self.order.truncate(self.position + 1);
            self.order.extend(rest);
        } else {
            self.order[(self.position + 1)..].sort_unstable();
        }
    }

    pub fn shuffled(&self) -> bool {
        self.shuffle
    }

    pub fn set_repeat(&mut self, repeat: Repeat) {
        self.repeat = repeat;
    }

    pub fn repeat(&self) -> Repeat {
        self.repeat
    }

    /// Queues `items` after the current item, in the order given.
    pub fn insert_next(&mut self, items: Vec<BaseItemDto>) {
        for (at, item) in (self.position + 1..).zip(items) {
            self.items.push(item);
            self.order.insert(at, self.items.len() - 1);
        }
    }

    /// Queues `items` at the end, in the order given.
    pub fn append(&mut self, items: Vec<BaseItemDto>) {
        for item in items {
            self.items.push(item);
            self.order.push(self.items.len() - 1);
        }
    }

    /// Every queued item, so a live refresh reaches the queue too.
    pub fn items_mut(&mut self) -> impl Iterator<Item = &mut BaseItemDto> {
        self.items.iter_mut()
    }
}
