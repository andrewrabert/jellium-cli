//! A windowed surface fetches a page at a time as the window moves over it.

/// Rows held by page, fetched as a window moves over them.
#[derive(Debug, Clone, PartialEq)]
pub struct Paged<T> {
    total: usize,
    /// One slot per row, filled as the page covering it arrives.
    rows: Vec<Option<T>>,
    /// The pages asked for and not yet answered, as row ranges.
    asked: Vec<std::ops::Range<usize>>,
}

impl<T> Paged<T> {
    /// How many rows one fetch carries.
    pub const PAGE: usize = 200;

    /// A surface of `total` rows with none held.
    pub fn new(total: usize) -> Paged<T> {
        Paged {
            total,
            rows: (0..total).map(|_| None).collect(),
            asked: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.total
    }

    pub fn is_empty(&self) -> bool {
        self.total == 0
    }

    /// The row at `index`, and `None` while its page is not held.
    pub fn row(&self, index: usize) -> Option<&T> {
        self.rows.get(index)?.as_ref()
    }

    /// The one page `built` needs that is neither held nor in flight, and
    /// `None` when everything `built` covers is held or asked for.
    /// The page is aligned to `PAGE`, so the same rows are always asked for
    /// under the same range.
    pub fn wanted(&self, built: std::ops::Range<usize>) -> Option<std::ops::Range<usize>> {
        let end = built.end.min(self.total);
        for index in built.start.min(end)..end {
            if self.rows[index].is_some() {
                continue;
            }
            let start = index - index % Self::PAGE;
            let page = start..(start + Self::PAGE).min(self.total);
            if self.asked.contains(&page) {
                continue;
            }
            return Some(page);
        }
        None
    }

    /// Records that `page` has been asked for.
    pub fn began(&mut self, page: std::ops::Range<usize>) {
        if !self.asked.contains(&page) {
            self.asked.push(page);
        }
    }

    /// Takes the rows `page` answered with.
    pub fn filled(&mut self, page: std::ops::Range<usize>, rows: Vec<T>) {
        self.asked.retain(|held| *held != page);
        for (offset, row) in rows.into_iter().enumerate() {
            let index = page.start + offset;
            if index >= self.total {
                break;
            }
            self.rows[index] = Some(row);
        }
    }

    /// Drops the rows `range` covers so a later window re-fetches them; every
    /// other row stands and the total is unchanged.
    pub fn forget(&mut self, range: std::ops::Range<usize>) {
        for index in range.start.min(self.total)..range.end.min(self.total) {
            self.rows[index] = None;
        }
    }

    /// Puts `rows` at the front and grows the total by their count, which is
    /// what a live activity entry does; the rows already held keep their
    /// contents and every page in flight is dropped, because each named a
    /// range that has since moved.
    pub fn prepend(&mut self, rows: Vec<T>) {
        self.total += rows.len();
        let mut held: Vec<Option<T>> = rows.into_iter().map(Some).collect();
        held.append(&mut self.rows);
        self.rows = held;
        self.asked.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_surface_holds_no_row() {
        let paged = Paged::<u32>::new(1000);
        assert_eq!(paged.len(), 1000);
        assert!(!paged.is_empty());
        assert_eq!(paged.row(0), None);
    }

    #[test]
    fn a_window_wants_the_aligned_page_covering_its_first_missing_row() {
        let paged = Paged::<u32>::new(1000);
        assert_eq!(paged.wanted(0..30), Some(0..200));
        assert_eq!(paged.wanted(250..300), Some(200..400));
    }

    #[test]
    fn a_page_in_flight_is_not_wanted_twice() {
        let mut paged = Paged::<u32>::new(1000);
        let page = paged.wanted(0..30).expect("a page");
        paged.began(page.clone());
        assert_eq!(paged.wanted(0..30), None);
        paged.filled(page, (0..200).collect());
        assert_eq!(paged.row(199), Some(&199));
        assert_eq!(paged.wanted(0..30), None);
    }

    #[test]
    fn a_last_page_stops_at_the_total() {
        let mut paged = Paged::<u32>::new(250);
        let page = paged.wanted(200..250).expect("a page");
        assert_eq!(page, 200..250);
        paged.began(page.clone());
        paged.filled(page, (0..50).collect());
        assert_eq!(paged.row(249), Some(&49));
    }

    #[test]
    fn a_prepend_grows_the_total_and_keeps_the_rows_held() {
        let mut paged = Paged::<u32>::new(10);
        paged.filled(0..10, (0..10).collect());
        paged.prepend(vec![100, 101]);
        assert_eq!(paged.len(), 12);
        assert_eq!(paged.row(0), Some(&100));
        assert_eq!(paged.row(2), Some(&0));
        assert_eq!(paged.row(11), Some(&9));
    }

    #[test]
    fn a_prepend_drops_every_page_in_flight() {
        let mut paged = Paged::<u32>::new(1000);
        let page = paged.wanted(0..30).expect("a page");
        paged.began(page);
        paged.prepend(vec![1]);
        assert_eq!(paged.wanted(0..30), Some(0..200));
    }

    #[test]
    fn forgetting_the_rows_shown_re_fetches_only_those() {
        let mut paged = Paged::<u32>::new(1000);
        paged.filled(0..200, (0..200).collect());
        paged.forget(10..20);
        assert_eq!(paged.row(9), Some(&9));
        assert_eq!(paged.row(10), None);
        assert_eq!(paged.row(20), Some(&20));
        assert_eq!(paged.len(), 1000);
        assert_eq!(paged.wanted(0..30), Some(0..200));
    }

    #[test]
    fn an_empty_surface_wants_nothing() {
        let paged = Paged::<u32>::new(0);
        assert!(paged.is_empty());
        assert_eq!(paged.wanted(0..30), None);
    }
}
