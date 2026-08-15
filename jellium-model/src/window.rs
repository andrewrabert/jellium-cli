/// Which windowed surface a scroll belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Id {
    Guide,
    Channels,
    Queue,
    Recordings,
    Schedule,
    Series,
    Activity,
    Log,
    Catalog,
    Users,
    Tasks,
    Devices,
    Plugins,
    /// The one browse surface on screen: a library grid, search results, a hub,
    /// a filtered list, a collection's contents, or either destination.
    Browse,
    /// A playlist's entries in playlist order.
    Entries,
}

/// The initials a name-sorted list offers, `#` first and then `A` to `Z`.
pub const LETTERS: [char; 27] = [
    '#', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
    'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
];

/// The value `nameStartsWithOrGreater` carries for `letter`, and `None` for
/// `#`, whose first item is the list's own first.
pub fn letter_bound(letter: char) -> Option<String> {
    (letter != '#').then(|| letter.to_ascii_uppercase().to_string())
}

/// Where one windowed surface is scrolled, how tall its rows are, and how tall
/// its viewport is.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Window {
    id: Id,
    row: f32,
    offset: f32,
    height: f32,
}

impl Window {
    /// Rows beyond the viewport built at each end, which is what keeps a
    /// scroll from showing an empty band.
    pub const MARGIN: usize = 4;

    /// A window over rows `row` pixels tall inside a page `height` pixels
    /// tall, scrolled to the top.
    pub fn new(id: Id, row: f32, height: f32) -> Window {
        Window {
            id,
            row: row.max(1.0),
            offset: 0.0,
            height: height.max(0.0),
        }
    }

    pub fn id(self) -> Id {
        self.id
    }

    pub fn row(self) -> f32 {
        self.row
    }

    /// Where the window is scrolled to.
    pub fn offset(self) -> f32 {
        self.offset
    }

    /// Takes an offset a caller remembered.
    pub fn moved(&mut self, offset: f32) {
        self.offset = offset.max(0.0);
    }

    /// Applies a scroll and the viewport height it reported.
    pub fn scrolled(&mut self, scrolled: Scrolled) {
        self.offset = scrolled.offset.max(0.0);
        self.height = scrolled.height.max(0.0);
    }

    /// Applies a page resize.
    pub fn resized(&mut self, height: f32) {
        self.height = height.max(0.0);
    }

    /// The rows of `count` the viewport shows, without the margin; it is what
    /// the image cache's wanted-set and a page step are computed from.
    pub fn shown(self, count: usize) -> std::ops::Range<usize> {
        if count == 0 {
            return 0..0;
        }
        let first = (self.offset / self.row).floor().max(0.0) as usize;
        let across = (self.height / self.row).ceil().max(1.0) as usize;
        let first = first.min(count);
        let last = first.saturating_add(across).min(count);
        first..last
    }

    /// The rows of `count` that are built: those the viewport shows, widened
    /// by `MARGIN` at each end and clamped to `0..count`.
    pub fn built(self, count: usize) -> std::ops::Range<usize> {
        let shown = self.shown(count);
        if shown.is_empty() {
            return shown;
        }
        let first = shown.start.saturating_sub(Self::MARGIN);
        let last = shown.end.saturating_add(Self::MARGIN).min(count);
        first..last
    }
}

/// A window over a grid of fixed-size cells laid out in rows.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Grid {
    window: Window,
    cell: f32,
    width: f32,
}

impl Grid {
    /// A grid of cells `cell` wide in rows `row` tall, inside a viewport
    /// `width` by `height`, scrolled to the top.
    pub fn new(id: Id, cell: f32, row: f32, width: f32, height: f32) -> Grid {
        Grid {
            window: Window::new(id, row, height),
            cell: cell.max(1.0),
            width: width.max(0.0),
        }
    }

    pub fn id(self) -> Id {
        self.window.id()
    }

    pub fn row(self) -> f32 {
        self.window.row()
    }

    /// Cells across, never fewer than one.
    pub fn columns(self) -> usize {
        ((self.width / self.cell).floor().max(1.0)) as usize
    }

    /// The rows `count` cells occupy.
    pub fn rows(self, count: usize) -> usize {
        count.div_ceil(self.columns())
    }

    pub fn scrolled(&mut self, scrolled: Scrolled) {
        self.window.scrolled(scrolled);
    }

    pub fn resized(&mut self, width: f32, height: f32) {
        self.width = width.max(0.0);
        self.window.resized(height);
    }

    /// The cells of `count` the viewport shows, without the margin.
    pub fn shown(self, count: usize) -> std::ops::Range<usize> {
        self.cells(self.window.shown(self.rows(count)), count)
    }

    /// The cells of `count` that are built: those shown, widened by
    /// `Window::MARGIN` rows at each end.
    pub fn built(self, count: usize) -> std::ops::Range<usize> {
        self.cells(self.window.built(self.rows(count)), count)
    }

    /// The cells the rows `rows` cover, clamped to `0..count`.
    fn cells(self, rows: std::ops::Range<usize>, count: usize) -> std::ops::Range<usize> {
        let columns = self.columns();
        let first = rows.start.saturating_mul(columns).min(count);
        let last = rows.end.saturating_mul(columns).min(count);
        first..last
    }

    /// Where the grid is scrolled to, which is what a sort change remembers.
    pub fn offset(self) -> f32 {
        self.window.offset()
    }

    /// Takes an offset a sort change remembered.
    pub fn moved(&mut self, offset: f32) {
        self.window.moved(offset);
    }

