//! An ordered list a reader moves one row at a time.

/// Which way a row moves in an ordered list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Toward {
    Earlier,
    Later,
}

/// `ordered` with `entry` one place `toward`.
// a list naming no `entry`, and a move off either end, is returned unchanged
// reference: library-options-sortable
pub fn moved<T: Clone + PartialEq>(ordered: &[T], entry: &T, toward: Toward) -> Vec<T> {
    let mut moved = ordered.to_vec();
    let Some(at) = moved.iter().position(|held| held == entry) else {
        return moved;
    };
    let to = match toward {
        Toward::Later => at + 1,
        Toward::Earlier => at.wrapping_sub(1),
    };
    if to < moved.len() {
        moved.swap(at, to);
    }
    moved
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ordered() -> Vec<&'static str> {
        vec!["one", "two", "three"]
    }

    #[test]
    fn a_move_at_either_end_changes_nothing() {
        assert_eq!(moved(&ordered(), &"one", Toward::Earlier), ordered());
        assert_eq!(moved(&ordered(), &"three", Toward::Later), ordered());
    }

    #[test]
    fn a_move_swaps_one_neighbour() {
        assert_eq!(
            moved(&ordered(), &"one", Toward::Later),
            vec!["two", "one", "three"]
        );
        assert_eq!(
            moved(&ordered(), &"three", Toward::Earlier),
            vec!["one", "three", "two"]
        );
    }

    #[test]
    fn a_list_naming_no_entry_is_returned_unchanged() {
        assert_eq!(moved(&ordered(), &"four", Toward::Later), ordered());
    }
}
