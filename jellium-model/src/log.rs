//! The tail of a log file is indexed by line so a window builds only the lines
//! it shows.

/// A count of bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bytes(u64);

impl Bytes {
    pub const fn of(count: u64) -> Bytes {
        Bytes(count)
    }

    /// The count itself, for the one boundary that carries only a number.
    pub fn count(self) -> u64 {
        self.0
    }

    /// How many mebibytes it holds.
    pub fn mebibytes(self) -> f64 {
        self.0 as f64 / (1024.0 * 1024.0)
    }
}

/// The most of a log file the local server delivers, which is what names the
/// body a tail.
pub const TAIL_LIMIT: Bytes = Bytes::of(2 * 1024 * 1024);

/// The last bytes of a log file, indexed by line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tail {
    text: String,
    lines: Vec<std::ops::Range<usize>>,
    size: Bytes,
}

impl Tail {
    /// Indexes `text` by line; `size` is the file's full length and `text` its
    /// last bytes.
    pub fn of(text: String, size: Bytes) -> Tail {
        let mut lines = Vec::new();
        let mut start = 0;
        for (at, byte) in text.bytes().enumerate() {
            if byte != b'\n' {
                continue;
            }
            let end = if at > start && text.as_bytes()[at - 1] == b'\r' {
                at - 1
            } else {
                at
            };
            lines.push(start..end);
            start = at + 1;
        }
        if start < text.len() {
            lines.push(start..text.len());
        }
        Tail { text, lines, size }
    }

    pub fn lines(&self) -> usize {
        self.lines.len()
    }

    pub fn line(&self, index: usize) -> &str {
        match self.lines.get(index) {
            Some(range) => &self.text[range.clone()],
            None => "",
        }
    }

    pub fn size(&self) -> Bytes {
        self.size
    }

    /// True when `text` is shorter than `size`, which is what names it a tail.
    pub fn truncated(&self) -> bool {
        (self.text.len() as u64) < self.size.count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_body_is_indexed_by_line() {
        let tail = Tail::of("one\ntwo\nthree".to_owned(), Bytes::of(13));
        assert_eq!(tail.lines(), 3);
        assert_eq!(tail.line(0), "one");
        assert_eq!(tail.line(1), "two");
        assert_eq!(tail.line(2), "three");
        assert_eq!(tail.line(3), "");
        assert!(!tail.truncated());
    }

    #[test]
    fn a_carriage_return_is_not_part_of_the_line() {
        let tail = Tail::of("one\r\ntwo\r\n".to_owned(), Bytes::of(10));
        assert_eq!(tail.lines(), 2);
        assert_eq!(tail.line(0), "one");
        assert_eq!(tail.line(1), "two");
    }

    #[test]
    fn a_body_shorter_than_the_file_is_a_tail() {
        let tail = Tail::of("tail\n".to_owned(), TAIL_LIMIT);
        assert!(tail.truncated());
        assert_eq!(tail.size(), TAIL_LIMIT);
    }

    #[test]
    fn an_empty_body_holds_no_line() {
        let tail = Tail::of(String::new(), Bytes::of(0));
        assert_eq!(tail.lines(), 0);
        assert!(!tail.truncated());
    }
}