    /// The offset that puts the row holding cell `index` at the top.
    pub fn resting(self, index: usize) -> f32 {
        (index / self.columns()) as f32 * self.row()
    }
}

/// One scroll of a windowed surface, carrying the viewport it was measured in.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Scrolled {
    pub id: Id,
    pub offset: f32,
    pub height: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    const ROW_HEIGHT: f32 = 64.0;

    /// A window over `ROW_HEIGHT` rows in a viewport `rows` rows tall,
    /// scrolled `scrolled` rows down.
    fn window(rows: f32, scrolled: f32) -> Window {
        let mut window = Window::new(Id::Guide, ROW_HEIGHT, rows * ROW_HEIGHT);
        window.scrolled(Scrolled {
            id: Id::Guide,
            offset: scrolled * ROW_HEIGHT,
            height: rows * ROW_HEIGHT,
        });
        window
    }

    #[test]
    fn a_window_builds_only_the_rows_the_viewport_shows_plus_the_margin() {
        let built = window(10.0, 100.0).built(500);
        assert_eq!(built, 100 - Window::MARGIN..110 + Window::MARGIN);
        assert!(built.len() < 500);
    }

    #[test]
    fn a_window_at_the_top_builds_no_row_before_the_first() {
        assert_eq!(window(10.0, 0.0).built(500).start, 0);
        assert_eq!(window(10.0, 2.0).built(500).start, 0);
    }

    #[test]
    fn a_window_at_the_end_builds_no_row_past_the_last() {
        let built = window(10.0, 495.0).built(500);
        assert_eq!(built.end, 500);
        assert!(built.start < 500);
    }

    #[test]
    fn a_window_over_fewer_rows_than_the_viewport_builds_them_all() {
        assert_eq!(window(10.0, 0.0).built(3), 0..3);
        assert_eq!(window(10.0, 0.0).built(0), 0..0);
    }

    const CELL: f32 = 150.0;

    /// A grid five cells across in a viewport `rows` rows tall, scrolled
    /// `scrolled` rows down.
    fn grid(rows: f32, scrolled: f32) -> Grid {
        let mut grid = Grid::new(Id::Browse, CELL, ROW_HEIGHT, CELL * 5.0, rows * ROW_HEIGHT);
        grid.scrolled(Scrolled {
            id: Id::Browse,
            offset: scrolled * ROW_HEIGHT,
            height: rows * ROW_HEIGHT,
        });
        grid
    }

    #[test]
    fn a_grid_lays_its_cells_out_across_the_viewport() {
        let grid = grid(10.0, 0.0);
        assert_eq!(grid.columns(), 5);
        assert_eq!(grid.rows(0), 0);
        assert_eq!(grid.rows(1), 1);
        assert_eq!(grid.rows(5), 1);
        assert_eq!(grid.rows(6), 2);
    }

    #[test]
    fn a_grid_narrower_than_one_cell_still_lays_out_one_column() {
        let grid = Grid::new(Id::Browse, CELL, ROW_HEIGHT, 10.0, 640.0);
        assert_eq!(grid.columns(), 1);
    }

    #[test]
    fn a_grid_builds_only_the_cells_the_viewport_shows_plus_the_margin() {
        let grid = grid(10.0, 100.0);
        assert_eq!(grid.shown(5_000), 500..550);
        assert_eq!(
            grid.built(5_000),
            (100 - Window::MARGIN) * 5..(110 + Window::MARGIN) * 5
        );
    }

    #[test]
    fn a_grid_builds_no_cell_before_the_first_or_past_the_last() {
        assert_eq!(grid(10.0, 0.0).built(5_000).start, 0);
        assert_eq!(grid(10.0, 995.0).built(5_000).end, 5_000);
        assert_eq!(grid(10.0, 0.0).built(3), 0..3);
        assert_eq!(grid(10.0, 0.0).built(0), 0..0);
    }

    #[test]
    fn a_grid_rests_a_cell_at_the_top_of_its_own_row() {
        let grid = grid(10.0, 0.0);
        assert_eq!(grid.resting(0), 0.0);
        assert_eq!(grid.resting(4), 0.0);
        assert_eq!(grid.resting(5), ROW_HEIGHT);
        assert_eq!(grid.resting(12), 2.0 * ROW_HEIGHT);
    }

    #[test]
    fn a_grid_takes_an_offset_a_sort_change_remembered() {
        let mut grid = grid(10.0, 100.0);
        let offset = grid.offset();
        grid.moved(0.0);
        assert_eq!(grid.offset(), 0.0);
        grid.moved(offset);
        assert_eq!(grid.offset(), offset);
        assert_eq!(grid.shown(5_000), 500..550);
    }

    #[test]
    fn a_resize_relays_the_grid_across_the_new_width() {
        let mut grid = grid(10.0, 0.0);
        grid.resized(CELL * 3.0, 10.0 * ROW_HEIGHT);
        assert_eq!(grid.columns(), 3);
        assert_eq!(grid.rows(7), 3);
    }

    #[test]
    fn the_letter_jump_offers_a_hash_and_then_a_to_z() {
        assert_eq!(LETTERS.len(), 27);
        assert_eq!(LETTERS[0], '#');
        assert_eq!(LETTERS[26], 'Z');
        assert_eq!(letter_bound('#'), None);
        assert_eq!(letter_bound('M'), Some("M".to_owned()));
    }

    #[test]
    fn the_rows_shown_exclude_the_margin() {
        let window = window(10.0, 100.0);
        assert_eq!(window.shown(500), 100..110);
        assert_eq!(window.built(500), 96..114);
    }
}
