//! The preference bag: every user preference `UserConfiguration` does not
//! model, held in one display-preferences record, read honour-or-default and
//! written back whole.

use jellium_protocol::{Bitrate, Quality, sync};
use jellyfin_api::types::DisplayPreferencesDto;
use serde::Serialize;

/// The display-preferences record Jellium Web owns.
pub const RECORD: &str = "usersettings";

/// The skip lengths the playback screen offers, in seconds.
pub const SKIPS: [i64; 6] = [5, 10, 15, 30, 60, 90];

/// The background opacities the subtitle screen offers, as percentages.
pub const OPACITIES: [i32; 5] = [0, 25, 50, 75, 100];

/// The extra time offsets the playback screen offers, in milliseconds.
pub const SYNC_OFFSETS: [i64; 9] = [-1000, -750, -500, -250, 0, 250, 500, 750, 1000];

/// The attempt limits the playback screen offers.
pub const SYNC_ATTEMPTS: [u32; 5] = [0, 1, 3, 5, 10];

const QUALITY: &str = "quality";
const SKIP_BACK: &str = "skipBack";
const SKIP_FORWARD: &str = "skipForward";
const SUBTITLE_SIZE: &str = "subtitleSize";
const SUBTITLE_COLOUR: &str = "subtitleColour";
const SUBTITLE_BACKGROUND: &str = "subtitleBackground";
const SUBTITLE_OPACITY: &str = "subtitleOpacity";
const SUBTITLE_SHADOW: &str = "subtitleShadow";
const CONTINUE_WATCHING_ROW: &str = "continueWatchingRow";
const NEXT_UP_ROW: &str = "nextUpRow";
const SYNC_EXTRA_OFFSET: &str = "syncExtraOffset";
const SYNC_METHOD: &str = "syncMethod";
const SYNC_RATE_ATTEMPTS: &str = "syncRateAttempts";
const SYNC_SEEK_ATTEMPTS: &str = "syncSeekAttempts";

/// The prefix a library's own sort is held under, followed by that library's
/// id.
const LIBRARY_SORT: &str = "librarySort:";

