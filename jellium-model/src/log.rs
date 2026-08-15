//! The tail of a log file is indexed by line so a window builds only the lines
//! it shows.

/// The last bytes of a log file, indexed by line.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tail {
    text: String,
    lines: Vec<std::ops::Range<usize>>,
    size: u64,
}

impl Tail {
    /// Indexes `text` by line; `size` is the file's full length and `text` its
    /// last bytes.
    pub fn of(text: String, size: u64) -> Tail {
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

    pub fn size(&self) -> u64 {
        self.size
    }

    /// True when `text` is shorter than `size`, which is what names it a tail.
    pub fn truncated(&self) -> bool {
        (self.text.len() as u64) < self.size
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_body_is_indexed_by_line() {
        let tail = Tail::of("one\ntwo\nthree".to_owned(), 13);
        assert_eq!(tail.lines(), 3);
        assert_eq!(tail.line(0), "one");
        assert_eq!(tail.line(1), "two");
        assert_eq!(tail.line(2), "three");
        assert_eq!(tail.line(3), "");
        assert!(!tail.truncated());
    }

    #[test]
    fn a_carriage_return_is_not_part_of_the_line() {
        let tail = Tail::of("one\r\ntwo\r\n".to_owned(), 10);
        assert_eq!(tail.lines(), 2);
        assert_eq!(tail.line(0), "one");
        assert_eq!(tail.line(1), "two");
    }

    #[test]
    fn a_body_shorter_than_the_file_is_a_tail() {
        let tail = Tail::of("tail\n".to_owned(), 2 * 1024 * 1024);
        assert!(tail.truncated());
        assert_eq!(tail.size(), 2 * 1024 * 1024);
    }

    #[test]
    fn an_empty_body_holds_no_line() {
        let tail = Tail::of(String::new(), 0);
        assert_eq!(tail.lines(), 0);
        assert!(!tail.truncated());
    }
}
