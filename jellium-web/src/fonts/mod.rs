//! The reference's whole base coverage rides in the bundle at both weights;
//! its five CJK families are served from the page's own origin and fetched
//! when a glyph needs one.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashSet;

use iced::Subscription;
use iced::futures::Stream;
use iced::futures::channel::mpsc;

use crate::error::Answer;
use crate::style::typeface::Weight;
use crate::text::Text;

mod coverage;

/// A unicode scalar as every table in this tree writes one: base sixteen, no
/// prefix, which is how the reference's own `unicode-range` and its icon
/// metadata spell a codepoint.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Codepoint {
    scalar: u32,
}

impl Codepoint {
    pub fn of(character: char) -> Codepoint {
        Codepoint {
            scalar: character as u32,
        }
    }

    /// The codepoint just past this one, and this one at the top of the space.
    fn after(self) -> Codepoint {
        Codepoint {
            scalar: self.scalar.saturating_add(1),
        }
    }
}

impl std::str::FromStr for Codepoint {
    type Err = std::num::ParseIntError;

    fn from_str(text: &str) -> Result<Codepoint, std::num::ParseIntError> {
        Ok(Codepoint {
            scalar: u32::from_str_radix(text, 16)?,
        })
    }
}

/// One of the five families the reference ships outside its base coverage.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Family {
    HongKong,
    Japanese,
    Korean,
    Simplified,
    Traditional,
}

impl Family {
    /// The family name the reference's own `@font-face` rule writes.
    pub fn name(self) -> &'static str {
        match self {
            Family::HongKong => "Noto Sans HK",
            Family::Japanese => "Noto Sans JP",
            Family::Korean => "Noto Sans KR",
            Family::Simplified => "Noto Sans SC",
            Family::Traditional => "Noto Sans TC",
        }
    }
}

/// A face the origin serves: one row of the served table.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Served {
    row: u16,
}

impl Served {
    fn declared(self) -> &'static coverage::Row {
        &coverage::rows()[usize::from(self.row)]
    }

    pub fn family(self) -> Family {
        self.declared().family
    }

    /// The path under the page's own origin the face is served at.
    pub fn path(self) -> &'static str {
        self.declared().path
    }
}

/// What draws a codepoint at a weight.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cover {
    /// The bundle's own base faces draw it.
    Embedded,
    /// A face the origin serves draws it.
    Served(Served),
    /// No face the reference declares carries it.
    Unshipped,
}

impl Cover {
    pub fn of(codepoint: char, weight: Weight) -> Cover {
        let codepoint = Codepoint::of(codepoint);
        if holds(coverage::embedded(), codepoint) {
            return Cover::Embedded;
        }
        let served = coverage::rows()
            .iter()
            .position(|row| row.weight == weight && holds(&row.ranges, codepoint));
        match served.map(u16::try_from) {
            Some(Ok(row)) => Cover::Served(Served { row }),
            _ => Cover::Unshipped,
        }
    }
}

/// True when one of `ranges`, which ascend and do not touch, holds
/// `codepoint`.
fn holds(ranges: &[(Codepoint, Codepoint)], codepoint: Codepoint) -> bool {
    ranges
        .binary_search_by(
            |(start, end)| match (*start > codepoint, *end < codepoint) {
                (true, _) => Ordering::Greater,
                (_, true) => Ordering::Less,
                _ => Ordering::Equal,
            },
        )
        .is_ok()
}

thread_local! {
    /// Every face this client has already put the question to the origin for,
    /// which is what keeps a later miss from asking twice.
    static ASKED: RefCell<HashSet<Served>> = RefCell::new(HashSet::new());

    /// The one channel every want is written into, held for the life of the
    /// page so a want raised before the subscription starts is still
    /// delivered.
    static CHANNEL: (mpsc::UnboundedSender<Served>, RefCell<mpsc::UnboundedReceiver<Served>>) = {
        let (sender, receiver) = mpsc::unbounded();
        (sender, RefCell::new(receiver))
    };
}

/// The sixteen base faces, each unpacked from its woff2, and the Material
/// Icons face, which the reference already ships as sfnt. A face that does not
/// unpack is left out and its sentence raised.
pub fn embedded() -> Vec<Vec<u8>> {
    const PACKED: [&[u8]; 16] = [
        include_bytes!("../../fonts/noto-sans-latin-400-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-latin-700-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-latin-ext-400-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-latin-ext-700-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-cyrillic-400-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-cyrillic-700-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-cyrillic-ext-400-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-cyrillic-ext-700-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-greek-400-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-greek-700-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-greek-ext-400-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-greek-ext-700-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-vietnamese-400-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-vietnamese-700-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-devanagari-400-normal.woff2"),
        include_bytes!("../../fonts/noto-sans-devanagari-700-normal.woff2"),
    ];
    const ICONS: &[u8] = include_bytes!("../../fonts/MaterialIcons-Regular.ttf");

    PACKED
        .into_iter()
        .filter_map(|packed| crate::failure::unpacked(Text::FailureFontUnpacked, packed))
        .chain(std::iter::once(ICONS.to_vec()))
        .collect()
}

/// Records every codepoint of `content` the origin must serve a face for, once
/// per face.
pub fn observed(content: &str, weight: Weight) {
    for character in content.chars() {
        if let Cover::Served(face) = Cover::of(character, weight) {
            ask(face);
        }
    }
}

fn ask(face: Served) {
    ASKED.with(|asked| {
        if !asked.borrow_mut().insert(face) {
            return;
        }
        CHANNEL.with(|(sender, _)| {
            if sender.unbounded_send(face).is_err() {
                web_sys::console::error_1(&wasm_bindgen::JsValue::from_str(
                    "the font want channel is closed",
                ));
            }
        });
    });
}

/// The faces `observed` asked for.
pub fn wants() -> Subscription<Served> {
    Subscription::run(|| Wants)
}

struct Wants;

impl Stream for Wants {
    type Item = Served;

    fn poll_next(
        self: std::pin::Pin<&mut Self>,
        context: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Option<Served>> {
        CHANNEL.with(|(_, receiver)| {
            std::pin::Pin::new(&mut *receiver.borrow_mut()).poll_next(context)
        })
    }
}

/// The face's woff2 bytes, read from the page's own origin.
pub async fn fetched(face: Served) -> Answer<Vec<u8>> {
    Answer::of(async move {
        let response = reqwest::Client::new()
            .get(format!("{}{}", crate::page::origin(), face.path()))
            .send()
            .await?;
        if !response.status().is_success() {
            return Err(crate::error::classify(response).await.into());
        }
        Ok(response.bytes().await?.to_vec())
    })
    .await
}

/// Marks `face` asked and answered, whether it drew or failed, so no later
/// miss asks for it again and no second failure is raised for it. One door,
/// because what a caller needs is that the question is closed, and nothing
/// reads which way it closed.
pub fn settled(face: Served) {
    ASKED.with(|asked| {
        asked.borrow_mut().insert(face);
    });
}