/// The key `library`'s sort is held under.
fn library_sort_key(library: uuid::Uuid) -> String {
    format!("{LIBRARY_SORT}{library}")
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleSize {
    Small,
    Medium,
    Large,
    Huge,
}

impl SubtitleSize {
    pub const ALL: [SubtitleSize; 4] = [
        SubtitleSize::Small,
        SubtitleSize::Medium,
        SubtitleSize::Large,
        SubtitleSize::Huge,
    ];

    /// The cue font size as a percentage of the element's own.
    pub fn percent(self) -> u16 {
        match self {
            SubtitleSize::Small => 75,
            SubtitleSize::Medium => 100,
            SubtitleSize::Large => 150,
            SubtitleSize::Huge => 200,
        }
    }

    fn key(self) -> &'static str {
        match self {
            SubtitleSize::Small => "small",
            SubtitleSize::Medium => "medium",
            SubtitleSize::Large => "large",
            SubtitleSize::Huge => "huge",
        }
    }

    fn parse(raw: &str) -> Option<SubtitleSize> {
        SubtitleSize::ALL.into_iter().find(|it| it.key() == raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleColour {
    White,
    Black,
    Red,
    Green,
    Blue,
    Yellow,
    Magenta,
    Cyan,
}

impl SubtitleColour {
    pub const ALL: [SubtitleColour; 8] = [
        SubtitleColour::White,
        SubtitleColour::Black,
        SubtitleColour::Red,
        SubtitleColour::Green,
        SubtitleColour::Blue,
        SubtitleColour::Yellow,
        SubtitleColour::Magenta,
        SubtitleColour::Cyan,
    ];

    /// The colour as `#rrggbb`.
    pub fn hex(self) -> &'static str {
        match self {
            SubtitleColour::White => "#ffffff",
            SubtitleColour::Black => "#000000",
            SubtitleColour::Red => "#ff0000",
            SubtitleColour::Green => "#00ff00",
            SubtitleColour::Blue => "#0000ff",
            SubtitleColour::Yellow => "#ffff00",
            SubtitleColour::Magenta => "#ff00ff",
            SubtitleColour::Cyan => "#00ffff",
        }
    }

    /// The colour's three channels, which is what an opacity is applied to.
    pub fn channels(self) -> (u8, u8, u8) {
        match self {
            SubtitleColour::White => (255, 255, 255),
            SubtitleColour::Black => (0, 0, 0),
            SubtitleColour::Red => (255, 0, 0),
            SubtitleColour::Green => (0, 255, 0),
            SubtitleColour::Blue => (0, 0, 255),
            SubtitleColour::Yellow => (255, 255, 0),
            SubtitleColour::Magenta => (255, 0, 255),
            SubtitleColour::Cyan => (0, 255, 255),
        }
    }

    fn key(self) -> &'static str {
        match self {
            SubtitleColour::White => "white",
            SubtitleColour::Black => "black",
            SubtitleColour::Red => "red",
            SubtitleColour::Green => "green",
            SubtitleColour::Blue => "blue",
            SubtitleColour::Yellow => "yellow",
            SubtitleColour::Magenta => "magenta",
            SubtitleColour::Cyan => "cyan",
        }
    }

    fn parse(raw: &str) -> Option<SubtitleColour> {
        SubtitleColour::ALL.into_iter().find(|it| it.key() == raw)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubtitleShadow {
    None,
    Drop,
    Outline,
}

impl SubtitleShadow {
    pub const ALL: [SubtitleShadow; 3] = [
        SubtitleShadow::None,
        SubtitleShadow::Drop,
        SubtitleShadow::Outline,
    ];

    /// The `text-shadow` value, empty for `None`.
    pub fn shadow(self) -> &'static str {
        match self {
            SubtitleShadow::None => "",
            SubtitleShadow::Drop => "2px 2px 4px rgba(0, 0, 0, 0.8)",
            SubtitleShadow::Outline => {
                "-1px -1px 0 #000, 1px -1px 0 #000, -1px 1px 0 #000, 1px 1px 0 #000"
            }
        }
    }

    fn key(self) -> &'static str {
        match self {
            SubtitleShadow::None => "none",
            SubtitleShadow::Drop => "drop",
            SubtitleShadow::Outline => "outline",
        }
    }

    fn parse(raw: &str) -> Option<SubtitleShadow> {
        SubtitleShadow::ALL.into_iter().find(|it| it.key() == raw)
    }
}

fn method_key(method: sync::SyncMethod) -> &'static str {
    match method {
        sync::SyncMethod::Auto => "auto",
        sync::SyncMethod::Rate => "rate",
        sync::SyncMethod::Seek => "seek",
    }
}

fn method_parse(raw: &str) -> Option<sync::SyncMethod> {
    match raw {
        "auto" => Some(sync::SyncMethod::Auto),
        "rate" => Some(sync::SyncMethod::Rate),
        "seek" => Some(sync::SyncMethod::Seek),
        _ => None,
    }
}

/// A bitrate ceiling as the record holds it: `auto`, or the ceiling in bits per
/// second.
pub fn quality_key(quality: Quality) -> String {
    match quality {
        Quality::Auto => "auto".to_owned(),
        Quality::Limit { bits_per_second } => bits_per_second.bits_per_second().to_string(),
    }
}

/// The ceiling `raw` names, and `None` for text that names none.
pub fn quality_parse(raw: &str) -> Option<Quality> {
    if raw == "auto" {
        return Some(Quality::Auto);
    }
    raw.parse::<i64>()
        .ok()
        .filter(|bits| *bits > 0)
        .map(|bits| Quality::Limit {
            bits_per_second: Bitrate::of(bits),
        })
}

