//! What the appearance module renders, checked where the module lives.

use jellium_model::appearance::css;
use jellium_model::appearance::scheme::{self, Color};

#[test]
fn the_boot_stylesheet_is_the_one_the_appearance_module_renders() {
    assert_eq!(
        include_str!("../../jellium-web/boot.css"),
        css::boot(),
        "jellium-web/boot.css has drifted from appearance::css::boot; \
         `just boot-css` writes it"
    );
}

/// Every color the scheme holds, beside the name the module gives it.
const COLORS: &[(&str, Color)] = &[
    ("BACKGROUND", scheme::BACKGROUND),
    ("SURFACE", scheme::SURFACE),
    ("TEXT", scheme::TEXT),
    ("TEXT_SECONDARY", scheme::TEXT_SECONDARY),
    ("LABEL", scheme::LABEL),
    ("ACCENT", scheme::ACCENT),
    ("ACCENT_FOCUS", scheme::ACCENT_FOCUS),
    ("SECONDARY", scheme::SECONDARY),
    ("ERROR", scheme::ERROR),
    ("DELETE", scheme::DELETE),
    ("STAR", scheme::STAR),
    ("RAISED", scheme::RAISED),
    ("RAISED_FOCUS", scheme::RAISED_FOCUS),
    ("ON_ACCENT", scheme::ON_ACCENT),
    ("ON_RAISED", scheme::ON_RAISED),
    ("INPUT", scheme::INPUT),
    ("CARD_PADDER", scheme::CARD_PADDER),
    ("SHADOW", scheme::SHADOW),
    ("HEADER", scheme::HEADER),
    ("BACKDROP_SCRIM", scheme::BACKDROP_SCRIM),
    ("LIST_HOVER", scheme::LIST_HOVER),
    ("LIST_FOCUS", scheme::LIST_FOCUS),
    ("TOAST", scheme::TOAST),
    ("DIALOG_BACKDROP", scheme::DIALOG_BACKDROP),
    ("SCROLLBAR_THUMB", scheme::SCROLLBAR_THUMB),
    ("SCROLLBAR_TRACK", scheme::SCROLLBAR_TRACK),
    ("INDICATOR", scheme::INDICATOR),
    ("SCRIM", scheme::SCRIM),
    ("ON_OSD", scheme::ON_OSD),
    ("ON_OSD_HEADER", scheme::ON_OSD_HEADER),
];

/// The channels and alpha a rendered color denotes, and None where the text is
/// not one of the three forms css allows a color to be written in here.
fn read(written: &str) -> Option<([u8; 3], f32)> {
    if let Some(arguments) = written
        .strip_prefix("rgba(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        let fields: Vec<&str> = arguments.split(", ").collect();
        let [red, green, blue, alpha] = fields.as_slice() else {
            return None;
        };
        return Some((
            [red.parse().ok()?, green.parse().ok()?, blue.parse().ok()?],
            alpha.parse().ok()?,
        ));
    }
    let digits = written.strip_prefix('#')?;
    if !digits.chars().all(|digit| digit.is_ascii_hexdigit()) {
        return None;
    }
    let channels: Vec<u8> = match digits.len() {
        3 => digits
            .chars()
            .map(|digit| {
                let value = digit.to_digit(16).unwrap_or_default() as u8;
                value << 4 | value
            })
            .collect(),
        6 => (0..3)
            .map(|channel| u8::from_str_radix(&digits[channel * 2..channel * 2 + 2], 16))
            .collect::<Result<Vec<u8>, _>>()
            .ok()?,
        _ => return None,
    };
    Some(([channels[0], channels[1], channels[2]], 1.0))
}

/// Whether every channel's two hex digits repeat, which is the case `#rgb`
/// writes in half the room.
fn repeating(color: Color) -> bool {
    [color.red(), color.green(), color.blue()]
        .iter()
        .all(|channel| channel >> 4 == channel & 0xf)
}

#[test]
fn a_rendered_color_is_written_the_way_the_reference_writes_it() {
    let source = include_str!("../src/appearance/scheme.rs");
    for line in source.lines() {
        let Some(name) = line
            .strip_prefix("pub const ")
            .and_then(|rest| rest.split_once(": Color"))
            .map(|(name, _)| name)
        else {
            continue;
        };
        assert!(
            COLORS.iter().any(|(known, _)| *known == name),
            "scheme::{name} is a color this test does not check"
        );
    }

    for (name, color) in COLORS.iter().copied().chain(
        scheme::CARD_BACKGROUNDS
            .iter()
            .map(|background| ("CARD_BACKGROUNDS", *background)),
    ) {
        let written = color.css();
        let Some((channels, alpha)) = read(&written) else {
            panic!("scheme::{name} renders as {written}, which css does not read as a color");
        };
        assert_eq!(
            channels,
            [color.red(), color.green(), color.blue()],
            "scheme::{name} renders as {written}, which is another color"
        );
        assert!(
            (alpha - color.alpha().fraction()).abs() < f32::EPSILON,
            "scheme::{name} renders as {written}, which is another alpha"
        );
        if color.alpha() == scheme::Alpha::OPAQUE {
            let expected = if repeating(color) { 4 } else { 7 };
            assert_eq!(
                written.len(),
                expected,
                "scheme::{name} renders as {written}, and css writes it shorter"
            );
        } else {
            assert!(
                written.starts_with("rgba("),
                "scheme::{name} renders as {written}, which drops its alpha"
            );
            let token = written
                .trim_end_matches(')')
                .rsplit(", ")
                .next()
                .expect("the rendered color carries an alpha");
            assert!(
                token == "0" || !(token.ends_with('0') || token.ends_with('.')),
                "scheme::{name} renders as {written}, whose alpha carries a digit that does not"
            );
        }
    }
}
