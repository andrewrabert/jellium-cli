//! What the appearance module renders, checked where the module lives.

use jellium_model::appearance::css;
use jellium_model::appearance::scheme::{Alpha, Color};

#[test]
fn the_boot_stylesheet_is_the_one_the_appearance_module_renders() {
    assert_eq!(
        include_str!("../../jellium-web/boot.css"),
        css::boot(),
        "jellium-web/boot.css has drifted from appearance::css::boot; \
         `just static-page` writes it"
    );
}

/// The alpha a css decimal denotes, and None where the text is not zero to one
/// written to at most three places.
fn opacity(written: &str) -> Option<Alpha> {
    let (whole, fraction) = written.split_once('.').unwrap_or((written, ""));
    if fraction.len() > 3 || !fraction.chars().all(|value| value.is_ascii_digit()) {
        return None;
    }
    let whole: u16 = whole.parse().ok()?;
    let fraction: u16 = format!("{fraction:0<3}").parse().ok()?;
    if whole > 1 || (whole == 1 && fraction > 0) {
        return None;
    }
    Some(Alpha::thousandths(whole * 1000 + fraction))
}

/// The color a rendered text denotes, and None where the text is not one of
/// the three forms css allows a color to be written in here.
fn read(written: &str) -> Option<Color> {
    if let Some(arguments) = written
        .strip_prefix("rgba(")
        .and_then(|rest| rest.strip_suffix(')'))
    {
        let fields: Vec<&str> = arguments.split(", ").collect();
        let [red, green, blue, alpha] = fields.as_slice() else {
            return None;
        };
        return Some(Color::rgba(
            red.parse().ok()?,
            green.parse().ok()?,
            blue.parse().ok()?,
            opacity(alpha)?,
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
                digit
                    .to_digit(16)
                    .map(|value| (value as u8) << 4 | value as u8)
            })
            .collect::<Option<Vec<u8>>>()?,
        6 => (0..3)
            .map(|channel| u8::from_str_radix(&digits[channel * 2..channel * 2 + 2], 16).ok())
            .collect::<Option<Vec<u8>>>()?,
        _ => return None,
    };
    let [red, green, blue] = channels.as_slice() else {
        return None;
    };
    Some(Color::rgb(*red, *green, *blue))
}

/// The alpha as a rendered color spells it, and None where the text is not an
/// `rgba(..)`.
fn spelling(written: &str) -> Option<&str> {
    written
        .strip_prefix("rgba(")
        .and_then(|rest| rest.strip_suffix(')'))
        .and_then(|arguments| arguments.rsplit_once(", "))
        .map(|(_, alpha)| alpha)
}

/// One channel whose two hex digits repeat and one whose do not, which is all
/// a neighbouring channel decides about the form a color is written in.
const KINDS: [u8; 2] = [0x00, 0x01];

/// Every `#` text the choice between the two forms turns on: each of the
/// three-digit forms, which is every color the short form is written for, and
/// each channel value standing in each of the three positions beside
/// neighbours of both kinds, which is every way the choice is made.
fn hashed() -> impl Iterator<Item = String> {
    let short = (0..16u32.pow(3)).map(|digits| format!("#{digits:03x}"));
    let long = (0..=u8::MAX).flat_map(|value| {
        KINDS.into_iter().flat_map(move |first| {
            KINDS.into_iter().flat_map(move |second| {
                [
                    [value, first, second],
                    [first, value, second],
                    [first, second, value],
                ]
            })
        })
    });
    short.chain(long.map(|[red, green, blue]| format!("#{red:02x}{green:02x}{blue:02x}")))
}

/// Every decimal css writes an alpha in, each beside the alpha it denotes: a
/// whole of zero or one to at most three places, in every spelling of that
/// grammar. The whole itself is not among them, an opaque color taking the `#`
/// form instead.
fn decimals() -> Vec<(String, Alpha)> {
    let mut found = Vec::new();
    for whole in 0..=1u16 {
        for places in 0..=3usize {
            for fraction in 0..10u32.pow(places as u32) {
                let text = match places {
                    0 => whole.to_string(),
                    _ => format!("{whole}.{fraction:0places$}"),
                };
                let Some(alpha) = opacity(&text) else {
                    continue;
                };
                if alpha == Alpha::OPAQUE {
                    continue;
                }
                found.push((text, alpha));
            }
        }
    }
    found
}

/// An opaque color is written in the shortest text that reads back as it.
#[test]
fn an_opaque_color_is_written_in_the_shortest_text_that_reads_back_as_it() {
    for text in hashed() {
        let color = read(&text).unwrap_or_else(|| panic!("{text} is a color css writes"));
        let written = color.css();
        assert_eq!(
            read(&written),
            Some(color),
            "{text} renders as {written}, which reads back as another color"
        );
        assert!(
            written.len() <= text.len(),
            "{text} renders as the longer {written}"
        );
    }
}

#[test]
fn every_alpha_is_written_in_the_shortest_decimal_that_reads_back_as_it() {
    for (text, alpha) in decimals() {
        let written = Color::rgba(0, 0, 0, alpha).css();
        let Some(spelled) = spelling(&written) else {
            panic!("{alpha:?} renders as {written}, which drops its alpha");
        };
        assert_eq!(
            opacity(spelled),
            Some(alpha),
            "{alpha:?} renders as {written}, whose alpha reads back as another"
        );
        assert!(
            spelled.len() <= text.len(),
            "{alpha:?} renders its alpha as the longer {spelled}, not as {text}"
        );
    }
}

/// Every channel value at every alpha reads back as itself; the three channels
/// are given different values at each step, so a render that swapped two of
/// them fails here.
#[test]
fn every_channel_at_every_alpha_reads_back_as_itself() {
    for value in 0..=u8::MAX {
        for count in 0..=1000u16 {
            let color = Color::rgba(
                value,
                value.wrapping_add(1),
                value.wrapping_add(2),
                Alpha::thousandths(count),
            );
            let written = color.css();
            assert_eq!(
                read(&written),
                Some(color),
                "{color:?} renders as {written}, which reads back as another color"
            );
        }
    }
}