/// Every user preference this client owns that `UserConfiguration` does not
/// model.
/// Each is held in the record under its own key: `quality`, `skipBack`,
/// `skipForward`, `subtitleSize`, `subtitleColour`,
/// `subtitleBackground`, `subtitleOpacity`, `subtitleShadow`,
/// `continueWatchingRow`, `nextUpRow`, `syncExtraOffset`, `syncMethod`,
/// `syncRateAttempts` and `syncSeekAttempts`.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Held {
    pub quality: Quality,
    pub skip_back_seconds: i64,
    pub skip_forward_seconds: i64,
    pub subtitle_size: SubtitleSize,
    pub subtitle_colour: SubtitleColour,
    pub subtitle_background: SubtitleColour,
    pub subtitle_opacity: i32,
    pub subtitle_shadow: SubtitleShadow,
    pub continue_watching: bool,
    pub next_up: bool,
    pub sync: sync::Tuning,
}

impl Default for Held {
    fn default() -> Held {
        Held {
            quality: Quality::Auto,
            skip_back_seconds: 10,
            skip_forward_seconds: 30,
            subtitle_size: SubtitleSize::Medium,
            subtitle_colour: SubtitleColour::White,
            subtitle_background: SubtitleColour::Black,
            subtitle_opacity: 0,
            subtitle_shadow: SubtitleShadow::Drop,
            continue_watching: true,
            next_up: true,
            sync: sync::Tuning::DEFAULT,
        }
    }
}

impl Held {
    /// How native text cues are drawn under these preferences.
    pub fn cues(&self) -> Cues {
        let (red, green, blue) = self.subtitle_background.channels();
        let alpha = f64::from(self.subtitle_opacity.clamp(0, 100)) / 100.0;
        Cues {
            size: self.subtitle_size.percent(),
            colour: self.subtitle_colour.hex(),
            background: format!("rgba({red}, {green}, {blue}, {alpha})"),
            shadow: self.subtitle_shadow.shadow(),
        }
    }
}

/// The cue style one set of subtitle preferences renders as, in the values the
/// glue installs.
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Cues {
    /// The font size as a percentage of the element's own.
    pub size: u16,
    /// `#rrggbb`.
    pub colour: &'static str,
    /// `rgba(r, g, b, a)`.
    pub background: String,
    /// The `text-shadow` value, empty for no shadow.
    pub shadow: &'static str,
}

/// The preference bag: the record as the server answered it, and the edits made
/// against it.
#[derive(Debug, Clone)]
pub struct Bag {
    read: DisplayPreferencesDto,
    edited: Vec<(String, String)>,
}

// DisplayPreferencesDto carries no PartialEq, so two bags are the same when the
// records they would write and the edits they hold are the same.
impl PartialEq for Bag {
    fn eq(&self, other: &Bag) -> bool {
        self.edited == other.edited
            && serde_json::to_value(&self.read).ok() == serde_json::to_value(&other.read).ok()
    }
}

/// Every key one set of preferences renders as, which is what an edit compares
/// against and what a write carries.
fn rendered(held: Held) -> Vec<(&'static str, String)> {
    vec![
        (QUALITY, quality_key(held.quality)),
        (SKIP_BACK, held.skip_back_seconds.to_string()),
        (SKIP_FORWARD, held.skip_forward_seconds.to_string()),
        (SUBTITLE_SIZE, held.subtitle_size.key().to_owned()),
        (SUBTITLE_COLOUR, held.subtitle_colour.key().to_owned()),
        (
            SUBTITLE_BACKGROUND,
            held.subtitle_background.key().to_owned(),
        ),
        (SUBTITLE_OPACITY, held.subtitle_opacity.to_string()),
        (SUBTITLE_SHADOW, held.subtitle_shadow.key().to_owned()),
        (CONTINUE_WATCHING_ROW, held.continue_watching.to_string()),
        (NEXT_UP_ROW, held.next_up.to_string()),
        (SYNC_EXTRA_OFFSET, held.sync.extra_offset_ms.to_string()),
        (SYNC_METHOD, method_key(held.sync.method).to_owned()),
        (SYNC_RATE_ATTEMPTS, held.sync.rate_attempts.to_string()),
        (SYNC_SEEK_ATTEMPTS, held.sync.seek_attempts.to_string()),
    ]
}

fn chosen<T: Copy + PartialEq>(value: T, offered: &[T], fallback: T) -> T {
    if offered.contains(&value) {
        value
    } else {
        fallback
    }
}

impl Bag {
    /// Holds `read`, the record exactly as the server answered it.
    pub fn of(read: DisplayPreferencesDto) -> Bag {
        Bag {
            read,
            edited: Vec::new(),
        }
    }

    /// The bag of a record the server does not hold, which reads as every
    /// default and writes as a record carrying only the keys a control names.
    pub fn missing() -> Bag {
        Bag::of(DisplayPreferencesDto {
            id: Some(RECORD.to_owned()),
            ..DisplayPreferencesDto::default()
        })
    }

    fn raw(&self, key: &str) -> Option<&str> {
        if let Some((_, value)) = self.edited.iter().find(|(held, _)| held == key) {
            return Some(value.as_str());
        }
        self.read.custom_prefs.get(key)?.as_deref()
    }

    fn set(&mut self, key: &str, value: String) {
        match self.edited.iter_mut().find(|(held, _)| held == key) {
            Some(entry) => entry.1 = value,
            None => self.edited.push((key.to_owned(), value)),
        }
    }

    /// What the bag holds now: the record with every edit applied, each key
    /// whose value does not parse read as that key's default and every other
    /// key unaffected.
    pub fn held(&self) -> Held {
        let fallback = Held::default();
        Held {
            quality: self
                .raw(QUALITY)
                .and_then(quality_parse)
                .unwrap_or(fallback.quality),
            skip_back_seconds: self
                .raw(SKIP_BACK)
                .and_then(|raw| raw.parse().ok())
                .map(|held| chosen(held, &SKIPS, fallback.skip_back_seconds))
                .unwrap_or(fallback.skip_back_seconds),
            skip_forward_seconds: self
                .raw(SKIP_FORWARD)
                .and_then(|raw| raw.parse().ok())
                .map(|held| chosen(held, &SKIPS, fallback.skip_forward_seconds))
                .unwrap_or(fallback.skip_forward_seconds),
            subtitle_size: self
                .raw(SUBTITLE_SIZE)
                .and_then(SubtitleSize::parse)
                .unwrap_or(fallback.subtitle_size),
            subtitle_colour: self
                .raw(SUBTITLE_COLOUR)
                .and_then(SubtitleColour::parse)
                .unwrap_or(fallback.subtitle_colour),
            subtitle_background: self
                .raw(SUBTITLE_BACKGROUND)
                .and_then(SubtitleColour::parse)
                .unwrap_or(fallback.subtitle_background),
            subtitle_opacity: self
                .raw(SUBTITLE_OPACITY)
                .and_then(|raw| raw.parse().ok())
                .map(|held| chosen(held, &OPACITIES, fallback.subtitle_opacity))
                .unwrap_or(fallback.subtitle_opacity),
            subtitle_shadow: self
                .raw(SUBTITLE_SHADOW)
                .and_then(SubtitleShadow::parse)
                .unwrap_or(fallback.subtitle_shadow),
            continue_watching: self
                .raw(CONTINUE_WATCHING_ROW)
                .and_then(|raw| raw.parse().ok())
                .unwrap_or(fallback.continue_watching),
            next_up: self
                .raw(NEXT_UP_ROW)
                .and_then(|raw| raw.parse().ok())
                .unwrap_or(fallback.next_up),
            sync: sync::Tuning {
                extra_offset_ms: self
                    .raw(SYNC_EXTRA_OFFSET)
                    .and_then(|raw| raw.parse().ok())
                    .map(|held| chosen(held, &SYNC_OFFSETS, fallback.sync.extra_offset_ms))
                    .unwrap_or(fallback.sync.extra_offset_ms),
                method: self
                    .raw(SYNC_METHOD)
                    .and_then(method_parse)
                    .unwrap_or(fallback.sync.method),
                rate_attempts: self
                    .raw(SYNC_RATE_ATTEMPTS)
                    .and_then(|raw| raw.parse().ok())
                    .map(|held| chosen(held, &SYNC_ATTEMPTS, fallback.sync.rate_attempts))
                    .unwrap_or(fallback.sync.rate_attempts),
                seek_attempts: self
                    .raw(SYNC_SEEK_ATTEMPTS)
                    .and_then(|raw| raw.parse().ok())
                    .map(|held| chosen(held, &SYNC_ATTEMPTS, fallback.sync.seek_attempts))
                    .unwrap_or(fallback.sync.seek_attempts),
            },
        }
    }

    /// Records an edit for every key `held` renders differently from what the
    /// bag holds now; no other key changes.
    pub fn edit(&mut self, held: Held) {
        for (key, value) in rendered(held) {
            if rendered(self.held()).contains(&(key, value.clone())) {
                continue;
            }
            self.set(key, value);
        }
    }

    /// True when the record the server answered holds a value under `quality`;
    /// an edit no save has taken is not that record.
    fn record_holds_quality(&self) -> bool {
        self.read
            .custom_prefs
            .get(QUALITY)
            .is_some_and(Option::is_some)
    }

    /// Takes `quality` as a value the server's own record now holds, which is
    /// what a migration write the server answered means; every edit and every
    /// other key stand.
    pub fn carried(&mut self, quality: Quality) {
        self.read
            .custom_prefs
            .insert(QUALITY.to_owned(), Some(quality_key(quality)));
    }

    /// The sort held for `library`, and `None` when the bag holds none.
    pub fn library_sort(&self, library: uuid::Uuid) -> Option<crate::sort::Sort> {
        crate::sort::Sort::parse(self.raw(&library_sort_key(library))?)
    }

    /// Records an edit setting `library`'s sort; every other library's stands.
    pub fn set_library_sort(&mut self, library: uuid::Uuid, sort: crate::sort::Sort) {
        self.set(&library_sort_key(library), sort.key().to_owned());
    }

    pub fn dirty(&self) -> bool {
        !self.edited.is_empty()
    }

    /// The whole record as it is to be written: what was read with every edit
    /// applied, so every custom preference no key names is carried through
    /// unchanged.
    pub fn written(&self) -> DisplayPreferencesDto {
        let mut written = self.read.clone();
        for (key, value) in &self.edited {
            written
                .custom_prefs
                .insert(key.clone(), Some(value.clone()));
        }
        written
    }

    /// Takes the record as it stands after a save, clearing the edits.
    pub fn saved(&mut self, read: DisplayPreferencesDto) {
        self.read = read;
        self.edited.clear();
    }

    pub fn discard(&mut self) {
        self.edited.clear();
    }
}

/// What the first load of this version does with a bitrate ceiling an earlier
/// version parked in the browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Migration {
    /// Nothing is written and the browser's entry is left as it is.
    Skipped,
    /// The ceiling is written to the record; what becomes of the browser's
    /// entry is `Parked::of` against the record that write leaves behind.
    Carried { quality: Quality },
}

/// What becomes of a bitrate ceiling an earlier version parked in the browser.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Parked {
    /// The ceiling stays in the browser, so a later load carries it again.
    Kept,
    /// The ceiling leaves the browser, because the record holds one.
    Dropped,
}

impl Parked {
    /// `Dropped` when the record the server answered holds a value under
    /// `quality` and the instance is not read-only; `Kept` otherwise.
    /// A write still in flight, a write the server refused, and every
    /// read-only load each read as `Kept`, because none of them puts a ceiling
    /// in the record the server answered.
    pub fn of(bag: &Bag, read_only: bool) -> Parked {
        if !read_only && bag.record_holds_quality() {
            Parked::Dropped
        } else {
            Parked::Kept
        }
    }
}

/// `Carried` when `parked` holds a ceiling, the record the server answered holds
/// none, and the instance is not read-only; `Skipped` otherwise, so a second
/// load and a second tab both write the same value and neither errs.
pub fn migration(parked: Option<Quality>, bag: &Bag, read_only: bool) -> Migration {
    match parked {
        Some(quality) if !read_only && Parked::of(bag, read_only) == Parked::Kept => {
            Migration::Carried { quality }
        }
        _ => Migration::Skipped,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn record(pairs: &[(&str, &str)]) -> DisplayPreferencesDto {
        DisplayPreferencesDto {
            id: Some(RECORD.to_owned()),
            custom_prefs: pairs
                .iter()
                .map(|(key, value)| ((*key).to_owned(), Some((*value).to_owned())))
                .collect(),
            ..DisplayPreferencesDto::default()
        }
    }

    #[test]
    fn a_record_the_server_does_not_hold_reads_as_every_default() {
        assert_eq!(Bag::missing().held(), Held::default());
    }

    #[test]
    fn a_key_that_does_not_parse_reads_as_its_default_and_no_other_key_moves() {
        let bag = Bag::of(record(&[
            ("subtitleOpacity", "seventeen"),
            ("skipBack", "15"),
        ]));
        let held = bag.held();
        assert_eq!(held.subtitle_opacity, Held::default().subtitle_opacity);
        assert_eq!(held.skip_back_seconds, 15);
    }

    #[test]
    fn a_value_outside_what_a_control_offers_reads_as_its_default() {
        let bag = Bag::of(record(&[("skipBack", "37"), ("subtitleColour", "puce")]));
        let held = bag.held();
        assert_eq!(held.skip_back_seconds, Held::default().skip_back_seconds);
        assert_eq!(held.subtitle_colour, Held::default().subtitle_colour);
    }

    #[test]
    fn a_custom_preference_no_key_names_is_carried_through_a_write() {
        let mut bag = Bag::of(record(&[("theirs", "kept"), ("skipBack", "10")]));
        bag.edit(Held {
            skip_back_seconds: 5,
            ..Held::default()
        });
        let written = bag.written();
        assert_eq!(written.custom_prefs["theirs"].as_deref(), Some("kept"));
        assert_eq!(written.custom_prefs["skipBack"].as_deref(), Some("5"));
    }

    #[test]
    fn an_edit_that_changes_nothing_leaves_the_bag_clean() {
        let mut bag = Bag::of(record(&[("skipBack", "5")]));
        let held = bag.held();
        bag.edit(held);
        assert!(!bag.dirty());
        bag.edit(Held {
            skip_back_seconds: 30,
            ..held
        });
        assert!(bag.dirty());
        assert_eq!(bag.held().skip_back_seconds, 30);
    }

    #[test]
    fn a_save_takes_the_written_record_as_read() {
        let mut bag = Bag::missing();
        bag.edit(Held {
            skip_back_seconds: 5,
            ..Held::default()
        });
        let written = bag.written();
        bag.saved(written);
        assert!(!bag.dirty());
        assert_eq!(bag.held().skip_back_seconds, 5);
    }

    #[test]
    fn a_discard_leaves_what_the_server_answered() {
        let mut bag = Bag::of(record(&[("skipBack", "60")]));
        bag.edit(Held {
            skip_back_seconds: 5,
            ..Held::default()
        });
        bag.discard();
        assert!(!bag.dirty());
        assert_eq!(bag.held().skip_back_seconds, 60);
    }

    #[test]
    fn the_migration_runs_once_and_never_under_read_only() {
        let quality = Quality::Limit {
            bits_per_second: Bitrate::of(4_000_000),
        };
        let missing = Bag::missing();
        assert_eq!(
            migration(Some(quality), &missing, false),
            Migration::Carried { quality }
        );
        assert_eq!(migration(Some(quality), &missing, true), Migration::Skipped);
        assert_eq!(migration(None, &missing, false), Migration::Skipped);
        let holding = Bag::of(record(&[("quality", "auto")]));
        assert_eq!(
            migration(Some(quality), &holding, false),
            Migration::Skipped
        );
    }

    #[test]
    fn the_cue_style_carries_the_chosen_size_colour_background_and_shadow() {
        let cues = Held {
            subtitle_size: SubtitleSize::Large,
            subtitle_colour: SubtitleColour::Yellow,
            subtitle_background: SubtitleColour::Black,
            subtitle_opacity: 50,
            subtitle_shadow: SubtitleShadow::None,
            ..Held::default()
        }
        .cues();
        assert_eq!(cues.size, 150);
        assert_eq!(cues.colour, "#ffff00");
        assert_eq!(cues.background, "rgba(0, 0, 0, 0.5)");
        assert_eq!(cues.shadow, "");
    }

    #[test]
    fn a_bitrate_ceiling_reaches_the_record_on_a_save_rather_than_on_an_edit() {
        let quality = Quality::Limit {
            bits_per_second: Bitrate::of(4_000_000),
        };
        let mut bag = Bag::missing();
        bag.edit(Held {
            quality,
            ..Held::default()
        });
        assert_eq!(bag.held().quality, quality);
        assert_eq!(Parked::of(&bag, false), Parked::Kept);
        let written = bag.written();
        bag.saved(written);
        assert_eq!(Parked::of(&bag, false), Parked::Dropped);
    }

    #[test]
    fn a_parked_ceiling_stays_while_the_record_holds_no_ceiling() {
        assert_eq!(Parked::of(&Bag::missing(), false), Parked::Kept);
    }

    #[test]
    fn a_parked_ceiling_stays_under_read_only_whatever_the_record_holds() {
        assert_eq!(Parked::of(&Bag::missing(), true), Parked::Kept);
        let holding = Bag::of(record(&[("quality", "auto")]));
        assert_eq!(Parked::of(&holding, true), Parked::Kept);
    }

    #[test]
    fn a_quality_edit_no_save_has_taken_leaves_the_parked_ceiling_standing() {
        let mut bag = Bag::missing();
        bag.edit(Held {
            quality: Quality::Limit {
                bits_per_second: Bitrate::of(4_000_000),
            },
            ..Held::default()
        });
        assert_eq!(Parked::of(&bag, false), Parked::Kept);
    }

    #[test]
    fn a_carried_quality_drops_the_parked_ceiling_and_moves_no_edit() {
        let quality = Quality::Limit {
            bits_per_second: Bitrate::of(4_000_000),
        };
        let mut bag = Bag::of(record(&[("skipBack", "60")]));
        bag.edit(Held {
            skip_back_seconds: 5,
            ..Held::default()
        });
        bag.carried(quality);
        assert_eq!(Parked::of(&bag, false), Parked::Dropped);
        assert_eq!(bag.held().quality, quality);
        assert!(bag.dirty());
        assert_eq!(bag.held().skip_back_seconds, 5);
    }

    #[test]
    fn a_library_sort_is_held_per_library_and_leaves_every_other_standing() {
        let one = uuid::Uuid::from_u128(1);
        let two = uuid::Uuid::from_u128(2);
        let mut bag = Bag::missing();
        assert_eq!(bag.library_sort(one), None);
        bag.set_library_sort(one, crate::sort::Sort::DateAdded);
        assert_eq!(bag.library_sort(one), Some(crate::sort::Sort::DateAdded));
        assert_eq!(bag.library_sort(two), None);
        bag.set_library_sort(two, crate::sort::Sort::Random);
        assert_eq!(bag.library_sort(one), Some(crate::sort::Sort::DateAdded));
        assert_eq!(bag.library_sort(two), Some(crate::sort::Sort::Random));
    }

    #[test]
    fn a_library_sort_that_does_not_parse_reads_as_none() {
        let library = uuid::Uuid::from_u128(1);
        let bag = Bag::of(record(&[(&format!("librarySort:{library}"), "sideways")]));
        assert_eq!(bag.library_sort(library), None);
    }

    #[test]
    fn a_record_that_already_held_a_ceiling_drops_the_parked_one() {
        let holding = Bag::of(record(&[("quality", "auto")]));
        assert_eq!(Parked::of(&holding, false), Parked::Dropped);
    }
}
